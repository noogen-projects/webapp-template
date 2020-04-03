# webapp-template
Cargo-generate template for web application

Setup dependencies and tools:
```
sudo apt install mariadb-server libmysqlclient20 libmysqlclient-dev libssl-dev
cargo install diesel_cli --no-default-features --features mysql
cargo install wasm-bindgen-cli
```

Create database and apply migrations:
```
sudo mysql -u root < db/create_db.sql
diesel migration run
```

Create test database and apply migrations:
```
sudo mysql -u root < db/create_test_db.sql
./diesel_test.sh migration run
```

Add user to DB via REST API:
```
curl --header "Content-Type: application/json" --request POST --data '"Vasya"' http://localhost:8080/user
```