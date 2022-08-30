# Containerize your existing application

## Availability

See available drivers, images etc.

### Drivers

- JS
- PHP

### Images

- Redis
- Mongo
- Elasticsearch
- MySQL
- PostgreSQL

### OS

- Ubuntu

## Run

```
cargo run -- --driver=php --path=examples/test-php-app
cargo run -- --driver=js --path=examples/test-js-app
cargo run -- --driver=js --path=examples/test-js-app --no-compose
cargo run -- --driver=js --path=examples/test-js-app --no-ignore
```

## Todo

- [ ] Separation of docker-compose to prod, dev and test environments.
- [ ] Generation of Kubernetes configuration files (deployments, persistent
      volumes etc.)
- [ ] Make Alpine OS available.
- [ ] Generation of CI files

## Refactor Todo

- [ ] Maybe separate some parts of `collect` function to pieces. Not important for
      now but it will grow big.
- [ ] Check if it makes sense to store `Vec<Image>` for images instead of
      `Vec<String>`
