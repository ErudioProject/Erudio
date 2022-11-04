FROM rustlang/rust:nightly AS builder


#RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev libc6-dev





WORKDIR /api
COPY ./Cargo.toml ./Cargo.toml
COPY ./apps ./apps
COPY ./libs ./libs
RUN cargo run --bin prisma_cli -- generate
RUN cargo build --package api --bin api --release

FROM debian:buster-slim

COPY --from=builder /code/target/release/api /api

CMD ["/api"]