all: generate update-schema

generate:
    cargo run -p generate | rustfmt > src/generated.rs

update-schema:
    cargo run --bin update-schema --features update-schema
