FROM rust:1.67 as builder
WORKDIR /usr/src/tldr
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/tldr-line-verifier /usr/local/bin/tldr-line-verifier
CMD ["tldr-line-verifier", "/home"]
