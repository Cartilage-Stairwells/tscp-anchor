FROM rust:1.75-slim as builder
WORKDIR /build
COPY . .
RUN cd crates/oracle-layer && cargo build --release --bin prover-server

FROM debian:bookworm-slim
RUN useradd -r -s /bin/false tscp
COPY --from=builder /build/crates/oracle-layer/target/release/prover-server /usr/local/bin/
USER tscp
EXPOSE 3030
CMD ["prover-server"]
