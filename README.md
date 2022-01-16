# chronology-api

## Local development

cargo-watch.

```sh
# Terminal 1
docker-compose up

# Terminal 2
# One-time setup if you don't have cargo watch
cargo install cargo-watch

# Start up the web server
cargo watch -x run

open http://localhost:8000
```

## Connecting to the database

```sh
docker-compose exec db bash
su postgres
psql
```

## Running db migrations

Install diesel CLI

```sh
cargo install diesel_cli
```

```sh
diesel migrations run
```