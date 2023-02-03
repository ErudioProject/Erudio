cargo sweep -s
cargo build --package api --bin api
cargo build --package prisma_cli --bin prisma_cli
cargo clippy
cargo test
cargo sweep -f