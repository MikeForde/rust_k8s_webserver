FROM rust:stable

COPY ./ ./

RUN cargo build --release

EXPOSE 8000/tcp

CMD ["./target/release/hello_cargo"]
