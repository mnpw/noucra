#!/usr/bin/env bash

# print trace of simple commands
set -x

# exit if command pipeline fails with the return value
# of last command to exit with non-zero status
set -eo pipefail

#+++++++++++++++++++++++++++++++++++++++++

# -x: check if executable
# [ -x "$(command -v <command>"] translates to
# test -x "..." which means
# run "$(...)" and test if it is an executable
if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx not installed."
    exit 1
fi

#+++++++++++++++++++++++++++++++++++++++++

# define db config variables
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
DB_NAME=${POSTGRES_NAME:=newsletter}
DB_PORT=${POSTGRES_PORT:=5432}

# launch postgres in docker
# -e: set env variables
# -p: [host port]:[container port] mapping
# -N: max connections
if [[-z "${SKIP_DOCKER}"]]; then
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_NAME=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000
fi

# ping postgres and proceed if it is alive and ready
# export password to be used by `psql`
export PGPASSWORD="${DB_PASSWORD}"
# -d "postgres": connect to default supplied db
# -c "\q": run exit command when connected
until psql -h "localhost" -p "${DB_PORT}" -U "${DB_USER}" -d "postgres" -c "\q"; do
    echo >&2 "Postgres is unavailable - sleeping."
    sleep 1
done

# >&2 directs to STDERR
# format for redirection is [source]>[destination]
# & signifies a file descriptor
echo >&2 "Postgres is running on port ${DB_PORT}."

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run

echo >&2 "Postgres has been migrated."
