# ---------- Build stage ----------
FROM rust:1.78 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
# ---------- Runtime stage ----------


FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/appent_data_api .
COPY config.yml ./config.yml
COPY src/data ./src/data
EXPOSE 9988
CMD ["./appent_data_api"]
