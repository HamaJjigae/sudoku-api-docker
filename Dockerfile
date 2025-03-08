FROM rust:1.84 as builder

WORKDIR /usr/src/sudoku-api

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo build --release

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/sudoku-api/target/release/sudoku-api /usr/local/bin/sudoku-api

ENV RUST_LOG=info

EXPOSE 8080

CMD ["sudoku-api"]
