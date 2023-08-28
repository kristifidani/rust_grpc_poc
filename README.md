# GRPC RUST API

### Introduction

This Rust project showcases the use of **gRPC** to build an API for movie services. It defines a gRPC service for retrieving a list of movies, and it includes a sample implementation for this service.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- **Rust** and **Cargo** installed. Check the official [website](https://www.rust-lang.org/learn/get-started).
- **Protocol Buffers** compiler installed on the local machine. You can download it from the official [GitHub repository](https://github.com/protocolbuffers/protobuf/releases/tag/v24.2).

### Build
- Build the project with: `cargo build`
- Run the project with: `cargo run`

### Test

- Run unit-tests with: `cargo test --bin movies-rust-grpc -- --nocapture`

### Usage

After running the project with `cargo run`, interact with the API using either of the following: 
 
- Install [grpcurl](https://github.com/fullstorydev/grpcurl/releases) retreive a list of all movies using:   
`grpcurl -plaintext -import-path proto -proto movie.proto 127.0.0.1:8080 movie.Movie/GetMovies`.
- Alternatively use [Postman](https://www.postman.com/).
