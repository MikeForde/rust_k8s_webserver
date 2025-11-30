FROM registry.access.redhat.com/ubi8/ubi-minimal:latest

WORKDIR /app
COPY target/release/hello_cargo ./hello_cargo

EXPOSE 8000
CMD ["./hello_cargo"]
