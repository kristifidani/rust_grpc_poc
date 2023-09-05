#!/bin/bash

IMPORT_PATH=../proto

grpcurl -plaintext -import-path $IMPORT_PATH -proto $IMPORT_PATH/movie.proto -d '{"id": "1", "title": "Edited Movie", "year": 2024, "genre": "Thriller"}' 127.0.0.1:8080 movie.Movie/EditMovie
