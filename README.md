# Personal Cloud
---
Personal Cloud Storage application written in Rust + React

Much thanks to [Rust Web Starter](https://github.com/ghotiphud/rust-web-starter) for doing a lot of the dirty work.

## Installation
---
There are a few requirements and steps to get the cloud running:

Requirements:
1. Install [Docker](https://www.docker.com/community-edition)
2. Install [Docker-Compose](https://docs.docker.com/compose/install/)
    - If you are on Windows, you won't need this step as Docker comes with Compose.
3. A knowledge of Rust and/or PostgreSQL as we will not provide a default user.
    - Logging in will require you to make a user without the application: the easiest way to do this is to update the login function to insert a new user as an `admin`.
    - This application is targeted for a friend of mine, not as a scalable website; a sign-up page is not likely to happen, at-least not in the near future.

Setup:
1. Copy `docker-compose.yml.example` into `docker-compose.yml`. Update as needed.
2. Copy `/api_server/.env.example` into `/api_server/.env`. Update as needed.
3. Run `docker-compose up`
    - If your migrations do not automatically run, then:
        1. In another terminal run `docker-compose exec api_server bash`
        2. In the shell, run `diesel migration run`
4. Known issues:
    - Windows doesn't behave nicely initially, and starting the drive `api_server_x` will fail. To fix this, just change `api_server/wait-for-it.sh` to Unix line endings.
    - PostgreSQL won't startup the first time; this is just part of it's install. `Ctrl+C` out and run `docker-compose up` again.
