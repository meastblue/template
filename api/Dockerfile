FROM rust:slim-bullseye as builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev postgresql-client && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src
COPY wait-for-db.sh .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y libssl1.1 postgresql-client && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/api .
COPY --from=builder /app/wait-for-db.sh .

RUN chmod +x wait-for-db.sh

EXPOSE 3000

CMD ["./wait-for-db.sh", "./api"]