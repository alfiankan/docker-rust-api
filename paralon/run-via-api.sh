#!/bin/bash

curl --unix-socket /var/run/docker.sock -H "Content-Type: application/json" \
  -d '{"Image": "paralon", "Env": ["MINIO_BUCKET_NAME=paralon", "MINIO_REGION=indonesia", "MINIO_ENDPOINT=http://paralon-minio-1:9000", "MINIO_ACCESS_KEY=paralon", "MINIO_SECRET_KEY=paralon123456789", "MINIO_SECURITY_TOKEN=''", "MINIO_SESSION_TOKEN=''", "MINIO_PROFILE=''", "MINIO_FILE_NAME=top-songs-transit.csv", "MINIO_FILE_OUT_NAME=top-songs-transit.csv"]}' \
  -X POST http://localhost/v1.41/containers/create


curl --unix-socket /var/run/docker.sock -H "Content-Type: application/json" \
  -d '{"Container": "9a1cbad074adbd1d4c37b379d2a0d6a8b38056cc37cc9ecd7c27b6d9847dc709"}' \
  -X POST http://localhost/v1.41/networks/4ae4ef1816b0/connect


curl --unix-socket /var/run/docker.sock -H "Content-Type: application/json" \
  -X POST http://localhost/v1.41/containers/9a1cbad074adbd1d4c37b379d2a0d6a8b38056cc37cc9ecd7c27b6d9847dc709/start

curl --unix-socket /var/run/docker.sock -H "Content-Type: application/json" \
  -X DELETE http://localhost/v1.41/containers/0ef4b5632d40