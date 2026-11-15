FROM rust:1-bookworm AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd --system app
COPY --from=build /src/target/release/pixle-rust /usr/local/bin/pixle-rust
USER app
EXPOSE 8082
ENTRYPOINT ["pixle-rust"]
