#!/bin/bash
# version: "3"

# services:
#   pipe-1-test:
#     build: .
#     environment:
#       MINIO_BUCKET_NAME: paralon
#       MINIO_REGION: indonesia
#       MINIO_ENDPOINT: http://paralon-minio-1:9000
#       MINIO_ACCESS_KEY: paralon
#       MINIO_SECRET_KEY: paralon123456789
#       MINIO_SECURITY_TOKEN: ""
#       MINIO_SESSION_TOKEN: ""
#       MINIO_PROFILE: ""
#       MINIO_FILE_TARGET: top-songs.csv
#       MINIO_FILE_OUTPUT_TARGET: top-songs-transit.csv
#     networks:
#       - paralon

# networks:
#   paralon:


docker run -it -e MINIO_BUCKET_NAME=paralon \
-e MINIO_REGION=indonesia \
-e MINIO_ENDPOINT=http://paralon-minio-1:9000 \
-e MINIO_ACCESS_KEY=paralon \
-e MINIO_SECRET_KEY=paralon123456789 \
-e MINIO_SECURITY_TOKEN="" \
-e MINIO_SESSION_TOKEN="" \
-e MINIO_PROFILE="" \
-e MINIO_FILE_NAME=top-songs-transit.csv \
-e MINIO_FILE_OUT_NAME=top-songs-transit.csv \
--network paralon_paralon paralon