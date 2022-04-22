FROM rust:1-slim-bullseye AS builder
WORKDIR /code

RUN USER=root cargo init
COPY Cargo.toml Cargo.toml
RUN cargo fetch
COPY src src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11
WORKDIR /app

COPY --from=builder /code/target/release/config-manager config-manager

USER 1001

EXPOSE 3000

CMD [ "/app/config-manager" ]