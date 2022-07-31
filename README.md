## PG setup
`sudo apt update`
`sudo apt install postgresql postgresql-contrib`
`sudo apt install libpq-dev`
verify the install worked using `psql --version`
now you have to do some tinkering. set the permissions to trust IPv4 connections.
`sudo chown [your username] /etc/postgresql/12/main/pg_hba.conf` 
set METHOD on line 96 to `trust`
`sudo service postgresql start`
should be good to go

## Install Rust and Cargo
if you already have rust you can skip this
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Local setup
`cargo install diesel_cli --no-default-features --features postgres`
`echo DATABASE_URL=postgres://postgres:password@localhost/diesel_demo > .env`
`diesel setup`
`sudo apt-get install jq`

## Run Server
`cargo run --bin rusty_chat`

## Create a new user
`cargo run --bin write_user`

## Delete a user
`cargo run --bin delete_user username`