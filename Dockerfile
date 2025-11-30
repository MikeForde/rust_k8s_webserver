FROM debian:bullseye-slim

WORKDIR /app
COPY target/release/hello_cargo ./hello_cargo

EXPOSE 8000
CMD ["./hello_cargo"]
