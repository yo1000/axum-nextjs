FROM rust:1.95 as builder

ARG SRC

RUN apt-get update \
    && apt-get install -y --no-install-recommends musl-tools \
    && rm -rf /var/lib/apt/lists/*

COPY $SRC /opt/app
WORKDIR /opt/app

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM debian:bookworm-slim as runner

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /opt/app/target/x86_64-unknown-linux-musl/release/api /usr/local/bin/eight-api

CMD ["eight-api"]
