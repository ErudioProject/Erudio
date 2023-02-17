FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

#RUN rustup target add x86_64-unknown-linux-musl
RUN rustup component add clippy
RUN cargo search
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./apps/erudio/backend-prisma-cli/ ./apps/erudio/backend-prisma-cli/
#RUN cp -r .cargo-tmp/* ~/.cargo | true
#RUN cp -r .cargo-tmp/.* ~/.cargo | true
WORKDIR apps/erudio/backend-prisma-cli/
RUN cargo run --release --bin prisma_cli -- generate
RUN cargo run --release --bin prisma_cli -- migrate deploy
RUN mkdir -p ../../../../shared-libs/rust/prisma-client/src/
WORKDIR ../../../
COPY . .
RUN cargo build --release --bin api


FROM debian:bullseye-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/api /usr/local/bin
ENTRYPOINT ["/usr/local/bin/api"]