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
</style>
