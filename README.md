# SNOMED Browser

This project is a small Rust web application that provides a browser for SNOMED CT concepts using a `snowstorm-lite` FHIR endpoint.

It serves a simple HTML UI that lets you:

- search SNOMED CT terms
- inspect concept details for a selected code
- proxy requests to `snowstorm-lite` from the browser through the Rust server

The server is built with `actix-web` and uses `reqwest` to call the backing FHIR API.

## How It Works

The application exposes three routes:

- `/` renders the browser UI
- `/api/search?q=<term>` calls FHIR `ValueSet/$expand` against `snowstorm-lite`
- `/api/lookup/<code>` calls FHIR `CodeSystem/$lookup` for a SNOMED CT concept

Search results return concept code and display text. Selecting a concept loads additional fields where available, including:

- display
- code
- fully specified name (FSN)
- version
- inactive flag
- module ID
- effective time

## Configuration

The application uses the `SNOWSTORM_BASE` environment variable to locate the backing FHIR service.

Example:

```bash
export SNOWSTORM_BASE="https://your-snowstorm-lite-host/fhir"
```

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

The application currently targets the following FHIR operations:

- `ValueSet/$expand?url=http://snomed.info/sct?fhir_vs&filter=<term>`
- `CodeSystem/$lookup?system=http://snomed.info/sct&code=<code>`

This keeps the browser UI simple while relying on `snowstorm-lite` for terminology search and concept lookup behaviour.

## OpenShift / Prebuilt Binary Approach

This repository still uses the earlier prebuilt-binary deployment approach for container usage and OpenShift experiments:

- build the Rust binary ahead of time
- copy the binary into the runtime image
- avoid compiling in-cluster

That approach reduces deployment-time compilation and avoids depending on crate downloads during image build in restricted environments.
