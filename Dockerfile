FROM rust:1-slim-bullseye AS builder
WORKDIR /code

RUN USER=root cargo init && apt update -y && apt install -y clang
COPY Cargo.toml Cargo.toml
RUN cargo fetch
COPY src src
RUN cargo build --release
RUN mkdir /data

FROM gcr.io/distroless/cc-debian11
WORKDIR /app
COPY --from=builder /code/target/release/config-manager config-manager
COPY --from=builder /data /data
EXPOSE 3000
VOLUME /data
CMD [ "/app/config-manager", "-d", "/data" ]