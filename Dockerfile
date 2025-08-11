FROM rust:1 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=build-env /app/target/release/ads-b-2-discord /
CMD ["/ads-b-2-discord"]