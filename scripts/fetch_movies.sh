#!/bin/bash

IMPORT_PATH=../proto

grpcurl -plaintext -import-path $IMPORT_PATH -proto $IMPORT_PATH/movie.proto 127.0.0.1:8080 movie.Movie/GetMovies
