FROM instrumentisto/rust:nightly-slim

COPY ./ ./

RUN cargo build --release

EXPOSE 8000/tcp

CMD ["./target/release/hello_cargo"]
