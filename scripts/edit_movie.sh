#!/bin/bash

IMPORT_PATH=../proto

grpcurl -plaintext -import-path $IMPORT_PATH -proto $IMPORT_PATH/movie.proto -d '{"index": "86379179-6ce6-40dd-82a4-2042df91a540", "title": "Edited Movie", "year": 2024, "genre": "Thriller"}' 127.0.0.1:8080 movie.Movie/EditMovie
