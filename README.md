# REST Todo API

A simple REST Todo Api in Rust with the [axum web framework](https://docs.rs/axum/latest/axum/). Created for learning purposes only.

## Run

```shell
# Start the database
$ docker compose up -d

# Build the app
$ cargo build

# Run migrations
$ sqlx migrate run

# Run the app
$ cargo run
```

### Migrations

```shell
# Create a new migration (-r for creating up and down files)
$ sqlx migrate add -r <name>

# Run migrations
$ sqlx migrate run
```
