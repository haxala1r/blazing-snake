FROM rust
WORKDIR /work

COPY src ./src
COPY Cargo.toml Cargo.lock ./

RUN cargo build

CMD ["cargo", "run"]