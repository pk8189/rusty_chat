## PG setup
`sudo apt update`
`sudo apt install postgresql postgresql-contrib`
`sudo apt install libpq-dev`
verify the install worked using `psql --version`
you might have to alter the pg_hba.conf file and reset the db get `diesel setup` to run below
/etc/postgresql/12/main/pg_hba.conf
set everything to 'trust'

## Local setup
`cargo install diesel_cli --no-default-features --features postgres`
`echo DATABASE_URL=postgres://postgres:password@localhost/diesel_demo > .env`
`diesel setup`


## Create a new user
`cargo run --bin write_user`

## Delete a user
`cargo run --bin delete_user username`