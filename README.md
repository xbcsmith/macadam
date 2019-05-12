# macadam

## Setup

### Get Rust

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    rustup override set nightly

### Get Diesel

    cargo install --no-default-features --features sqlite diesel_cli

### Setup DB

    echo "DATABASE_URL=receipts.db" > .env

    diesel migration run

## Run bins

    cargo run --bin list_receipts

    cargo run --bin create_receipt

    cargo run --bin issue_receipt 1

    cargo run --bin list_receipts

    cargo run --bin revoke_receipt foo

    cargo run --bin list_receipts

