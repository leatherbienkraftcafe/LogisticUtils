<script>
  import { invoke } from '@tauri-apps/api/core';

  let payload = '';
  let rows = [];
  let error = '';

  const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

  function parseLocally(text) {
    const lines = text
      .split(/\r?\n/)
      .map((line) => line.trim())
      .filter(Boolean);

    const result = [];
    for (const line of lines) {
      const parts = line.split(/[:\t\s]+/);
      if (parts.length >= 2) {
        const key = parts[0];
        const value = parts.slice(1).join(' ');
        result.push({ key, value });
      }
    }
    return result;
  }

  async function parseText() {
    error = '';
    if (isTauri) {
      try {
        rows = await invoke('parse_text', { payload });
        return;
      } catch (err) {
        error = `Rust側の解析に失敗したため、ブラウザ内解析にフォールバックしました: ${err}`;
      }
    }
    rows = parseLocally(payload);
  }

  function addRow() {
    rows = [...rows, { key: '', value: '' }];
  }

  function removeRow(index) {
    rows = rows.filter((_, i) => i !== index);
  }

  let copyStatus = '';

  async function copyForClickPost() {
    const tsv = rows.map((row) => row.value).join('\t');
    try {
      await navigator.clipboard.writeText(tsv);
      copyStatus = 'コピーしました。クリックポストなどの入力欄に貼り付けてください。';
    } catch (err) {
      copyStatus = `コピーに失敗しました: ${err}`;
    }
  }

  let clickpostUrl = 'https://clickpost.jp/';
  let browserStatus = '';

  async function openClickPostBrowser() {
    browserStatus = '';
    try {
      await invoke('open_clickpost_browser', { url: clickpostUrl });
      browserStatus = 'ブラウザを開きました。画面上でログインなどを行ってください。';
    } catch (err) {
      browserStatus = `ブラウザの起動に失敗しました: ${err}`;
    }
  }

  async function injectRows() {
    browserStatus = '';
    try {
      const result = await invoke('inject_rows', { rows });
      browserStatus = `連携結果: ${result.join(' / ')}`;
    } catch (err) {
      browserStatus = `データ連携に失敗しました: ${err}`;
    }
  }
</script>

<main>
  <h1>LogisticUtils</h1>
  <p>配送先データを貼り付けて、解析結果を確認・編集できます。</p>

  <textarea bind:value={payload} rows="10" placeholder="Amazon/Yahoo/Excel のコピー内容をここに貼り付け"></textarea>
  <button on:click={parseText}>解析</button>

  {#if error}
    <p class="error">{error}</p>
  {/if}

  {#if rows.length}
    <section>
      <h2>抽出結果（編集可）</h2>
      <table>
        <thead>
          <tr><th>キー</th><th>値</th><th></th></tr>
        </thead>
        <tbody>
          {#each rows as row, i (i)}
            <tr>
              <td><input type="text" bind:value={row.key} /></td>
              <td><input type="text" bind:value={row.value} /></td>
              <td><button class="remove" on:click={() => removeRow(i)}>削除</button></td>
            </tr>
          {/each}
        </tbody>
      </table>
      <button on:click={addRow}>行を追加</button>
      <button on:click={copyForClickPost}>クリックポスト用にコピー（タブ区切り）</button>
      {#if copyStatus}
        <p class="status">{copyStatus}</p>
      {/if}

      {#if isTauri}
        <div class="browser-panel">
          <h3>ブラウザ連携（実験的）</h3>
          <p class="hint">
            専用の Chrome を起動し、認証はその画面で手動で行ってください。
            対象欄への反映は <code>clickpost-field-map.json</code>（アプリの設定フォルダ）の
            セレクタ設定に従います。未設定の項目は反映されません。
          </p>
          <input type="text" bind:value={clickpostUrl} placeholder="対象画面の URL" />
          <button on:click={openClickPostBrowser}>ClickPostを開く</button>
          <button on:click={injectRows}>データを連携</button>
          {#if browserStatus}
            <p class="status">{browserStatus}</p>
          {/if}
        </div>
      {/if}
    </section>
  {/if}
</main>

<style>
  main {
    padding: 32px;
    font-family: system-ui, sans-serif;
    line-height: 1.6;
  }

  textarea {
    width: 100%;
    min-height: 240px;
    font-family: ui-monospace, monospace;
    margin-bottom: 16px;
    padding: 12px;
    border: 1px solid #ccc;
    border-radius: 8px;
  }

  button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 10px 18px;
    border: none;
    border-radius: 8px;
    background: #0f62fe;
    color: white;
    cursor: pointer;
    transition: background 0.2s ease;
  }

  button:hover {
    background: #0b53d1;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 20px;
  }

  th,
  td {
    border: 1px solid #ddd;
    padding: 10px;
    text-align: left;
  }

  th {
    background: #f4f4f4;
  }

  td input[type='text'] {
    width: 100%;
    box-sizing: border-box;
    padding: 6px 8px;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-family: inherit;
    font-size: inherit;
  }

  button.remove {
    background: #da1e28;
    padding: 6px 12px;
  }

  button.remove:hover {
    background: #b0151f;
  }

  .error {
    color: #da1e28;
  }

  .status {
    color: #24a148;
  }

  .browser-panel {
    margin-top: 24px;
    padding: 16px;
    border: 1px dashed #ccc;
    border-radius: 8px;
  }

  .browser-panel input[type='text'] {
    width: 100%;
    box-sizing: border-box;
    padding: 8px 10px;
    margin-bottom: 12px;
    border: 1px solid #ccc;
    border-radius: 6px;
    font-family: inherit;
  }

  .hint {
    font-size: 0.9em;
    color: #555;
  }
</style>
