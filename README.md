# json-query

A CLI tool written in Rust for querying JSON arrays from the command line.

## Usage

```
cargo run -- -f <file> -q <key> [-v <value>]
```

### Flags

| Flag | Description |
|------|-------------|
| `-f` | Path to the JSON file |
| `-q` | Key to query (supports dot notation for nested keys) |
| `-v` | (Optional) Filter — only return objects where the queried key equals this value |

## Examples

**Get all values for a key across every object in the array:**

```sh
cargo run -- -f users.json -q username
```

Output: all `username` values from every object.

**Filter objects by key value:**

```sh
cargo run -- -f users.json -q username -v amira_hassan
```

Output: the full object where `username` equals `amira_hassan`.

**Query nested keys using dot notation:**

```sh
cargo run -- -f users.json -q profile.first_name
cargo run -- -f users.json -q account.role -v admin
```

## JSON format

The input file must be a JSON array of objects:

```json
[
  { "id": 1, "username": "jsmith92", "profile": { "first_name": "John" } },
  { "id": 2, "username": "amira_hassan", "profile": { "first_name": "Amira" } }
]
```

## Building

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
cargo build
```