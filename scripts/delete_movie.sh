#!/bin/bash

IMPORT_PATH=../proto

grpcurl -plaintext -import-path $IMPORT_PATH -proto $IMPORT_PATH/movie.proto -d '{"index": "bd24fa48-690d-476e-b9c9-e6a746f02941"}' 127.0.0.1:8080 movie.Movie/DeleteMovie
