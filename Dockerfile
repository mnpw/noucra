FROM lukemathwalker/cargo-chef:latest-rust-1.61.0 as chef
WORKDIR /app
RUN apt update -y && apt install lld clang -y

##️⃣ stage 0: identify and prepare project dependency requirements 
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json 

##️⃣ stage 1: build project depencies and then project
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# build just the project dependencies
RUN cargo chef cook --release --recipe-path recipe.json
# FROM, COPY and RUN commands above will be cached not re-run if recipe.json
# does not change
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin noucra

##️⃣ stage 2: prepare runtime environment and run app
# rust toolchain is not required to run the built binary
FROM debian:bullseye-slim AS runtime
RUN apt-get update -y \
    # openssl is dynamically linked to some rust libs
    # ca-certificate is required to verify TLS certificates
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # remove packages that were automatically installed to satisfy dependencies for
    # other packages and are now no longer needed
    && apt-get autoremove -y \
    # clears out the local repository of retrieved package files
    && apt-get clean -y \
    # /var/lib/apt/lists/ is storage area for state information for each package
    # resource specified in sources.list(5) 
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/noucra noucra
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./noucra"]