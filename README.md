# Rust Property & Payment API

A robust and secure RESTful API built with Rust, Axum, and SQLx, designed for managing users, properties, and payments. This project serves as a practical demonstration of modern backend development principles using Rust, including asynchronous database interactions with PostgreSQL, secure password hashing, and a foundation for JWT-based authentication. It's envisioned as the core backend for a property management or real estate application.

## Table of Contents
* [Features](#-features)
* [Technologies Used](#-technologies-used)
* [Local Setup & Installation](#user-content-️-local-setup--installation)
* [Testing the API](#-testing-the-api)
* [Future Enhancements](#-future-enhancements)

id="user-content-️-local-setup--installation"
## ✨ Features

* **User Management:**
    * Register new user accounts.
    * Login users and issue JSON Web Tokens (JWTs).
* **Property Management:**
    * Create new property listings with associated details.
    * Retrieve all available properties.
* **Payment Management:**
    * Record new payment transactions.
    * List payments, with optional filtering by user or property.
* **Health Check:** A simple endpoint to verify API operational status.

## 🚀 Technologies Used

* **Rust:** A safe, concurrent, and performant language.
* **Axum:** A fast, ergonomic, and modular web framework built on Tokio and Tower.
* **SQLx:** An asynchronous, compile-time checked ORM for Rust, interacting with PostgreSQL.
* **PostgreSQL:** A powerful, open-source relational database.
* **Docker & Docker Compose:** For containerization of the application and orchestration of multi-service environments (app + database).
* **Cargo Chef:** For optimizing Docker build times by efficiently caching Rust dependencies.
* **AWS Elastic Container Registry (ECR):** A fully-managed Docker container registry for storing application images.
* **AWS Elastic Container Service (ECS) with Fargate:** A serverless container orchestration service for deploying and managing the application.
* **AWS Relational Database Service (RDS):** A managed database service for PostgreSQL in the cloud.
* **AWS Application Load Balancer (ALB):** For distributing incoming traffic to the ECS tasks.
* **AWS Virtual Private Cloud (VPC):** Provides a logically isolated section of the AWS Cloud where AWS resources are launched.
* **AWS CloudWatch:** For monitoring, logging, and observing the application's performance and health.
* **Bcrypt:** For secure password hashing and verification.
* **jsonwebtoken:** For creating and verifying JWTs.
* **Tokio:** The asynchronous runtime for Rust.
* **dotenvy:** For managing environment variables.

## ☁️ Cloud Deployment (AWS)

This project has been successfully deployed to AWS. The public endpoint below was active during the demo and testing phase.

**Important Note:** To avoid ongoing charges, the AWS resources related to this deployment (ALB, ECS cluster, RDS instance, etc.) may have been deleted. If the endpoint does not work, it is likely due to resource deprovisioning.

## ⚙️ Local Setup & Installation

Follow these steps to get the application running locally.

### Prerequisites

* **Rust & Cargo:** Install Rust and Cargo using `rustup`: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
* **PostgreSQL:** Ensure you have a PostgreSQL server installed and running.
* **`sqlx-cli`:** The command-line tool for `sqlx` migrations:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### 1. Clone the Repository

```bash
git clone https://github.com/PaulGLujan/rust_api.git
cd rust_api
```

### 2. Environment Variables

Create a .env file in the root of your project (next to Cargo.toml and docker-compose.yml) and populate it with your database credentials and a JWT secret. Docker Compose will automatically pick up variables from this file.

```
# .env

# PostgreSQL credentials for the Dockerized DB
POSTGRES_USER=myuser
POSTGRES_PASSWORD=mypassword
POSTGRES_DB=dev_db

# JWT Secret for your Rust app
JWT_SECRET=a_super_secure_jwt_key_for_docker_compose_env
```

Remember to replace myuser, mypassword, dev_db, and a_super_secure_jwt_key_for_docker_compose_env with your desired values.

### 3. Docker Compose Setup & Run

Your PostgreSQL database will be automatically set up and run in a Docker container alongside your Rust API using Docker Compose.

The docker-compose.yml file:

- Defines a db service (PostgreSQL).
- Defines an app service (your Rust API), building its image from the Dockerfile and linking to the db service.
- Configures a persistent volume (./db_data) for your database files, so your data persists even if containers are recreated.

To build the Docker images and start both services:

```
docker compose up --build -d
```

- --build: Ensures your Rust application image is built (or rebuilt if changes are detected in your Dockerfile or Rust source).
- -d: Runs the containers in "detached" mode (in the background).

### 4. Run Database Migrations

Once the db service is up, you need to apply your database schema migrations. You will use your locally installed sqlx-cli to connect to the Dockerized database.

First, set the DATABASE_URL environment variable in your current terminal session to point to your Dockerized database:

```
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}"
```

**Important:** Replace** ${POSTGRES_USER}, ${POSTGRES_PASSWORD}, and ${POSTGRES_DB} with the actual values from your project's .env file. (e.g., postgres://myuser:mypassword@localhost:5432/dev_db)

Then, run the migrations:

```
cargo sqlx migrate run
```

### 5. Verify Services Are Running (Optional)

You can check the status of your running services:

```
docker compose ps
```

And view logs from your application:

```
docker compose logs app
```

(Press `Ctrl+C` to exit logs.)

### 6. Stop Services

When you're done, you can stop and remove the containers:

```
docker compose down
```

- To remove the database data and start fresh, use `docker compose down -v`.

## 🧪 Testing the API

You can test the API manually using command-line tools like `curl` or graphical clients like Postman. All examples below assume the API is accessible at `rust-api-alb-151556608.us-east-2.elb.amazonaws.com` (replace with your local or deployed endpoint as appropriate).

### Manual Testing with `curl`

Open a new terminal window and send requests to your API endpoint.

### 1. Health Check

Verifies the server is running.

```
curl rust-api-alb-151556608.us-east-2.elb.amazonaws.com/health_check
```

#### Expected Response: `OK` (HTTP Status `200 OK`)

### 2. User Registration

Creates a new user account. Change `demo_user` and `demo@example.com` for subsequent attempts.

```
curl -X POST -H "Content-Type: application/json" -d '{
    "username": "demo_user",
    "email": "demo@example.com",
    "password": "SecurePassword123"
}' rust-api-alb-151556608.us-east-2.elb.amazonaws.com/register
```

#### Expected Response (Success - HTTP Status `200 OK`):

```
{
  "id": "...",
  "username": "demo_user",
  "password_hash": "...",
  "email": "demo@example.com",
  "created_at": "...",
  "updated_at": "..."
}
```

#### Expected Response (Conflict - HTTP Status `409 Conflict` if username/email taken):

```
{
  "error": "Conflict: Username or email already taken."
}
```

### 3. User Login

Authenticates a user and retrieves a JWT. Copy this JWT token!

```
curl -X POST -H "Content-Type: application/json" -d '{
    "username": "demo_user",
    "password": "SecurePassword123"
}' rust-api-alb-151556608.us-east-2.elb.amazonaws.com/login
```

#### Expected Response (Success - HTTP Status `200 OK`):

```
{
  "user_id": "...",
  "username": "demo_user",
  "token": "eyJhbGciOiJIUzI1NiJ9..." # <-- THIS IS YOUR JWT TOKEN
}
```

#### Expected Response (Unauthorized - HTTP Status `401 Unauthorized` if credentials are wrong):

```
{
  "error": "Unauthorized: Invalid username or password"
}
```

### 4. Create Property

(Note: As of current implementation, this endpoint does NOT require authentication. This will be added in future work.) The `current_tenant_id` must be a valid user id.

```
curl -X POST -H "Content-Type: application/json" -d '{
    "address": "123 Main St, New York, NY",
    "unit_number": "4",
    "current_rent_amount": "3000.00",
    "current_tenant_id": "109b2942-4696-4dda-88f3-aa47962d4baa"
}' rust-api-alb-151556608.us-east-2.elb.amazonaws.com/properties
```

#### Expected Response (Success - HTTP Status `200 OK`):

```
{
  "id": "...",
  "address": "123 Main St, New York, NY",
  "unit_numbers": ["1A"],
  "current_rent_amount": "3000.00",
  "current_tenant_id": "109b2942-4696-4dda-88f3-aa47962d4baa"
  "created_at": "...",
  "updated_at": "..."
}
```

### 5. List Properties

```
curl rust-api-alb-151556608.us-east-2.elb.amazonaws.com/properties
```

#### Expected Response (Success - HTTP Status `200 OK`):

```
[
  {
    "id": "...",
    "address": "123 Main St, New York, NY",
    "unit_numbers": ["1A"],
    "current_rent_amount": "3000.00",
    "current_tenant_id": "109b2942-4696-4dda-88f3-aa47962d4baa"
    "created_at": "...",
    "updated_at": "..."
  }
  // ... other properties
]
```

### 6. Create Payment

(Note: This endpoint does NOT require authentication. This will be added in future work.)
You'll need a user_id and property_id from previous steps.

```
curl -X POST -H "Content-Type: application/json" -d '{
    "user_id": "YOUR_USER_ID_FROM_REGISTRATION",
    "property_id": "YOUR_PROPERTY_ID_FROM_PROPERTY_CREATION",
    "amount": "1500.00",
    "currency": "USD",
    "notes": "Monthly rent",
    "due_date": "2025-06-01",
    "period_start": "2025-06-01",
    "period_end": "2025-06-30"
}' rust-api-alb-151556608.us-east-2.elb.amazonaws.com/payments
```

#### Expected Response (Success - HTTP Status `200 OK`):

```
{
  "id": "...",
  "user_id": "...",
  "property_id": "...",
  "amount": "1500.00",
  "currency": "USD",
  "status": "Pending",
  "notes": "Monthly rent",
  "transaction_id": null,
  "due_date": "2025-06-01",
  "period_start": "2025-06-01",
  "period_end": "2025-06-30"
}
```

### 7. List Payments

You can list all payments, or filter by `user_id` or `property_id`.

```
# List all payments
curl rust-api-alb-151556608.us-east-2.elb.amazonaws.com/payments

# List payments for a specific user
curl rust-api-alb-151556608.us-east-2.elb.amazonaws.com/payments?user_id=YOUR_USER_ID

# List payments for a specific property
curl rust-api-alb-151556608.us-east-2.elb.amazonaws.com/payments?property_id=YOUR_PROPERTY_ID
```

#### Expected Response (Success - HTTP Status `200 OK`):

```
[
  {
    "id": "...",
    "user_id": "...",
    "property_id": "...",
    "amount": "1500.00",
    "currency": "USD",
    "status": "Pending",
    "notes": "Monthly rent",
    "transaction_id": null,
    "due_date": "2025-06-01",
    "period_start": "2025-06-01",
    "period_end": "2025-06-30"
  },
  // ... other payments
]
```

## 💡 Future Enhancements

- Authentication Middleware: Implement JWT validation to secure create_property, create_payment, and potentially list_payments endpoints.
- Authorization: Role-based access control (e.g., only authenticated users can create payments, or only admins can create properties).
- Full CRUD: Add GET by ID, PUT/PATCH (update), and DELETE functionality for properties and payments.
- Input Validation: More robust server-side validation for request bodies.
- Pagination, Filtering, Sorting: Advanced querying capabilities for listing endpoints.
- Error Handling Refinements: More specific error messages and HTTP status codes for various scenarios.
- Deployment: CI/CD pipeline