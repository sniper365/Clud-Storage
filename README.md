# Project

A simple storage system plus more for personal use.

## Setup:

### Requirements:

* Postgres Database
* Rust: [installation instructions](https://www.rust-lang.org/en-US/install.html)

### Setup:

1. Create a Database in Postgres

2. Setup the `.env` file. Setup instructions can be found below

3. Setup Rocket.toml (Optional) Setup instructions can be found [here](https://rocket.rs/guide/configuration/#rockettoml)

4. Create the directory `storage`. Configuring where to store is on my to-do list.

5. `Diesel` needs the schema to match the project before it starts. `diesel migration run` is the [easiest](http://diesel.rs/guides/getting-started/) way to do this.

### Startup:
Simply run the command `cargo run`

### Build Errors:
This project requires a package `Diesel` and `Rocket`. Before compiling, both `Diesel` and `Rocket` run other code to build dependencies.

This is important because if it cannot connect to the database, or it mismatches the database, it will look like a compiling error: thats because it is. When errors arise; the first place to look is your database.

### ENV Options:
- DATABASE_URL: Connection string to your database. Looks something like `postgres://postgres:secret@db/database`

- APP_KEY: Secret key that authentication will use to verify tokens

- APP_INDEX (Optional): If you don't want to serve up `index.html` or your file is named something else.

## TODO List:
- Prettier pre-installed frontend; I went with an idea that worked, doesn't mean I like it. Also doesn't mean I didn't make obvious design flaws.

- Configurable and multiple storage locations. All of the contents of the application should not need to be in one folder. This is a blatant violation of the DIP.

- Configurable build location. Should be able to specify where the build is; the `frontend` is another DIP violation.

- MySQL. Most people use MySQL, Postgres is just my personal favorite. Should be able to connect to MySQL database.

- Workers, specifically file compression. If there's a 4K upload, shouldn't give a 4K preview.

- WebSocket. I love automation; there's more to this that I intend to do. My friend's suggestion to build this was just a push to start something bigger.
