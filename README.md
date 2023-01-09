# axum-graphql

## docker compose

```
$ docker compose up -d
```

## migrate

```
$ cargo install sea-orm-cli
$ sea-orm-cli migrate up
```

## generate entities

```
$ sea-orm-cli generate entity -u mysql://user:password@localhost:3306/db -o src/entities
```

## run

```
$ cargo run
```

## run watch

install cargo-watch

```
$ cargo install cargo-watch
```

run

```
$ cargo watch -x run
```
