# eliasfl-hash

The database is serialized in JSON format

## How to run

Se all commands and syntax

```sh
cargo run -- help
```

Insert a value with key `123`

```sh
cargo run -- --file database.json insert 123 "test value"
```

Get that value

```sh
cargo run -- --file database.json get 123
```

Remove that value

```sh
cargo run -- --file database.json remove 123
```
