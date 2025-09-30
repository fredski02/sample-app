# Sample App (Rust + Axum + Askama + SQLite + Kafka)


## Run Kafka
```bash
docker compose up -d
```

## Set env var
for sqlx compile errors to show set the following var
typically this needs to be a URI to this project location followed by sample.db
```bash
EXPORT DATABASE_UR="path-to-local.db" 
```

## Configure
```bash
cp .env.example .env
```

## Run
```bash
cargo run
```


Open http://localhost:3000



