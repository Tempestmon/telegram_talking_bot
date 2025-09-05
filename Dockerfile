FROM blackdex/rust-musl:x86_64-musl-stable-1.89.0 AS builder

WORKDIR /app

COPY ./Cargo.toml /app/
COPY ./src /app/src

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN ls -la /app/target/x86_64-unknown-linux-musl/release/

FROM alpine:3.18

RUN apk add --no-cache ca-certificates
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/mopsorez_telegram_bot /usr/local/bin/mopsorez_telegram_bot

CMD ["mopsorez_telegram_bot"]
