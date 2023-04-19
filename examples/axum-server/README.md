# Example

## In memory

1. Run `cargo run --features memory`

## Postgres

1. Apply the migration by running the query or using the `sqlx` cli.

2. Run the docker compose `docker compose up -d`

3. Run `cargo run --features postgres`.