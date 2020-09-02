# chronology-api

## Local development

Install the nightly build of rust and docker-compose.

```sh
# Terminal 1
docker-compose up

# Terminal 2
cargo watch -x run
```

## Connecting to the database

```sh
docker-compose exec db bash
su postgres
psql
```