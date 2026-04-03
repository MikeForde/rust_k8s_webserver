pub const INDEX_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>Simple SNOMED Browser</title>
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <style>
    .concept-card {
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 12px;
    background: #fafafa;
    margin-bottom: 12px;
  }
  .concept-card.current {
    background: #eef6ff;
    border-color: #bcd3ee;
  }
  .concept-name {
    font-weight: 600;
  }
  .concept-meta {
    color: #666;
    font-size: 12px;
    margin-top: 2px;
  }
  .concept-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 16px;
  }

  body {
    font-family: Arial, sans-serif;
    margin: 0;
    background: #f6f8fa;
    color: #222;
  }
  .wrap {
    max-width: 960px;
    margin: 0 auto;
    padding: 24px;
    height: 100vh;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
  }
  h1 {
    margin-top: 0;
  }
  .search-row {
    display: flex;
    gap: 12px;
    margin-bottom: 16px;
  }
  input {
    flex: 1;
    padding: 12px;
    font-size: 16px;
  }
  button {
    padding: 12px 16px;
    font-size: 16px;
    cursor: pointer;
  }
  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    flex: 1;
    min-height: 0;
  }
  .panel {
    background: white;
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 16px;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }
  .panel-body {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
  }
  .result {
    padding: 10px;
    border-bottom: 1px solid #eee;
    cursor: pointer;
  }
  .result:hover {
    background: #f1f5f9;
  }
  .code {
    color: #555;
    font-size: 13px;
  }
  .display {
    font-weight: 600;
  }
  .section-title {
    margin-top: 20px;
    margin-bottom: 8px;
  }
  ul.code-list {
    margin: 0;
    padding-left: 20px;
  }
  ul.code-list li {
    margin-bottom: 6px;
    font-family: monospace;
  }
  pre {
    white-space: pre-wrap;
    word-break: break-word;
    background: #f8f8f8;
    padding: 12px;
    border-radius: 6px;
    overflow: auto;
  }
  .muted {
    color: #666;
  }
</style>
</head>
<body>
  <div class="wrap">
    <h1>Simple SNOMED Browser</h1>
    <p class="muted">Search SNOMED CT terms using Snowstorm Lite.</p>

    <div class="search-row">
      <input id="searchBox" placeholder="Type a term, e.g. appendicitis, asthma, penicillin" />
      <button id="searchBtn">Search</button>
    </div>

   <div class="grid">
  <div class="panel">
    <h2>Results</h2>
    <div id="results" class="panel-body"></div>
  </div>

  <div class="panel">
    <h2>Concept Details</h2>
    <div id="details" class="panel-body muted">Select a concept from the results.</div>
  </div>
</div>
  </div>

  <script>
    const searchBox = document.getElementById('searchBox');
    const searchBtn = document.getElementById('searchBtn');
    const resultsEl = document.getElementById('results');
    const detailsEl = document.getElementById('details');

    async function runSearch() {
      const q = searchBox.value.trim();
      if (!q) {
        resultsEl.innerHTML = '<div class="muted">Enter a search term.</div>';
        return;
      }

      resultsEl.innerHTML = '<div class="muted">Searching...</div>';
      detailsEl.innerHTML = '<div class="muted">Select a concept from the results.</div>';

      try {
        const resp = await fetch(`/api/search?q=${encodeURIComponent(q)}`);
        const data = await resp.json();

        if (!Array.isArray(data) || data.length === 0) {
          resultsEl.innerHTML = '<div class="muted">No results found.</div>';
          return;
        }

        resultsEl.innerHTML = '';
        data.forEach(item => {
          const row = document.createElement('div');
          row.className = 'result';
          row.innerHTML = `
            <div class="display">${escapeHtml(item.display)}</div>
            <div class="code">${escapeHtml(item.code)}</div>
          `;
          row.addEventListener('click', () => loadDetails(item.code));
          resultsEl.appendChild(row);
        });
      } catch (err) {
        resultsEl.innerHTML = `<div class="muted">Search failed: ${escapeHtml(String(err))}</div>`;
      }
    }

    async function loadDetails(code) {
      detailsEl.innerHTML = '<div class="muted">Loading details...</div>';

      try {
        const resp = await fetch(`/api/lookup/${encodeURIComponent(code)}`);
        const data = await resp.json();

        detailsEl.innerHTML = `
            <h3 class="section-title">Parents</h3>
            ${renderConceptList(data.parents)}

            <h3 class="section-title">Concept</h3>
            <div class="concept-card current">
                <div class="concept-name">${escapeHtml(data.display || '')}</div>
                <div class="concept-meta">${escapeHtml(data.code || '')}</div>
                <div style="margin-top:8px;"><strong>FSN:</strong> ${escapeHtml(data.fsn || '')}</div>
                <div><strong>Inactive:</strong> ${escapeHtml(String(data.inactive))}</div>
                <div><strong>Effective time:</strong> ${escapeHtml(data.effective_time || '')}</div>
            </div>

            <h3 class="section-title">Children</h3>
            ${renderConceptList(data.children)}

            <h3>Raw</h3>
            <pre>${escapeHtml(JSON.stringify(data, null, 2))}</pre>
            `;
      } catch (err) {
        detailsEl.innerHTML = `<div class="muted">Lookup failed: ${escapeHtml(String(err))}</div>`;
      }
    }

    function renderConceptList(items) {
        if (!Array.isArray(items) || items.length === 0) {
            return '<div class="muted">None</div>';
        }

        return `
            <div class="concept-list">
            ${items.map(item => `
                <div class="concept-card">
                <div class="concept-name">${escapeHtml(item.display || '')}</div>
                <div class="concept-meta">(${escapeHtml(item.code || '')})</div>
                </div>
            `).join('')}
            </div>
        `;
        }

    function escapeHtml(str) {
      return String(str)
        .replaceAll('&', '&amp;')
        .replaceAll('<', '&lt;')
        .replaceAll('>', '&gt;')
        .replaceAll('"', '&quot;')
        .replaceAll("'", '&#39;');
    }

    searchBtn.addEventListener('click', runSearch);
    searchBox.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') runSearch();
    });
  </script>
</body>
</html>
"#;