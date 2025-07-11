# GRPC RUST API

### Introduction

This Rust project showcases the use of **gRPC** to build an API for movie services. It defines a gRPC service for retrieving a list of movies, creating a movie, deleting and updating movies. Also it includes sample implementations for these services.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- **Rust** and **Cargo** installed. Check the official [website](https://www.rust-lang.org/learn/get-started).
- **Protocol Buffers** compiler installed on the local machine. You can download it from the official [GitHub repository](https://github.com/protocolbuffers/protobuf/releases/tag/v24.2).

### Build and Run

- Build the project: `cargo build`
- Run the project: `cargo run`

Make sure you have set the `.env` variables:
* DB_URL=`postgres://postgres:postgres@localhost:5432/postgres`
* DB_NAME=`movies`

### Usage

1. Start docker containers: `docker-compose up -d`
1. Run the project: `cargo run`
1. Create movie: `cd script/ && bash add_movie.sh`
1. Fetch movies: `cd script/ && bash fetch_movies.sh`
1. Edit movie: `cd script/ && bash edit_movie.sh`
1. Delete movie: `cd script/ && bash delete_movie.sh`
