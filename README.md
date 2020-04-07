# webapp-template

Cargo-generate template for web application

## Development notes

Setup dependencies and tools:

```shell script
sudo apt install mariadb-server libmysqlclient20 libmysqlclient-dev libssl-dev
cargo install diesel_cli --no-default-features --features mysql
cargo install wasm-bindgen-cli
```

Create database and apply migrations:

```shell script
sudo mysql -u root < db/create_db.sql
diesel migration run
```

Create test database and apply migrations:

```shell script
sudo mysql -u root < db/create_test_db.sql
./diesel_test.sh migration run
```

Build and run:

```shell script
./build_webclient.sh
cargo run -p webapp-server
```

Add user via REST API:

```shell script
curl --header "Content-Type: application/json" --request POST --data '"Vasya"' http://localhost:8080/user
```

Check the project:

```shell script
cargo check --all-features --all-targets
```

Run all tests:

```shell script
cargo test --all-features --all-targets
```

Check and perform formatting:

```shell script
cargo +nightly fmt -- --check
cargo +nightly fmt
```