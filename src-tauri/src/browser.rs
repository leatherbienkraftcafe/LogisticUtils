use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::Page;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;

use crate::parser::ParsedRow;

/// アプリ専用の Chrome インスタンスとの接続を保持するセッション。
/// `_handler_task` は CDP のイベントループを回し続けるために保持するだけで、
/// ドロップすると接続が切れるため明示的にフィールドとして持つ。
pub struct BrowserSession {
    // Drop すると起動した Chrome プロセスが終了するため、読み出さなくても保持し続ける必要がある。
    #[allow(dead_code)]
    browser: Browser,
    page: Page,
    _handler_task: tokio::task::JoinHandle<()>,
}

pub type BrowserState = Mutex<Option<BrowserSession>>;

pub fn new_state() -> BrowserState {
    Mutex::new(None)
}

/// キー(パーサーが抽出した項目名) → 対象画面の CSS セレクタ、のマッピング。
/// ClickPost 等の実際のフィールド構成は未調査のため、ユーザーが設定ファイルを編集して使う前提。
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FieldMapping(pub HashMap<String, String>);

fn default_field_mapping() -> FieldMapping {
    FieldMapping(HashMap::from([
        ("氏名".to_string(), "#example-name-input-not-verified".to_string()),
        ("郵便番号".to_string(), "#example-zip-input-not-verified".to_string()),
        ("住所".to_string(), "#example-address-input-not-verified".to_string()),
        ("電話番号".to_string(), "#example-tel-input-not-verified".to_string()),
    ]))
}

pub fn field_mapping_path(config_dir: &Path) -> PathBuf {
    config_dir.join("clickpost-field-map.json")
}

/// 設定ファイルが存在しなければ、ユーザーが後で編集できるようデフォルトのひな形を書き出す。
pub fn load_or_init_field_mapping(config_dir: &Path) -> Result<FieldMapping, String> {
    let path = field_mapping_path(config_dir);
    if !path.exists() {
        fs::create_dir_all(config_dir).map_err(|e| e.to_string())?;
        let default = default_field_mapping();
        let contents = serde_json::to_string_pretty(&default).map_err(|e| e.to_string())?;
        fs::write(&path, contents).map_err(|e| e.to_string())?;
        return Ok(default);
    }
    let contents = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&contents).map_err(|e| e.to_string())
}

/// アプリ専用プロファイルで Chrome/Chromium を起動し、指定 URL を開く。
/// 既にセッションがあれば使い回し、新しいタブで URL を開く。
/// ログイン認証はユーザーがこの起動されたウィンドウ内で手動で行う想定。
pub async fn open(
    state: &BrowserState,
    profile_dir: &Path,
    url: &str,
) -> Result<(), String> {
    let mut guard = state.lock().await;

    if guard.is_none() {
        fs::create_dir_all(profile_dir).map_err(|e| e.to_string())?;

        let config = BrowserConfig::builder()
            .with_head()
            .user_data_dir(profile_dir)
            .build()
            .map_err(|e| e.to_string())?;

        let (browser, mut handler) = Browser::launch(config)
            .await
            .map_err(|e| format!("Chrome の起動に失敗しました: {e}"))?;

        let handler_task = tokio::spawn(async move {
            while let Some(event) = handler.next().await {
                if event.is_err() {
                    break;
                }
            }
        });

        let page = browser
            .new_page(url)
            .await
            .map_err(|e| format!("ページを開けませんでした: {e}"))?;

        *guard = Some(BrowserSession {
            browser,
            page,
            _handler_task: handler_task,
        });
        return Ok(());
    }

    // 既存セッションがあれば、そのタブを対象 URL に遷移させる。
    let session = guard.as_ref().unwrap();
    session
        .page
        .goto(url)
        .await
        .map_err(|e| format!("ページ遷移に失敗しました: {e}"))?;
    Ok(())
}

/// 現在開いているページに対して、フィールドマッピングに従って値を注入する。
/// マッピングに存在しないキーはスキップする(対象画面に該当欄が無いか、未設定のため)。
pub async fn inject(
    state: &BrowserState,
    mapping: &FieldMapping,
    rows: &[ParsedRow],
) -> Result<Vec<String>, String> {
    let guard = state.lock().await;
    let session = guard
        .as_ref()
        .ok_or_else(|| "ブラウザがまだ開かれていません。先に「ClickPostを開く」を実行してください。".to_string())?;

    let mut applied = Vec::new();
    let mut missing = Vec::new();

    for row in rows {
        let Some(selector) = mapping.0.get(&row.key) else {
            missing.push(row.key.clone());
            continue;
        };

        let script = format!(
            r#"(() => {{
  const el = document.querySelector({selector});
  if (!el) return false;
  el.value = {value};
  el.dispatchEvent(new Event('input', {{ bubbles: true }}));
  el.dispatchEvent(new Event('change', {{ bubbles: true }}));
  return true;
}})()"#,
            selector = json!(selector),
            value = json!(row.value),
        );

        let found: bool = session
            .page
            .evaluate(script)
            .await
            .map_err(|e| format!("'{}' への注入でエラー: {e}", row.key))?
            .into_value()
            .map_err(|e| e.to_string())?;

        if found {
            applied.push(row.key.clone());
        } else {
            missing.push(format!("{}(セレクタに一致する要素なし)", row.key));
        }
    }

    if !missing.is_empty() {
        applied.push(format!("未反映: {}", missing.join(", ")));
    }

    Ok(applied)
}
