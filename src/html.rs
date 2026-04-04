const PAGE_TEMPLATE: &str = r##"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>__PAGE_TITLE__</title>
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <style>
__STYLES__
  </style>
</head>
<body>
  <div class="shell">
    <nav class="topbar">
      <a class="brand" href="/">Snowstorm Lite Console</a>
      <div class="topbar-links">__NAV__</div>
    </nav>

    <main class="wrap">
      <header class="page-header">
        <h1>__HEADING__</h1>
        <p class="muted lead">__SUBTITLE__</p>
      </header>
      __BODY__
    </main>
  </div>

  <script>
__SCRIPT__
  </script>
</body>
</html>
"##;

const SHARED_STYLES: &str = r##"
    :root {
      color-scheme: light;
      --bg: #f4f7fb;
      --card: #ffffff;
      --card-soft: #f8fbff;
      --border: #d8e2f0;
      --border-strong: #b6c7de;
      --text: #17324d;
      --muted: #5f7288;
      --accent: #245ea8;
      --accent-soft: #e8f1fb;
      --danger: #a43737;
      --danger-soft: #fff1f1;
      --shadow: 0 12px 36px rgba(23, 50, 77, 0.08);
    }

    * {
      box-sizing: border-box;
    }

    body {
      margin: 0;
      background: var(--bg);
      color: var(--text);
      font-family: Arial, sans-serif;
    }

    a {
      color: inherit;
      text-decoration: none;
    }

    .shell {
      min-height: 100vh;
    }

    .topbar {
      position: sticky;
      top: 0;
      z-index: 20;
      display: flex;
      flex-wrap: wrap;
      justify-content: space-between;
      gap: 12px;
      padding: 16px 24px;
      border-bottom: 1px solid rgba(216, 226, 240, 0.9);
      background: rgba(255, 255, 255, 0.95);
      backdrop-filter: blur(10px);
    }

    .brand {
      font-weight: 700;
      letter-spacing: 0.02em;
    }

    .topbar-links {
      display: flex;
      flex-wrap: wrap;
      gap: 8px;
    }

    .nav-link {
      padding: 8px 12px;
      border: 1px solid transparent;
      border-radius: 999px;
      color: var(--muted);
      font-size: 14px;
    }

    .nav-link:hover {
      background: #eef4fb;
      color: var(--accent);
    }

    .nav-link.active {
      background: var(--accent-soft);
      border-color: #c7daf5;
      color: var(--accent);
      font-weight: 700;
    }

    .wrap {
      max-width: 1360px;
      margin: 0 auto;
      padding: 24px;
    }

    .page-header {
      margin-bottom: 18px;
    }

    h1,
    h2,
    h3 {
      margin: 0 0 10px;
    }

    h2 {
      font-size: 19px;
    }

    .lead {
      margin: 0;
      max-width: 900px;
    }

    .muted {
      color: var(--muted);
    }

    .small {
      font-size: 13px;
    }

    .grid,
    .tool-grid,
    .link-strip {
      display: grid;
      gap: 16px;
    }

    .grid {
      grid-template-columns: minmax(0, 1fr) minmax(0, 1.2fr);
      margin-bottom: 16px;
    }

    .tool-grid {
      grid-template-columns: 320px minmax(0, 1fr) minmax(0, 1fr);
      align-items: start;
    }

    .link-strip {
      grid-template-columns: repeat(4, minmax(0, 1fr));
      margin-top: 18px;
    }

    .panel,
    .mini-card {
      background: var(--card);
      border: 1px solid var(--border);
      border-radius: 16px;
      box-shadow: var(--shadow);
    }

    .panel {
      padding: 18px;
      min-width: 0;
    }

    .panel-body {
      min-width: 0;
    }

    .mini-card {
      display: block;
      padding: 16px;
      background: linear-gradient(180deg, #ffffff, #f8fbff);
    }

    .mini-card strong,
    .operation-name,
    .display,
    .concept-name {
      display: block;
      font-weight: 700;
    }

    .mini-card span {
      display: block;
      margin-top: 6px;
      color: var(--muted);
      font-size: 14px;
      line-height: 1.45;
    }

    .mini-card:hover,
    .result:hover,
    .nav-card:hover,
    .operation-card:hover {
      border-color: var(--border-strong);
      background: var(--card-soft);
    }

    .search-row,
    .toolbar,
    .field-row,
    .operation-title-row,
    .response-meta {
      display: flex;
      gap: 12px;
      align-items: center;
      flex-wrap: wrap;
    }

    .search-row {
      margin-bottom: 16px;
    }

    .search-row .grow {
      flex: 1 1 320px;
    }

    .toggle {
      display: inline-flex;
      align-items: center;
      gap: 10px;
      padding: 12px 14px;
      border: 1px solid var(--border);
      border-radius: 12px;
      background: #fff;
      color: var(--text);
      font-size: 14px;
      font-weight: 700;
      white-space: nowrap;
    }

    .toggle input {
      width: 16px;
      height: 16px;
      margin: 0;
      padding: 0;
      border: 0;
      background: transparent;
      accent-color: var(--accent);
    }

    .field-row.two-col {
      display: grid;
      grid-template-columns: 150px minmax(0, 1fr);
      gap: 12px;
    }

    input,
    textarea,
    select,
    button {
      width: 100%;
      border-radius: 12px;
      border: 1px solid var(--border-strong);
      font: inherit;
    }

    input,
    textarea,
    select {
      padding: 12px 14px;
      background: #fff;
      color: var(--text);
    }

    textarea {
      resize: vertical;
      min-height: 120px;
      line-height: 1.45;
    }

    .code-area,
    pre,
    .code {
      font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
    }

    button {
      width: auto;
      padding: 11px 16px;
      cursor: pointer;
      background: var(--accent);
      color: #fff;
      border-color: var(--accent);
      font-weight: 700;
    }

    button.secondary {
      background: #fff;
      color: var(--text);
      border-color: var(--border-strong);
    }

    button:disabled {
      opacity: 0.7;
      cursor: wait;
    }

    .field {
      display: block;
      margin-bottom: 14px;
    }

    .field span {
      display: block;
      margin-bottom: 6px;
      font-weight: 700;
      font-size: 14px;
    }

    .operation-list,
    .concept-list {
      display: flex;
      flex-direction: column;
      gap: 10px;
    }

    .operation-card,
    .concept-card,
    .result {
      width: 100%;
      padding: 14px;
      border: 1px solid var(--border);
      border-radius: 14px;
      background: #fff;
      text-align: left;
    }

    .operation-card.active {
      border-color: #83aee3;
      background: var(--accent-soft);
    }

    .operation-card {
      box-shadow: none;
    }

    .operation-title-row {
      justify-content: space-between;
      margin-bottom: 10px;
    }

    .method-badge,
    .protected-badge {
      display: inline-flex;
      align-items: center;
      padding: 4px 9px;
      border-radius: 999px;
      font-size: 12px;
      font-weight: 700;
    }

    .method-badge {
      background: #eaf3fe;
      color: var(--accent);
    }

    .protected-badge {
      background: #fff4dc;
      color: #8a6112;
    }

    .protected-badge.quiet {
      background: #edf3f8;
      color: var(--muted);
    }

    .result,
    .nav-card {
      cursor: pointer;
    }

    .concept-card.current {
      background: var(--accent-soft);
      border-color: #83aee3;
    }

    .result.inactive,
    .concept-card.inactive {
      background: #fff3f3;
      border-color: #eccaca;
    }

    .concept-card.current.inactive {
      background: linear-gradient(180deg, #eef5ff, #fff3f3);
      border-color: #d7b3b3;
    }

    .result-header,
    .concept-header {
      display: flex;
      align-items: start;
      justify-content: space-between;
      gap: 12px;
    }

    .inactive-badge {
      display: inline-flex;
      align-items: center;
      padding: 4px 8px;
      border-radius: 999px;
      background: #f8dede;
      color: #8d3f3f;
      font-size: 12px;
      font-weight: 700;
      white-space: nowrap;
    }

    .results-summary {
      margin-bottom: 12px;
    }

    .concept-meta,
    .code {
      color: var(--muted);
      font-size: 13px;
      margin-top: 4px;
    }

    .section-title {
      margin-top: 18px;
      margin-bottom: 8px;
    }

    .callout,
    .error-box {
      padding: 12px 14px;
      border-radius: 12px;
      margin-bottom: 14px;
      line-height: 1.5;
    }

    .callout {
      background: #f2f7fd;
      border: 1px solid #d6e6f8;
    }

    .error-box {
      background: var(--danger-soft);
      border: 1px solid #e8bdbd;
      color: var(--danger);
    }

    .response-meta {
      margin-bottom: 12px;
      font-size: 14px;
    }

    pre {
      margin: 0;
      padding: 14px;
      border-radius: 12px;
      border: 1px solid #e4ebf3;
      background: #f7f9fc;
      white-space: pre-wrap;
      word-break: break-word;
      overflow: auto;
      min-height: 240px;
    }

    @media (max-width: 1180px) {
      .tool-grid {
        grid-template-columns: 1fr;
      }

      .link-strip {
        grid-template-columns: repeat(2, minmax(0, 1fr));
      }
    }

    @media (max-width: 760px) {
      .wrap,
      .topbar {
        padding: 16px;
      }

      .grid,
      .link-strip,
      .field-row.two-col {
        grid-template-columns: 1fr;
      }

      .search-row,
      .toolbar {
        flex-direction: column;
        align-items: stretch;
      }

      button {
        width: 100%;
      }
    }
"##;

const BROWSER_BODY: &str = r##"
    <div class="search-row">
      <input id="searchBox" class="grow" placeholder="Type a term, e.g. appendicitis, asthma, penicillin" />
      <label class="toggle" for="inactiveFilter">
        <input id="inactiveFilter" type="checkbox" />
        <span>Hide inactive</span>
      </label>
      <button id="searchBtn">Search</button>
    </div>

    <section class="grid">
      <div class="panel">
        <h2>Results</h2>
        <div id="results" class="panel-body muted">Enter a search term to begin.</div>
      </div>

      <div class="panel">
        <h2>Concept Details</h2>
        <div id="details" class="panel-body muted">Select a concept from the results.</div>
      </div>
    </section>

    <section class="link-strip">
      <a class="mini-card" href="/codesystems">
        <strong>Code Systems</strong>
        <span>List code systems, run lookups, and test concept subsumption.</span>
      </a>
      <a class="mini-card" href="/valuesets">
        <strong>Value Set Explorer</strong>
        <span>Expand value sets, run ECL queries, and validate codes.</span>
      </a>
      <a class="mini-card" href="/mapping">
        <strong>Mapping and Tools</strong>
        <span>Use translate, metadata, batch lookup, and partial hierarchy utilities.</span>
      </a>
      <a class="mini-card" href="/admin">
        <strong>Admin</strong>
        <span>Create, update, and delete ValueSets with password-protected actions.</span>
      </a>
    </section>
"##;

const BROWSER_SCRIPT_TEMPLATE: &str = r##"
    const INACTIVE_FILTERED_DEFAULT = __INACTIVE_FILTERED__;
    const searchBox = document.getElementById('searchBox');
    const searchBtn = document.getElementById('searchBtn');
    const inactiveFilterEl = document.getElementById('inactiveFilter');
    const resultsEl = document.getElementById('results');
    const detailsEl = document.getElementById('details');
    let hasSearched = false;
    let lastSearchResults = [];

    inactiveFilterEl.checked = INACTIVE_FILTERED_DEFAULT;

    async function runSearch() {
      const q = searchBox.value.trim();
      if (!q) {
        hasSearched = false;
        lastSearchResults = [];
        resultsEl.innerHTML = '<div class="muted">Enter a search term.</div>';
        return;
      }

      hasSearched = true;
      resultsEl.innerHTML = '<div class="muted">Searching...</div>';
      detailsEl.innerHTML = '<div class="muted">Select a concept from the results.</div>';

      try {
        const resp = await fetch(`/api/search?q=${encodeURIComponent(q)}`);
        const data = await resp.json();

        if (!Array.isArray(data) || data.length === 0) {
          lastSearchResults = [];
          resultsEl.innerHTML = '<div class="muted">No results found.</div>';
          return;
        }

        lastSearchResults = data;
        renderSearchResults();
      } catch (err) {
        lastSearchResults = [];
        resultsEl.innerHTML = `<div class="muted">Search failed: ${escapeHtml(String(err))}</div>`;
      }
    }

    function renderSearchResults() {
      if (!Array.isArray(lastSearchResults) || lastSearchResults.length === 0) {
        resultsEl.innerHTML = hasSearched
          ? '<div class="muted">No results found.</div>'
          : '<div class="muted">Enter a search term to begin.</div>';
        return;
      }

      const filteredResults = inactiveFilterEl.checked
        ? lastSearchResults.filter(item => item.inactive !== true)
        : lastSearchResults;
      const hiddenCount = lastSearchResults.length - filteredResults.length;

      if (filteredResults.length === 0) {
        resultsEl.innerHTML = hiddenCount > 0
          ? `<div class="muted">No active results found. Uncheck the filter to show ${hiddenCount} inactive concept${hiddenCount === 1 ? '' : 's'}.</div>`
          : '<div class="muted">No results found.</div>';
        return;
      }

      resultsEl.innerHTML = `
        ${hiddenCount > 0 ? `<div class="results-summary muted small">Showing ${filteredResults.length} of ${lastSearchResults.length} results. ${hiddenCount} inactive concept${hiddenCount === 1 ? '' : 's'} hidden.</div>` : ''}
        ${filteredResults.map(item => `
          <div class="result${item.inactive === true ? ' inactive' : ''}" data-code="${escapeHtml(item.code || '')}">
            <div class="result-header">
              <div>
                <div class="display">${escapeHtml(item.display || '')}</div>
                <div class="code">${escapeHtml(item.code || '')}</div>
              </div>
              ${item.inactive === true ? '<span class="inactive-badge">Inactive</span>' : ''}
            </div>
          </div>
        `).join('')}
      `;

      resultsEl.querySelectorAll('[data-code]').forEach(el => {
        el.addEventListener('click', () => {
          const code = el.getAttribute('data-code');
          if (code) {
            loadDetails(code);
          }
        });
      });
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
          <div class="concept-card current${data.inactive === true ? ' inactive' : ''}">
            <div class="concept-header">
              <div>
                <div class="concept-name">${escapeHtml(data.display || '')}</div>
                <div class="concept-meta">${escapeHtml(data.code || '')}</div>
              </div>
              ${data.inactive === true ? '<span class="inactive-badge">Inactive</span>' : ''}
            </div>
            <div style="margin-top:8px;"><strong>FSN:</strong> ${escapeHtml(data.fsn || '')}</div>
            <div><strong>Inactive:</strong> ${escapeHtml(String(data.inactive))}</div>
            <div><strong>Effective time:</strong> ${escapeHtml(data.effective_time || '')}</div>
          </div>

          <h3 class="section-title">Children</h3>
          ${renderConceptList(data.children)}

          <h3 class="section-title">Raw</h3>
          <pre>${escapeHtml(JSON.stringify(data, null, 2))}</pre>
        `;

        detailsEl.querySelectorAll('[data-code]').forEach(el => {
          el.addEventListener('click', () => {
            const nextCode = el.getAttribute('data-code');
            if (nextCode) {
              loadDetails(nextCode);
            }
          });
        });
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
            <div class="concept-card nav-card" data-code="${escapeHtml(item.code || '')}">
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
    inactiveFilterEl.addEventListener('change', renderSearchResults);
    searchBox.addEventListener('keydown', event => {
      if (event.key === 'Enter') {
        runSearch();
      }
    });
"##;

const TOOL_BODY: &str = r##"
    <section class="tool-grid">
      <div class="panel">
        <h2>Examples</h2>
        <div id="operationList" class="panel-body operation-list"></div>
      </div>

      <div class="panel">
        <h2>Request</h2>
        <div class="panel-body">
          <div id="operationDescription" class="callout muted">Choose an example and adjust the request before sending it.</div>

          <div class="field-row two-col">
            <label class="field">
              <span>Method</span>
              <select id="methodInput">
                <option>GET</option>
                <option>POST</option>
                <option>PUT</option>
                <option>DELETE</option>
              </select>
            </label>

            <label class="field">
              <span>FHIR path</span>
              <input id="pathInput" placeholder="CodeSystem/$lookup" />
            </label>
          </div>

          <label class="field">
            <span>Query parameters</span>
            <textarea id="queryInput" rows="9" placeholder="key=value, one per line"></textarea>
          </label>

          <label class="field">
            <span>JSON body</span>
            <textarea id="bodyInput" class="code-area" rows="16" placeholder="{ }"></textarea>
          </label>

          <label id="passwordField" class="field" hidden>
            <span>Admin password</span>
            <input id="passwordInput" type="password" placeholder="Required for protected actions" />
          </label>

          <div id="requestError" class="error-box" hidden></div>

          <div class="toolbar">
            <button id="runRequestBtn">Run request</button>
            <button id="resetRequestBtn" class="secondary" type="button">Reset example</button>
          </div>
        </div>
      </div>

      <div class="panel">
        <h2>Response</h2>
        <div class="panel-body">
          <div id="responseMeta" class="response-meta muted">Run an operation to inspect the Snowstorm Lite response.</div>
          <pre id="responseBody">No response yet.</pre>
        </div>
      </div>
    </section>
"##;

const TOOL_SCRIPT_TEMPLATE: &str = r##"
    const OPERATIONS = __OPERATIONS__;

    const operationList = document.getElementById('operationList');
    const operationDescription = document.getElementById('operationDescription');
    const methodInput = document.getElementById('methodInput');
    const pathInput = document.getElementById('pathInput');
    const queryInput = document.getElementById('queryInput');
    const bodyInput = document.getElementById('bodyInput');
    const passwordField = document.getElementById('passwordField');
    const passwordInput = document.getElementById('passwordInput');
    const requestError = document.getElementById('requestError');
    const responseMeta = document.getElementById('responseMeta');
    const responseBody = document.getElementById('responseBody');
    const runRequestBtn = document.getElementById('runRequestBtn');
    const resetRequestBtn = document.getElementById('resetRequestBtn');

    let selectedIndex = 0;

    function renderOperations() {
      operationList.innerHTML = OPERATIONS.map((operation, index) => `
        <button type="button" class="operation-card${index === selectedIndex ? ' active' : ''}" data-index="${index}">
          <div class="operation-title-row">
            <span class="method-badge">${escapeHtml(operation.method)}</span>
            <span class="protected-badge${operation.destructive ? '' : ' quiet'}">${operation.destructive ? 'Protected' : 'Read only'}</span>
          </div>
          <span class="operation-name">${escapeHtml(operation.name)}</span>
          <span class="muted small">${escapeHtml(operation.description || '')}</span>
        </button>
      `).join('');

      operationList.querySelectorAll('[data-index]').forEach(button => {
        button.addEventListener('click', () => {
          const index = Number(button.getAttribute('data-index'));
          selectOperation(index);
        });
      });
    }

    function selectOperation(index) {
      selectedIndex = index;
      const operation = OPERATIONS[index];
      methodInput.value = operation.method;
      pathInput.value = operation.path || '';
      queryInput.value = formatQuery(operation.query || []);
      bodyInput.value = operation.body ? JSON.stringify(operation.body, null, 2) : '';
      passwordField.hidden = !operation.destructive;
      if (!operation.destructive) {
        passwordInput.value = '';
      }
      operationDescription.textContent = operation.longDescription || operation.description || 'Adjust the request and send it to Snowstorm Lite.';
      clearError();
      renderOperations();
    }

    function formatQuery(query) {
      return query.map(item => `${item.key}=${item.value}`).join('\n');
    }

    function parseQuery(value) {
      return value
        .split('\n')
        .map(line => line.trim())
        .filter(line => line && !line.startsWith('#'))
        .map(line => {
          const separator = line.indexOf('=');
          if (separator === -1) {
            throw new Error(`Invalid query line: ${line}`);
          }
          return {
            key: line.slice(0, separator).trim(),
            value: line.slice(separator + 1).trim(),
          };
        });
    }

    function parseBody(value) {
      if (!value.trim()) {
        return null;
      }

      return JSON.parse(value);
    }

    function clearError() {
      requestError.hidden = true;
      requestError.textContent = '';
    }

    function setError(message) {
      requestError.hidden = false;
      requestError.textContent = message;
    }

    function prettyPrint(text) {
      if (!text) {
        return '';
      }

      try {
        return JSON.stringify(JSON.parse(text), null, 2);
      } catch (_) {
        return text;
      }
    }

    async function runRequest() {
      const operation = OPERATIONS[selectedIndex];
      clearError();

      if (operation.destructive && !passwordInput.value.trim()) {
        setError('Enter the admin password for this protected action.');
        return;
      }

      let payload;
      try {
        payload = {
          method: methodInput.value,
          path: pathInput.value.trim(),
          query: parseQuery(queryInput.value),
          body: parseBody(bodyInput.value),
          password: passwordInput.value.trim() || undefined,
        };
      } catch (error) {
        setError(error instanceof Error ? error.message : String(error));
        return;
      }

      runRequestBtn.disabled = true;
      responseMeta.textContent = 'Running request...';
      responseBody.textContent = 'Waiting for Snowstorm Lite response...';

      try {
        const response = await fetch('/api/fhir', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(payload),
        });

        const text = await response.text();
        const contentType = response.headers.get('content-type') || 'unknown content type';
        responseMeta.textContent = `${response.status} ${response.statusText} | ${contentType}`;
        responseBody.textContent = prettyPrint(text);
      } catch (error) {
        responseMeta.textContent = 'Request failed';
        responseBody.textContent = error instanceof Error ? error.message : String(error);
      } finally {
        runRequestBtn.disabled = false;
      }
    }

    function escapeHtml(str) {
      return String(str)
        .replaceAll('&', '&amp;')
        .replaceAll('<', '&lt;')
        .replaceAll('>', '&gt;')
        .replaceAll('"', '&quot;')
        .replaceAll("'", '&#39;');
    }

    runRequestBtn.addEventListener('click', runRequest);
    resetRequestBtn.addEventListener('click', () => selectOperation(selectedIndex));

    renderOperations();
    selectOperation(0);
"##;

const CODESYSTEM_OPERATIONS: &str = r##"[
  {
    "name": "List Code Systems",
    "description": "Browse CodeSystem resources available on the server.",
    "method": "GET",
    "path": "CodeSystem",
    "query": [
      { "key": "_count", "value": "10" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Find CodeSystem by URL and version",
    "description": "Match a SNOMED CodeSystem resource by canonical URL and version.",
    "method": "GET",
    "path": "CodeSystem",
    "query": [
      { "key": "url", "value": "http://snomed.info/sct" },
      { "key": "version", "value": "http://snomed.info/sct/900000000000207008/version/20241101" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Lookup concept by code",
    "description": "Run CodeSystem/$lookup for a SNOMED CT concept.",
    "method": "GET",
    "path": "CodeSystem/$lookup",
    "query": [
      { "key": "system", "value": "http://snomed.info/sct" },
      { "key": "code", "value": "73211009" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Lookup concept with properties",
    "description": "Request specific lookup properties such as normalForm and sufficientlyDefined.",
    "method": "GET",
    "path": "CodeSystem/$lookup",
    "query": [
      { "key": "system", "value": "http://snomed.info/sct" },
      { "key": "code", "value": "73211009" },
      { "key": "property", "value": "normalForm" },
      { "key": "property", "value": "sufficientlyDefined" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Test concept subsumption",
    "description": "Use CodeSystem/$subsumes to compare two concepts.",
    "method": "GET",
    "path": "CodeSystem/$subsumes",
    "query": [
      { "key": "system", "value": "http://snomed.info/sct" },
      { "key": "codeA", "value": "307355007" },
      { "key": "codeB", "value": "118940003" }
    ],
    "body": null,
    "destructive": false
  }
]"##;

const VALUESET_OPERATIONS: &str = r##"[
  {
    "name": "List ValueSets",
    "description": "List ValueSet resources stored on the server.",
    "method": "GET",
    "path": "ValueSet",
    "query": [],
    "body": null,
    "destructive": false
  },
  {
    "name": "Expand named ValueSet",
    "description": "Expand a named ValueSet by canonical URL.",
    "method": "GET",
    "path": "ValueSet/$expand",
    "query": [
      { "key": "url", "value": "http://example.com/value-sets/chronic-diseases" },
      { "key": "count", "value": "100" },
      { "key": "offset", "value": "0" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "List descendants of a concept",
    "description": "Expand an implicit SNOMED descendant set using isa.",
    "method": "GET",
    "path": "ValueSet/$expand",
    "query": [
      { "key": "url", "value": "http://snomed.info/sct?fhir_vs=isa/27624003" },
      { "key": "count", "value": "10" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Run ECL query",
    "description": "Execute an ECL-backed expansion with a term filter.",
    "method": "GET",
    "path": "ValueSet/$expand",
    "query": [
      { "key": "url", "value": "http://snomed.info/sct?fhir_vs=ecl/<<404684003 |Clinical finding (finding)|" },
      { "key": "filter", "value": "skin" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Run fuzzy ECL query",
    "description": "Use fuzzy term matching by appending a tilde to the filter.",
    "method": "GET",
    "path": "ValueSet/$expand",
    "query": [
      { "key": "url", "value": "http://snomed.info/sct?fhir_vs=ecl/<404684003" },
      { "key": "filter", "value": "athma~" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Search a ValueSet for a term",
    "description": "Filter medicinal products to items containing aspirin.",
    "method": "GET",
    "path": "ValueSet/$expand",
    "query": [
      { "key": "url", "value": "http://snomed.info/sct?fhir_vs=ecl/<<763158003" },
      { "key": "filter", "value": "Aspirin" },
      { "key": "count", "value": "10" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "List refset members",
    "description": "Expand the members of a SNOMED reference set.",
    "method": "GET",
    "path": "ValueSet/$expand",
    "query": [
      { "key": "url", "value": "http://snomed.info/sct?fhir_vs=refset/723264001" },
      { "key": "count", "value": "10" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Expand implicit ValueSet via POST",
    "description": "Submit a Parameters resource to expand an inline ValueSet definition.",
    "method": "POST",
    "path": "ValueSet/$expand",
    "query": [],
    "body": {
      "resourceType": "Parameters",
      "parameter": [
        {
          "name": "valueSet",
          "resource": {
            "resourceType": "ValueSet",
            "compose": {
              "include": [
                {
                  "system": "http://snomed.info/sct",
                  "filter": [
                    {
                      "property": "parent",
                      "op": "=",
                      "value": "138875005"
                    }
                  ]
                }
              ]
            }
          }
        }
      ]
    },
    "destructive": false
  },
  {
    "name": "Expand implicit ValueSet with properties",
    "description": "Ask the server to include extra properties in the expansion.",
    "method": "POST",
    "path": "ValueSet/$expand",
    "query": [],
    "body": {
      "resourceType": "Parameters",
      "parameter": [
        {
          "name": "valueSet",
          "resource": {
            "resourceType": "ValueSet",
            "compose": {
              "include": [
                {
                  "system": "http://snomed.info/sct",
                  "filter": [
                    {
                      "property": "parent",
                      "op": "=",
                      "value": "138875005"
                    }
                  ]
                }
              ]
            }
          }
        },
        { "name": "property", "valueString": "sufficientlyDefined" },
        { "name": "property", "valueString": "inactive" },
        { "name": "property", "valueString": "parent" },
        { "name": "count", "valueInteger": 1000 }
      ]
    },
    "destructive": false
  },
  {
    "name": "Expand all SNOMED concepts",
    "description": "Browse the implicit ValueSet of all SNOMED concepts.",
    "method": "GET",
    "path": "ValueSet/$expand",
    "query": [
      { "key": "url", "value": "http://snomed.info/sct?fhir_vs" },
      { "key": "count", "value": "10" },
      { "key": "offset", "value": "0" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Search all SNOMED concepts",
    "description": "Search within the implicit set of all SNOMED concepts.",
    "method": "GET",
    "path": "ValueSet/$expand",
    "query": [
      { "key": "url", "value": "http://snomed.info/sct?fhir_vs" },
      { "key": "count", "value": "10" },
      { "key": "offset", "value": "0" },
      { "key": "filter", "value": "myocar inf" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Validate code with GET",
    "description": "Check whether a code is part of a SNOMED reference set.",
    "method": "GET",
    "path": "ValueSet/$validate-code",
    "query": [
      { "key": "url", "value": "http://snomed.info/sct?fhir_vs=refset/723264001" },
      { "key": "system", "value": "http://snomed.info/sct" },
      { "key": "code", "value": "77905000" },
      { "key": "display", "value": "Adrenal vein" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Validate code with POST",
    "description": "Submit Parameters to ValueSet/$validate-code.",
    "method": "POST",
    "path": "ValueSet/$validate-code",
    "query": [],
    "body": {
      "resourceType": "Parameters",
      "parameter": [
        { "name": "url", "valueUri": "http://snomed.info/sct?fhir_vs=isa/27624003" },
        {
          "name": "coding",
          "valueCoding": {
            "system": "http://snomed.info/sct",
            "code": "707480001"
          }
        }
      ]
    },
    "destructive": false
  }
]"##;

const MAPPING_OPERATIONS: &str = r##"[
  {
    "name": "Translate inactive SNOMED code",
    "description": "Find the active replacement using the SAME AS historical association map.",
    "method": "GET",
    "path": "ConceptMap/$translate",
    "query": [
      { "key": "code", "value": "134811001" },
      { "key": "system", "value": "http://snomed.info/sct" },
      { "key": "url", "value": "http://snomed.info/sct?fhir_cm=900000000000527005" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Translate SNOMED to ICD-10",
    "description": "Translate a SNOMED concept into an ICD-10 target code.",
    "method": "GET",
    "path": "ConceptMap/$translate",
    "query": [
      { "key": "code", "value": "254153009" },
      { "key": "system", "value": "http://snomed.info/sct" },
      { "key": "targetsystem", "value": "http://hl7.org/fhir/sid/icd-10" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Translate SNOMED to ICD-10-CM",
    "description": "Translate using an edition-specific SNOMED version and an ICD-10-CM target system.",
    "method": "GET",
    "path": "ConceptMap/$translate",
    "query": [
      { "key": "code", "value": "254153009" },
      { "key": "system", "value": "http://snomed.info/sct" },
      { "key": "version", "value": "http://snomed.info/sct/731000124108" },
      { "key": "targetsystem", "value": "http://hl7.org/fhir/sid/icd-10-cm" }
    ],
    "body": null,
    "destructive": false
  },
  {
    "name": "Batch lookup",
    "description": "Send a FHIR batch bundle with multiple CodeSystem/$lookup operations.",
    "method": "POST",
    "path": "",
    "query": [],
    "body": {
      "resourceType": "Bundle",
      "type": "batch",
      "entry": [
        {
          "request": {
            "method": "POST",
            "url": "CodeSystem/$lookup"
          },
          "resource": {
            "resourceType": "Parameters",
            "parameter": [
              { "name": "system", "valueUri": "http://loinc.org" },
              { "name": "code", "valueCode": "23245-4" }
            ]
          }
        },
        {
          "request": {
            "method": "POST",
            "url": "CodeSystem/$lookup"
          },
          "resource": {
            "resourceType": "Parameters",
            "parameter": [
              { "name": "system", "valueUri": "http://snomed.info/sct" },
              { "name": "code", "valueCode": "17311000168105" }
            ]
          }
        }
      ]
    },
    "destructive": false
  },
  {
    "name": "Partial hierarchy",
    "description": "Request a partial hierarchy around a set of SNOMED codes.",
    "method": "POST",
    "path": "partial-hierarchy",
    "query": [],
    "body": {
      "system": "http://snomed.info/sct",
      "includeTerms": true,
      "codes": ["84410009", "386617003"]
    },
    "destructive": false
  },
  {
    "name": "Capability Statement",
    "description": "Fetch the server capability statement.",
    "method": "GET",
    "path": "metadata",
    "query": [],
    "body": null,
    "destructive": false
  },
  {
    "name": "Terminology Capabilities",
    "description": "Fetch the terminology-focused metadata view.",
    "method": "GET",
    "path": "metadata",
    "query": [
      { "key": "mode", "value": "terminology" }
    ],
    "body": null,
    "destructive": false
  }
]"##;

const ADMIN_OPERATIONS: &str = r##"[
  {
    "name": "Create ValueSet",
    "description": "Create a simple extensional ValueSet with generated id.",
    "method": "POST",
    "path": "ValueSet",
    "query": [],
    "body": {
      "resourceType": "ValueSet",
      "url": "http://example.com/fhir/vs/gender",
      "version": "0.1",
      "title": "Gender",
      "name": "gender",
      "status": "draft",
      "experimental": true,
      "compose": {
        "include": [
          {
            "system": "http://snomed.info/sct",
            "concept": [
              { "code": "248153007" },
              { "code": "248152002" },
              { "code": "407377005" },
              { "code": "407376001" }
            ]
          }
        ]
      }
    },
    "destructive": true
  },
  {
    "name": "Create ValueSet with specific id",
    "description": "Create or overwrite a ValueSet at a known id using PUT.",
    "method": "PUT",
    "path": "ValueSet/sex",
    "query": [],
    "body": {
      "resourceType": "ValueSet",
      "id": "sex",
      "url": "http://example.com/fhir/vs/sex",
      "version": "0.1",
      "name": "Sex",
      "status": "draft",
      "experimental": true,
      "compose": {
        "include": [
          {
            "system": "http://snomed.info/sct",
            "concept": [
              { "code": "248153007" },
              { "code": "248152002" },
              { "code": "407377005" },
              { "code": "407376001" }
            ]
          }
        ]
      }
    },
    "destructive": true
  },
  {
    "name": "Create intensional ValueSet",
    "description": "Create a ValueSet driven by a SNOMED constraint filter.",
    "method": "POST",
    "path": "ValueSet",
    "query": [],
    "body": {
      "resourceType": "ValueSet",
      "url": "http://example.com/value-sets/chronic-diseases",
      "version": "1.0.1",
      "status": "draft",
      "title": "Chronic Diseases",
      "name": "chronic-diseases",
      "experimental": true,
      "compose": {
        "include": [
          {
            "system": "http://snomed.info/sct",
            "filter": [
              {
                "property": "constraint",
                "op": "=",
                "value": "<< 27624003"
              }
            ]
          }
        ]
      }
    },
    "destructive": true
  },
  {
    "name": "Create ValueSet with excludes",
    "description": "Create an intensional ValueSet that excludes explicit concept members.",
    "method": "POST",
    "path": "ValueSet",
    "query": [],
    "body": {
      "resourceType": "ValueSet",
      "url": "http://example.com/ValueSet/education-levels",
      "version": "1",
      "title": "Education Levels",
      "status": "draft",
      "experimental": true,
      "compose": {
        "include": [
          {
            "system": "http://snomed.info/sct",
            "filter": [
              {
                "property": "concept",
                "op": "is-a",
                "value": "365460000"
              }
            ]
          }
        ],
        "exclude": [
          {
            "system": "http://snomed.info/sct",
            "concept": [
              { "code": "224298008" },
              { "code": "365460000" }
            ]
          }
        ]
      }
    },
    "destructive": true
  },
  {
    "name": "Update ValueSet",
    "description": "Update an existing ValueSet with a revised concept list.",
    "method": "PUT",
    "path": "ValueSet/sex",
    "query": [],
    "body": {
      "resourceType": "ValueSet",
      "id": "sex",
      "url": "http://example.com/fhir/vs/sex",
      "version": "0.1",
      "name": "Sex",
      "status": "draft",
      "experimental": true,
      "compose": {
        "include": [
          {
            "system": "http://snomed.info/sct",
            "concept": [
              { "code": "248153007" },
              { "code": "248152002" },
              { "code": "32570681000036106" },
              { "code": "407377005" },
              { "code": "407376001" }
            ]
          }
        ]
      }
    },
    "destructive": true
  },
  {
    "name": "Delete ValueSet by URL and version",
    "description": "Delete all ValueSets that match a canonical URL and version pair.",
    "method": "DELETE",
    "path": "ValueSet",
    "query": [
      { "key": "url", "value": "http://example.com/fhir/vs/sex" },
      { "key": "version", "value": "0.1" }
    ],
    "body": null,
    "destructive": true
  },
  {
    "name": "Delete ValueSet by id",
    "description": "Delete a specific ValueSet resource by id.",
    "method": "DELETE",
    "path": "ValueSet/sex",
    "query": [],
    "body": null,
    "destructive": true
  }
]"##;

fn render_page(
    page_title: &str,
    heading: &str,
    subtitle: &str,
    body: &str,
    script: &str,
    current_nav: &str,
) -> String {
    PAGE_TEMPLATE
        .replace("__PAGE_TITLE__", page_title)
        .replace("__STYLES__", SHARED_STYLES)
        .replace("__NAV__", &render_nav(current_nav))
        .replace("__HEADING__", heading)
        .replace("__SUBTITLE__", subtitle)
        .replace("__BODY__", body)
        .replace("__SCRIPT__", script)
}

fn render_nav(current_nav: &str) -> String {
    [
        ("/", "Browser", "browser"),
        ("/codesystems", "Code Systems", "codesystems"),
        ("/valuesets", "ValueSets", "valuesets"),
        ("/mapping", "Mapping and Tools", "mapping"),
        ("/admin", "Admin", "admin"),
    ]
    .into_iter()
    .map(|(href, label, key)| {
        let class_name = if key == current_nav {
            "nav-link active"
        } else {
            "nav-link"
        };

        format!(r#"<a class="{class_name}" href="{href}">{label}</a>"#)
    })
    .collect::<Vec<_>>()
    .join("")
}

fn tool_script(operations: &str) -> String {
    TOOL_SCRIPT_TEMPLATE.replace("__OPERATIONS__", operations)
}

fn browser_script(inactive_filtered: bool) -> String {
    BROWSER_SCRIPT_TEMPLATE.replace(
        "__INACTIVE_FILTERED__",
        if inactive_filtered { "true" } else { "false" },
    )
}

fn tool_page(
    page_title: &str,
    heading: &str,
    subtitle: &str,
    current_nav: &str,
    operations: &str,
) -> String {
    render_page(
        page_title,
        heading,
        subtitle,
        TOOL_BODY,
        &tool_script(operations),
        current_nav,
    )
}

pub fn browser_page(inactive_filtered: bool) -> String {
    render_page(
        "SNOMED Browser",
        "SNOMED Browser",
        "Search SNOMED CT International terms using Snowstorm Lite.",
        BROWSER_BODY,
        &browser_script(inactive_filtered),
        "browser",
    )
}

pub fn codesystems_page() -> String {
    tool_page(
        "Snowstorm Lite Code Systems",
        "Code Systems",
        "Browse CodeSystem resources, inspect concept details, and compare concept hierarchy relationships.",
        "codesystems",
        CODESYSTEM_OPERATIONS,
    )
}

pub fn valuesets_page() -> String {
    tool_page(
        "Snowstorm Lite ValueSets",
        "ValueSet Explorer",
        "Expand named and implicit ValueSets, run ECL-backed searches, and validate codes.",
        "valuesets",
        VALUESET_OPERATIONS,
    )
}

pub fn mapping_page() -> String {
    tool_page(
        "Snowstorm Lite Mapping and Tools",
        "Mapping and Tools",
        "Run translation operations, batch lookups, partial hierarchy utilities, and inspect the server capability metadata.",
        "mapping",
        MAPPING_OPERATIONS,
    )
}

pub fn admin_page() -> String {
    tool_page(
        "Snowstorm Lite Admin",
        "Protected ValueSet Admin",
        "Create, update, and delete ValueSets from the browser. Each write action is password-protected.",
        "admin",
        ADMIN_OPERATIONS,
    )
}
