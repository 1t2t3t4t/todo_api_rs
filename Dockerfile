FROM rust:1.51-slim AS build_deps

WORKDIR /app

COPY Cargo.lock .
COPY Cargo.toml .

RUN mkdir src
RUN touch src/main.rs
RUN echo "fn main() {}" >> src/main.rs

RUN cargo build --release

COPY . .

RUN cargo build --release

FROM rust:1.51-slim AS runtime

WORKDIR /app

COPY --from=build_deps /app/target/release/todo_api_rs .

ENTRYPOINT ["./todo_api_rs"]