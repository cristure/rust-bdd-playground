# Use the official Rust image as a base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml ./

# Copy the tests
COPY tests ./tests

# This is to keep the container running, optional if you want it to exit after tests
ENTRYPOINT ["cargo", "test"]
