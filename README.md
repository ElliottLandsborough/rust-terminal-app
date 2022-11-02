# Rust Terminal App

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/ElliottLandsborough/rust-terminal-app/tree/main.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/ElliottLandsborough/rust-terminal-app/tree/main)

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