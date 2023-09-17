﻿# Use a base image with the latest version of Rust installed
FROM rust:latest

ARG SQLX_OFFLINE=true

# Set the working directory in the container
WORKDIR /app

# Copy the local application code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

EXPOSE 8080

# Specify the command to run when the container starts
CMD ["./target/release/sqlx"]