FROM rust:1.85 AS builder
WORKDIR /usr/src/tldr
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/tldr-line-verifier /usr/local/bin/tldr-line-verifier
CMD ["tldr-line-verifier", "/home"]
