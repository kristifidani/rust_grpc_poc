# Rust gRPC API PoC

### Introduction

This project is intented as a PoC to showcases the use of **gRPC** API in **Rust**. It defines a gRPC service with CRUD operation for movies. 

### Prerequisites

Before you begin, ensure you have met the following requirements:

- **Rust** and **Cargo** installed. Check the official [website](https://www.rust-lang.org/learn/get-started).
- **Protocol Buffers** compiler installed on the local machine. You can download it from the official [GitHub repository](https://github.com/protocolbuffers/protobuf/releases/tag/v24.2).

### Build and Run

- Build the project: `cargo build`
- Run the project: `cargo run`

_Before running_:
 * Run the **PostgresDb** container: `docker-compose up -d`
 * Make sure you have set the `.env` variables:  
 DB_URL=`postgres://postgres:postgres@localhost:5432/postgres`

### Testing

* Run _unit tests_: `make unit-tests`
* Run _integration tests_: `make integration-tests`  

Alternatively you can execute some scripts:  
1. Navigate to the scripts directory: `cd ./scripts`
1. Then you can execute the following scripts:  
    * _Fetch movies_: `bash fetch_movies.sh` 
    * _Add movie_: `bash add_movie.sh`
    * _Edit movie_: `bash edit_movie.sh`
    * _Delete movie_: `bash delete_movie.sh`
