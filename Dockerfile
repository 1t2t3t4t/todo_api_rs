FROM rust:1.51-slim

WORKDIR /app

COPY Cargo.lock .
COPY Cargo.toml .

RUN mkdir src
RUN touch src/main.rs
RUN echo "fn main() {}" >> src/main.rs

RUN cargo build --release

COPY . .

RUN cargo build --release

ENTRYPOINT ["cargo", "run", "--release"]