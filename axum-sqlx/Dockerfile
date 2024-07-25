FROM rust:1.79-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev pkg-config

COPY . .

RUN cargo build --release

FROM ubuntu:24.10 AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev pkg-config

COPY --from=builder /app/target/release/plann-er .
COPY dev.db .

EXPOSE 3333

CMD ["./plann-er"]
