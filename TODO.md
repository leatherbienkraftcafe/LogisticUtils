# TODO

## 1. 開発の次ステップ

- [x] `frontend` の UI でサンプル入力テキストを受け取り、解析可能にする
- [x] 解析結果を表形式で表示し、編集できるようにする
- [x] Tauri コマンド経由でフロントエンドと Rust バックエンドを接続する
- [x] 解析ロジックを Rust 側パーサーとして整理し、単体テストを追加する
- [x] クリックポストなどの配送管理画面への出力 / 注入設計を検討する（`docs/design/clickpost-output.md`。フェーズ1のクリップボードコピー、フェーズ2のCDP自動注入（アプリ専用Chrome起動＋セレクタ設定ファイル）を実装済み。実際のセレクタ調査と実機での動作確認が未了）

## 2. 環境と検証

- [x] Linux コンテナで `cargo tauri dev` を Xvfb で実行し、表示が安定するか確認（起動・UI表示・テキスト解析(Tauriコマンド経由)・編集を確認。数分間クラッシュなし。ClickPostブラウザ連携はこのコンテナにChromeが無いため未検証）
- [x] フロントエンドの `npm run dev` と `cargo tauri dev` を同時実行して動作確認（`devUrl` が `4173`（vite preview用ポート）のままで `npm run dev`（デフォルト`5173`）と不一致のため `cargo tauri dev` が接続待ちのまま固まる不具合を発見・修正。`devUrl` を `http://localhost:5173` に変更し、同時実行できることを確認）
- [x] `tauri.conf.json` の `frontendDist` と `devUrl` の設定が本番ビルドでも適切か確認（上記の `devUrl` 修正を実施。`frontendDist: ../frontend/dist` は `npm run build` の出力先と一致しており本番ビルドも問題なし）

## 3. ドキュメント・作業引き継ぎ

- [ ] この README を最新の実装状況に合わせて定期更新する
- [ ] フロントエンド/バックエンドの依存関係を明確にする
- [ ] 開発環境セットアップ手順を `README.md` に追記する；特にWindowsとMacの環境

## 4. Git とリリース

- [x] 必要ファイルを `git add` してコミット
- [x] `origin/main` にプッシュ
- [ ] 将来的にリリース用アセットやインストーラーの配置を検討する

## 5. 補助的機能

- [ ] Amazon,Yahooの画面コピペから設定をキャリブレーションできる機能
