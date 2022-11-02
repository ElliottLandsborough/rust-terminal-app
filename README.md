# Rust Terminal App

https://github.com/ElliottLandsborough/rust-terminal-app

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/ElliottLandsborough/rust-terminal-app/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/ElliottLandsborough/rust-terminal-app/tree/main)

## Example Output

```bash
Please input a github username below:
>> ElliottLandsborough
Fetching preferred languages for 'ElliottLandsborough' on github...
1. PHP
2. Go
3. JavaScript
4. Python
5. Rust
6. TypeScript
```

## How to run

With docker:

```bash
docker-compose up
```

In bash:

```bash
cargo run
```

Specify a github username inline:

```bash
echo "ElliottLandsborough" | rust-terminal-app
```

## Dependencies

- rust or docker
- `libssl-dev` on Ubuntu or `openssl-devel` on Fedora
