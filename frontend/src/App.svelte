<script>
  let payload = '';
  let parsed = null;

  function parseText() {
    const lines = payload
      .split(/\r?\n/)
      .map((line) => line.trim())
      .filter(Boolean);

    const result = {};
    for (const line of lines) {
      const parts = line.split(/[:\t\s]+/);
      if (parts.length >= 2) {
        const key = parts[0];
        const value = parts.slice(1).join(' ');
        result[key] = value;
      }
    }

    parsed = Object.keys(result).length ? result : null;
  }
</script>

<main>
  <h1>LogisticUtils</h1>
  <p>配送先データを貼り付けて、解析結果を確認できます。</p>

  <textarea bind:value={payload} rows="10" placeholder="Amazon/Yahoo/Excel のコピー内容をここに貼り付け"></textarea>
  <button on:click={parseText}>解析</button>

  {#if parsed}
    <section>
      <h2>抽出結果</h2>
      <table>
        <thead>
          <tr><th>キー</th><th>値</th></tr>
        </thead>
        <tbody>
          {#each Object.entries(parsed) as [key, value]}
            <tr>
              <td>{key}</td>
              <td>{value}</td>
            </tr>
          {/each}
        </tbody>
      </table>
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
</style>
