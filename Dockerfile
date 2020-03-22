# build stage
FROM rustlang/rust:nightly-slim as build

# install libpq and create new empty binary project
RUN apt-get update; \
    apt-get install -y --no-install-recommends libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin app
WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# build this project to cache dependencies
RUN cargo build --release; \
    rm src/*.rs

# copy project source and necessary files
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .

# add Rocket.toml and secret.key for Docker env and rebuild app with project source
RUN touch Rocket.toml; \
    printf "[global.databases]\npostgres_database = { url = \"postgres://postgres:postgres@db/postgres\" }\n" > Rocket.toml; \
    mv src/secret.key.sample src/secret.key; \
    rm ./target/release/deps/address_book_rest_api*; \
    cargo build --release

# deploy stage
FROM debian:buster-slim

# create app directory
RUN mkdir app
WORKDIR /app

# install libpq
RUN apt-get update; \
    apt-get install -y --no-install-recommends libpq-dev; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /app/target/release/address_book_rest_api .
COPY --from=build /app/Rocket.toml .
COPY --from=build /app/diesel.toml .

# expose port
EXPOSE 8000

# run the binary
ENTRYPOINT ["/app/address_book_rest_api"]
