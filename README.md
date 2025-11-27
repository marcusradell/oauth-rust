# Lab Rust IAM

An implementation of OAuth 2 to learn more about how it works and internet security.

The app consist of an Authentication Server and a Client.

The app aims to be as usable as possible while the OAuth functionality also works.
The tests might be failing though as the app gets rebuilt.

## Setup

`cp .env.template .env` and edit the file with relevant values for you.
_You can skip the `ANTHROPIC_API_KEY` if you aren't using Aider._

`docker compose up` to setup the database.

## Run

`bacon` or `cargo run` to run the application.

Go to `http://localhost:3000` to follow along the authorization flow.

### Dev DB

`(source .env && psql "$DATABASE_URL")` to connect to the database.
