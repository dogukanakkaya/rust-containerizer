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

### OS

- Ubuntu

## Run

```
cargo run -- --driver=php --path=examples/test-php-app
cargo run -- --driver=js --path=examples/test-js-app
cargo run -- --driver=js --path=examples/test-js-app --compose=false
```

## Todo

- [x] Add several available drivers (JS, PHP)
- [x] Add several available images (Redis, Mongo, Elasticsearch)
- [x] Add docker-compose generation option
- [ ] Separation of docker-compose to prod, dev and test environments.
- [ ] Generation of .dockerignore file
- [ ] Make more image available, mostly databases (PgSQL, MySQL etc.)
- [ ] Generation of Kubernetes configuration files (deployments, persistent
      volumes etc.)
- [ ] Make Alpine OS available.
- [ ] Research for how to collect more information about definitions (check
      .env, language's or framework's configuration files etc.)
