# rust-perfect-endpoints

⚠️ Disclaimer: the endpoints are not actually **perfect**, they just demonstrate the structure that is perfectly legible and maintanable for me.

### How to run
```bash
docker-compose up -d
cargo run
```

#### If you want to run in full performance mode (genuinely makes a difference)
```bash
docker-compose up -d
cargo run --release
```

## What is being used here:

#### App-wise:
- Rocket (for the server, with tokio as runtime)
- SeaORM/SQLX (for persistence and querying)
- Reqwest (for HTTP requests)

#### Project-wise:
- PostgreSQL
- Node Red (for request mocking)
