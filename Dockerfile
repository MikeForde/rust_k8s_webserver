FROM registry.access.redhat.com/ubi9/ubi-minimal

RUN microdnf install -y ca-certificates \
    && microdnf clean all

WORKDIR /app
COPY target/release/hello_cargo ./hello_cargo

EXPOSE 8000
CMD ["./hello_cargo"]
