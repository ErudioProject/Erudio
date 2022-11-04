FROM clux/muslrust:nightly AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Notice that we are specifying the --target flag!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .

# I hate this
RUN apt update
RUN apt install wget
RUN wget http://archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.0g-2ubuntu4_amd64.deb
RUN dpkg -i libssl1.1_1.1.0g-2ubuntu4_amd64.deb
# And i heate it eaven more that this works
# Correcting misspeling is not allowed

RUN rustup target add x86_64-unknown-linux-musl
WORKDIR apps/prisma-cli/
RUN cargo run --bin prisma_cli -- generate
WORKDIR ../../
RUN cargo build --release --target x86_64-unknown-linux-musl --bin api


FROM alpine AS runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api /usr/local/bin/
USER myuser
CMD ["/usr/local/bin/api"]