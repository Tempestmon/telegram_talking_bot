FROM clux/muslrust:1.89.0-stable AS builder

WORKDIR /app

COPY ./Cargo.toml /app/
COPY ./src /app/src

RUN apk update && apk add --no-cache openssl-dev pkgconf musl-dev

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/mopsorez_bot /usr/local/bin/mopsorez_bot

CMD ["mopsorez_bot"]
