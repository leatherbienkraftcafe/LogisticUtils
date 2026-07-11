# LogisticUtils

Rust + Tauri を使ったデスクトップアプリの開発リポジトリです。
フロントエンドは `Svelte + Vite`、バックエンドは `Tauri/Rust` で構成されています。

## 現在の成果

- `frontend/` に Svelte + Vite の UI プロジェクトを作成
- `src-tauri/` に Tauri Rust バックエンドを構築
- `tauri.conf.json` は Tauri v2 の設定に対応
- `cargo check` は Linux コンテナ環境で実行済み
- Xvfb を使ってコンテナ内で `cargo tauri dev` を起動できる状態

## 目的

- Amazon / Yahoo / Excel の配送先データを抽出するパーサーを実装する
- 抽出したデータをデスクトップ UI で確認・編集できるようにする
- 将来的に ClickPost などの配送管理画面への自動注入を実現する

## 主要構成

- `frontend/`
  - Svelte + Vite の UI プロジェクト
  - `npm run dev` で開発サーバを起動
- `src-tauri/`
  - Tauri の Rust アプリ本体
  - `Cargo.toml` で `tauri = "2.11"` と `wry` を利用
  - `build.rs` で Tauri のビルドフックを実行
- `.gitignore`
  - Rust / Node / Tauri のビルド生成物を除外

## セットアップ

### 前提

- Node.js / npm
- Rust toolchain (`rustup`, `cargo`)

### 開発手順

1. 依存をインストール

```bash
cd /workspaces/LogisticUtils/frontend
npm install
```

2. フロントエンドを起動

```bash
cd /workspaces/LogisticUtils/frontend
npm run dev
```

3. Tauri を起動

```bash
cd /workspaces/LogisticUtils
cargo tauri dev
```

Linux コンテナ環境では Xvfb を使って実行します:

```bash
Xvfb :99 -screen 0 1024x768x24 &
DISPLAY=:99 cargo tauri dev
```

## 今後の優先課題

1. テキスト入力/クリップボードの配送先データ抽出パーサーを実装
2. フロントエンドで抽出結果を表示・編集する UI を作成
3. Tauri バックエンドとフロントエンドの連携を整備
4. ClickPost などの外部配送画面への出力/注入機能を設計
5. Windows/Mac 向けのビルドとパッケージングを検証

## 注意事項

- 現時点では UI と Tauri の連携までを scaffold した段階です。
- Linux コンテナではフル GUI 起動の確認に Xvfb が必要です。
