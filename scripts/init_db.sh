#!/usr/bin/env bash

# print trace of simple commands
set -x

# exit if command pipeline fails with the return value 
# of last command to exit with non-zero status
set -eo pipefail

# define db config variables
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
DB_NAME=${POSTGRES_NAME:=newsletter}
DB_PORT=${POSTGRES_PORT:=5432}

# launch postgres in docker
# -e: set env variables
# -p: [host port]:[container port] mapping
# -N: max connections
docker run \
-e POSTGRES_USER=${DB_USER} \
-e POSTGRES_PASSWORD=${DB_PASSWORD} \
-e POSTGRES_NAME=${DB_NAME} \
-p "${DB_PORT}":5432 \
-d postgres \
postgres -N 1000
