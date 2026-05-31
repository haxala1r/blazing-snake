FROM rust AS builder
WORKDIR /work

COPY src ./src
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release


FROM debian
COPY --from=builder /work/target/release/snake /work/snake

CMD ["/work/snake"]