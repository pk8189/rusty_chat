## Local setup

`cargo install diesel_cli --no-default-features --features postgres`

`echo DATABASE_URL=postgres://postgres:password@localhost/diesel_demo > .env`
`diesel setup`

## Create a new user
`cargo run --bin write_user`

## Delete a user
`cargo run --bin delete_user username`