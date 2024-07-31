# Oracle Database REST Service with Actix

## Objective

This project aims to provide a REST service to access an Oracle database, using the Actix framework for the web server. It is designed as an educational demonstration of creating a REST API with CORS (Cross-Origin Resource Sharing) management.

## Technologies Used

- **Rust**: Programming language used to develop the service.
- **Actix**: Web framework in Rust for building high-performance HTTP servers.
- **Oracle**: Relational database used to store data.
- **dotenv**: For managing environment variables.
- **serde**: For data serialization and deserialization.
- **validator**: For validating request data.

## Features

- **Oracle Database Connection**: Connection and CRUD (Create, Read, Update, Delete) operations on client data.
- **REST API**: Provides REST endpoints to interact with the database.
- **CORS Management**: CORS configuration to allow access from different domains.

## Configuration

### Prerequisites

- **Rust**: Make sure Rust is installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).
- **Oracle Database**: Ensure access to an Oracle Database instance.

### Environment Variables

Create a `.env` file at the root of the project to configure the necessary environment variables:

```env
ORACLE_USER=<your_oracle_user>
ORACLE_PASSWORD=<your_oracle_password>
ORACLE_HOST=<your_oracle_host>
ORACLE_SERVICE=<your_oracle_service>
```


### Compilation rustls sous Windows 

Install the following:

Installer Cmake https://cmake.org/download/
Installer Nasm https://www.nasm.us/pub/nasm/releasebuilds/2.16.02/win64/
Installer Clang https://github.com/vovkos/llvm-package-windows/releases/clang-master

Add the following environment variables:

Ajouter  des variables d'envirenement 

```env
set LIBCLANG_PATH=C:\clang-13.0.0\bin  
set PATH=C:\Program Files\CMake\bin;%PATH%
set PATH=C:\Program Files\NASM;%PATH%

```

### Compilation rustls sous Windows 

```sh
cargo build

cargo run

```

### Testing

Unit tests are available to verify CRUD operations. To run the tests, use the following command:

```sh
cargo test

```


### REST Endpoints

Add a Customer

URL: /customers

Method: POST

Description: Adds a new customer to the database.

Request Body:

json

```json
{
    "first_name": "John",
    "last_name": "Doe",
    "address": "123 Main St",
    "email": "john.doe@example.com",
    "password": "securepassword"
}
```
