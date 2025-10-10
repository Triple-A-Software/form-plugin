set dotenv-load := true

codegen:
    cargo test export_bindings

dev:
    cargo watch -x run

uidev:
    cd ui && bun run dev

create-migration name:
    sqlx migrate add {{name}}

reset-db:
    sqlx database reset

build:
    cargo build --release
    cd ui && bun vite build

seed:
    cargo run -p seed
