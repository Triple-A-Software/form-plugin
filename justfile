set dotenv-load := true

codegen:
    cargo test export_bindings

dev:
    cargo watch -x run

create-migration name:
    sqlx migrate add {{name}}

reset-db:
    sqlx database reset

build:
    cargo build
    cd ui && bun vite build
