# Rocket REST API with JWT

![CI](https://github.com/SakaDream/rocket-rest-api-with-jwt/workflows/CI/badge.svg)
![Docker CICD](https://github.com/SakaDream/rocket-rest-api-with-jwt/workflows/Docker%20CICD/badge.svg)

A Rusty Rocket 🚀 fuelled with Diesel 🛢 and secured by JWT 🔐

## Require

You can build and run app from source:

- [Rust](https://rustup.rs/)
- [Postgres](https://www.postgresql.org/)

Or using [Docker](https://www.docker.com/)

## How to run

### Manual

- Install Rust nightly (Thanks for Rustup 1.20!): `rustup install nightly`
- Set Rust Nightly to project: Go to the root of the project, open cmd/terminal and run `rustup override set nightly`
- Rename `secret.key.sample` to `secret.key` or create your own key by running `head -c16 /dev/urandom > secret.key` in command line (Linux/UNIX only) and copy to `/src` folder
- Create a database in postgres cli or [pgAdmin](https://www.pgadmin.org/) tool
- Rename `Rocket.toml.sample` to `Rocket.toml` and update the database connection string in `url` key.
- Build with release profile: `cargo build --release`
- Run release binary in command line/terminal. On Windows: `target/release/address_book_rest_api.exe`, on *UNIX: `target/release/address_book_rest_api`
- Enjoy! 😄

### Docker

- Enter into project directory and run `docker-compose up`
- Enjoy! 😄

## APIs

### Address: `localhost:8000`

### `POST /api/auth/signup`: Signup
  - Request body:
  ```
  {
     "username": string,
     "email": string,
     "password": string       // a raw password
  }
  ```
  - Response
    - 200 OK
    ```
    {
       "message": "signup successfully",
       "data": ""
    }
    ```
    - 400 Bad Request
    ```
    {
       "message": "error when signing up, please try again",
       "data": ""
    }
    ```

### `POST /api/auth/login`: Login
  - Request body:
  ```
  {
     "username_or_email": string,
     "password": string       // a raw password
  }
  ```
  - Response
    - 200 OK
    ```
    {
       "message": "login successfully",
       "data": {
         "token": string      // bearer token
       }
    }
    ```
    - 400 Bad Request
    ```
    {
       "message": "wrong username or password, please try again",
       "data": ""
    }
    ```

### `GET /api/address-book`: Get all people information
  - Header:
    - Authorization: bearer \<token\>
  - Response
    - 200 OK
    ```
    {
       "message": "ok",
       "data": [
          {
            "id": int32,
            "name": string,
            "gender": boolean,      // true for male, false for female
            "age": int32,
            "address": string,
            "phone": string,
            "email": string
          }
       ]
    }
    ```

### `GET /api/address-book/{id}`: Get person information by id
  - Param path:
    - id: int32
  - Header:
    - Authorization: bearer \<token\>
  - Response
    - 200 OK
    ```
    {
       "message": "ok",
       "data": {
         "id": int32,
         "name": string,
         "gender": boolean,      // true for male, false for female
         "age": int32,
         "address": string,
         "phone": string,
         "email": string
       }
    }
    ```
    - 404 Not Found
    ```
    {
       "message": "person with id {id} not found",
       "data": ""
    }
    ```

### `GET /api/address-book/{query}`: Search for person information by keyword
  - Param path:
    - query: string
  - Header:
    - Authorization: bearer \<token\>
  - Response
    - 200 OK
    ```
    {
       "message": "ok",
       "data": [
         {
           "id": int32,
           "name": string,
           "gender": boolean,      // true for male, false for female
           "age": int32,
           "address": string,
           "phone": string,
           "email": string
         }
       ]
    }
    ```

### `POST /api/address-book`: Add person information
  - Header:
    - Authorization: bearer \<token\>
  - Request body:
    ```
    {
      "name": string,
      "gender": boolean,      // true for male, false for female
      "age": int32,
      "address": string,
      "phone": string,
      "email": string
    }
    ```
  - Response
    - 201 Created
    ```
    {
      "message": "ok",
      "data": ""
    }
    ```
    - 500 Internal Server Error
    ```
    {
      "message": "can not insert data",
      "data": ""
    }
    ```  

### `PUT /api/address-book/{id}`: Update person information by id
  - Param path:
    - id: int32
  - Header:
    - Authorization: bearer \<token\>
  - Request body:
  ```
  {
    "name": string,
    "gender": boolean,      // true for male, false for female
    "age": int32,
    "address": string,
    "phone": string,
    "email": string
  }
  ```
  - Response
    - 200 OK
    ```
    {
      "message": "ok",
      "data": ""
    }
    ```
    - 500 Internal Server Error
    ```
    {
      "message": "can not update data",
      "data": ""
    }
    ```

### `DELETE /api/address-book/{id}`: Delete person information by id
  - Param path:
    - id: int32
  - Header:
    - Authorization: bearer \<token\>
  - Response
    - 200 OK
    ```
    {
      "message": "ok",
      "data": ""
    }
    ```
    - 500 Internal Server Error
    ```
    {
      "message": "can not delete data",
      "data": ""
    }
    ```

### Errors:
  - Invalid or missing token
    - Status code: 401 Unauthorized
    - Response:
    ```
    {
      "message": "invalid token, please login again",
      "data": ""
    }
    ```
