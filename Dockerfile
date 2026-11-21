FROM rust:1-slim-bookworm AS build
WORKDIR /app
COPY . .
RUN cargo build --release -p essential-app

FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /app/target/release/essential /usr/local/bin/essential
ENTRYPOINT ["/usr/local/bin/essential"]
