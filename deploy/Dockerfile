FROM rust:1.95-slim as builder
WORKDIR /build
COPY . .
RUN cargo build --release -p prover-server

FROM debian:bookworm-slim
RUN useradd -r -s /bin/false tscp
COPY --from=builder /build/target/release/prover-server /usr/local/bin/
USER tscp
EXPOSE 3030
CMD ["prover-server"]
