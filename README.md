# SNOMED Browser

This project is a Rust web application that provides a browser for SNOMED CT concepts using a `snowstorm-lite` FHIR endpoint.

It serves a HTML UI that lets you:

- search SNOMED CT terms
- inspect concept details for a selected code
- proxy requests to `snowstorm-lite` from the browser through the Rust server
- work across additional pages for CodeSystem, ValueSet, translation, metadata, batch, and admin operations from the supplied Postman collection

The server is built with `actix-web` and uses `reqwest` to call the backing FHIR API.

## Pages

The application includes these pages:

- `/` simple SNOMED browser for search and concept drill-down
- `/codesystems` list CodeSystems, run `$lookup`, and test `$subsumes`
- `/valuesets` list ValueSets, run `$expand`, ECL, and `$validate-code`
- `/mapping` run `$translate`, batch lookup, partial hierarchy, and metadata operations
- `/admin` create, update, and delete ValueSets behind password protection

## How It Works

The application exposes these main routes:

- `/`, `/codesystems`, `/valuesets`, `/mapping`, `/admin` render HTML pages
- `/api/search?q=<term>` calls FHIR `ValueSet/$expand` against `snowstorm-lite`
- `/api/lookup/<code>` calls FHIR `CodeSystem/$lookup` for a SNOMED CT concept
- `/api/fhir` forwards validated FHIR requests from the additional pages to the configured Snowstorm Lite base URL

Search results return concept code and display text. Selecting a concept loads additional fields where available, including:

- display
- code
- fully specified name (FSN)
- version
- inactive flag
- module ID
- effective time

## Configuration

Uses local `.env` file locally or OpenShift equivalent.

Required settings for the expanded UI:

- `SNOWSTORM_BASE` points at the backing FHIR endpoint
- `APP_ADMIN_PASSWORD` protects destructive UI actions on the `/admin` page
- `SNOWSTORM_ADMIN_USERNAME` and `SNOWSTORM_ADMIN_PASSWORD` are used by the server when forwarding protected ValueSet write requests upstream

Example:

```bash
cp .env.example .env
```

The checked-in `.env` contains generated secrets for local development. If your Snowstorm Lite instance uses a different admin password, update `SNOWSTORM_ADMIN_PASSWORD` in `.env` to match it.

If `SNOWSTORM_BASE` is not set, the app falls back to the default configured in `src/main.rs`.

## Run Locally

Start the development server with Cargo:

```bash
cargo run
```

The app listens on:

```text
http://0.0.0.0:8080
```

Open `http://localhost:8080` in a browser.

## Build

Create a release binary with:

```bash
cargo build --release
```

The compiled binary is written to:

```text
target/release/hello_cargo
```

## Container Image

The `Dockerfile` is set up to copy in a prebuilt release binary rather than compiling inside the image.

Typical flow:

```bash
cargo build --release
docker build -t snomed-browser .
docker run -p 8080:8080 -e SNOWSTORM_BASE="https://your-snowstorm-lite-host/fhir" snomed-browser
```

## API Notes

The application covers these operations from the Snowstorm Lite Postman collection:

- `CodeSystem` search and filtered lookup
- `CodeSystem/$lookup`
- `CodeSystem/$subsumes`
- `ValueSet`
- `ValueSet/$expand`
- `ValueSet/$validate-code`
- `ConceptMap/$translate`
- root-level FHIR batch requests
- `partial-hierarchy`
- `metadata` and `metadata?mode=terminology`

Destructive ValueSet create, update, and delete requests are blocked unless:

- the user supplies the correct `APP_ADMIN_PASSWORD`
- the server has upstream Snowstorm admin credentials configured

This keeps the default browser page simple while exposing the wider Snowstorm Lite API surface on dedicated pages.

## OpenShift / Prebuilt Binary Approach

Repository uses prebuilt-binary deployment approach for container usage and OpenShift experiments:

- build the Rust binary ahead of time
- copy the binary into the runtime image
- avoid compiling in-cluster

This approach reduces deployment-time compilation and avoids depending on crate downloads during image build in restricted environments.
