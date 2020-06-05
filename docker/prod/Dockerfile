# base image
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
