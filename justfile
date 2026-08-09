set dotenv-load := true

codegen:
    cargo test export_bindings
    ./codegen.sh

dev:
    cargo watch -x run

uidev:
    cd ui && bun run dev

create-migration name:
    sqlx migrate add {{name}}

reset-db:
    sqlx database reset

build:
    cross build --release --target=x86_64-unknown-linux-gnu
    cd ui && bun vite build

seed:
    cargo run -p seed

create-release version:
    ./update_version {{version}}
    just build
    plugin-cli package
