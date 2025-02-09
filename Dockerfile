FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

COPY ./ ./

RUN cargo build --release

EXPOSE 8000/tcp

CMD ["./target/release/hello_cargo"]
