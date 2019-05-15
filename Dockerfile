# build stage
FROM rustlang/rust:nightly-slim as build

# install libpq
RUN apt-get update
RUN apt-get install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*

# create new empty binary project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# build this project to cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy project source and necessary files
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .

# add Rocket.toml and secret.key for Docker env
RUN touch Rocket.toml
RUN printf "[global.databases]\npostgres_database = { url = \"postgres://postgres:postgres@db/postgres\" }\n" > Rocket.toml
RUN mv src/secret.key.sample src/secret.key

# rebuild app with project source
RUN rm ./target/release/deps/address_book_rest_api*
RUN cargo build --release

# deploy stage
FROM debian:stretch-slim

# create app directory
RUN mkdir app
WORKDIR /app

# install libpq
RUN apt-get update
RUN apt-get install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /app/target/release/address_book_rest_api .
COPY --from=build /app/Rocket.toml .
COPY --from=build /app/diesel.toml .

# expose port
EXPOSE 8000

# run the binary
ENTRYPOINT ["/app/address_book_rest_api"]
