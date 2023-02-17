FROM clux/muslrust:stable AS chef
USER root
RUN apt update
RUN apt install wget
# RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
# RUN tar zxvf cargo-binstall-x86_64-unknown-linux-musl.tgz -C $HOME/.cargo/bin
# RUN cargo +nightly binstall --no-confirm cargo-chef
WORKDIR /app

# FROM chef AS planner
# COPY . .
# RUN cp .cargo-tmp ~/.cargo | true
# RUN cargo +nightly chef prepare --recipe-path recipe.json

FROM chef AS builder
#COPY --from=planner /app/recipe.json recipe.json

# I hate this
RUN wget http://archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.0g-2ubuntu4_amd64.deb
RUN dpkg -i libssl1.1_1.1.0g-2ubuntu4_amd64.deb
# And i heate it eaven more that this works
# Correcting misspeling is not allowed

RUN rustup target add x86_64-unknown-linux-musl
RUN rustup component add clippy

#RUN cargo +nightly chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
RUN cargo search
COPY . .
#RUN cp -r .cargo-tmp/* ~/.cargo | true
#RUN cp -r .cargo-tmp/.* ~/.cargo | true
WORKDIR apps/erudio/backend-prisma-cli/
RUN cargo run --target x86_64-unknown-linux-musl --bin prisma_cli -- generate # --relese don't work for some reason
WORKDIR ../../../
RUN cargo build --target x86_64-unknown-linux-musl --bin api


FROM alpine AS runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/target/x86_64-unknown-linux-musl/debug/api /usr/local/bin/
USER myuser
CMD ["/usr/local/bin/api"]