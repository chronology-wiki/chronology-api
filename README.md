# chronology-api

## Local development

Install the nightly build of rust and docker-compose.

```sh
# Terminal 1
docker-compose up

# Terminal 2
# One-time setup if you don't have cargo watch
cargo install cargo-watch

# Start up the web server
cargo watch -x run
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
diesel migrations run
```