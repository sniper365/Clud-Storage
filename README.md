# Personal Cloud
---
Personal Cloud Storage application written in Rust + React

Much thanks to [Rust Web Starter](https://github.com/ghotiphud/rust-web-starter) for doing a lot of the dirty work.

## Installation
---
There are a few requirements and steps to get the cloud running:

Requirements:
1. PostgreSQL Database
2. A knowledge of Rust and/or PostgreSQL as we will not provide a default user.
    - Logging in will require you to make a user without the application: the easiest way to do this is to update the login function to insert a new user as an admin.

Setup:
1. Copy `docker-compose.yml.example` into `docker-compose.yml`. Update as needed.
2. Copy `/api_server/.env.example` into `/api_server/.env`. Update as needed.
3. Run `docker-compose up` and `docker-compose exec api_server bash` in two separate terminals
    - In the api_server terminal: run `diesel migration run`
