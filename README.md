# Backend Component

This component uses Rocket.rs to create an efficient REST API component that can be used to perform CRUD operations on data. This service connects to a PostgreSQL database that must be provided by the user by means of an environment variable named `DATABASE_URL`. Also provide values for `JWT_SECRET` and `JWT_EXPIRATION` to enable JWT token generation and validation. (See the `.env.example` file for an example of the environment variables to be provided.)

For the purposes of this project, a PostgreSQL database is provided by the `docker-compose.yml` file at the root of this repository

## Technologies
Rust: Base language for the backend service. A compiled language that provides memory safety and performance.
Rocket.rs: A web framework for Rust that makes it simple to create fast and secure web services.
Diesel.rs: A query builder and ORM for Rust that provides a type-safe and composable way to interact with databases.
PostgreSQL: A powerful, open-source object-relational database system.

## Requirements
- Rust v1.78.0
- Cargo v1.18.0

## Installation (Cargo)

To install the dependencies, run the following command:

```bash
cargo build
```

To start the backend service, run the following command:

```bash
npm start
```

## Installation (Docker)

I've provided a Makefile to simplify the process of building and running the frontend service using Docker. To build the Docker image, run the following commands:

```bash
make docker.build
make docker.run
```

## Usage

The backend service will be running on `http://localhost:8000`. A postman collection is provided to test some of the endpoints implemented in the service.

Note: You can provide the ROCKET_PORT and ROCKET_ADDRESS env variables to change the port and address where the service will be running.