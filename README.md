# Backend Component

This component uses Rocket.rs to create an efficient REST API component that can be used to perform CRUD operations on data. This service connects to a PostgreSQL database that must be provided by the user by means of an environment variable named `DATABASE_URL`. Also provide values for `JWT_SECRET` and `JWT_EXPIRATION` to enable JWT token generation and validation. (See the `.env.example` file for an example of the environment variables to be provided.)

For the purposes of this project, a PostgreSQL database is provided by the `docker-compose.yml` file. This file also provides a `pgadmin` service that can be used to interact with the database.

## Installation (NPM)

To install the dependencies, run the following command:

```bash
npm install

To start the frontend service, run the following command:

```bash
npm start
```

# Installation (Docker)

I've provided a Makefile to simplify the process of building and running the frontend service using Docker. To build the Docker image, run the following commands:

```bash
make docker.build
make docker.run
```

## Usage

The frontend service will be running on `http://localhost:3000`. You can access this URL from your web browser to interact with the web interface.

Notes: 
- The frontend service requires the backend service to be running in order to work properly.