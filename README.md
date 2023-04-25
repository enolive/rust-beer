# Beer App with Rust/SvelteKit

This repo contains a complete implementation of a beer CRUD API with a frontend.

The data is stored inside a [MongoDB](https://www.mongodb.com/).

## Contents

- [beer-kit](./beer-kit): The frontend, written with [SvelteKit](https://kit.svelte.dev/)
- [beer-service](./beer-service): The backend, written with [Actix Web/Rust](https://actix.rs/)
- data: just an empty directory for the database volume used in the `docker-compose` setup.

## Quickstart

Just run 

```bash
docker-compose up
```
be patient, this will take a couple of minutes!

- Open http://localhost:3000 in your browser for the frontend
- Open http://localhost:9080/swagger-ui/ in your browser for the API docs
