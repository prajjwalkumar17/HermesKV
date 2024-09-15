FROM rust:bookworm as builder

# Set the working directory inside the container to /hermes
WORKDIR /hermes

# Copy the current directory (where Dockerfile is located) to /hermes in the container
COPY . .

# Build the project
RUN cargo build --release

# Stage 2
FROM debian:bookworm

# Install curl
RUN apt-get update \
    && apt-get install -y curl

# Set environment variables
ENV BINARY=hermes
ENV BIN_DIR=/usr/local/bin

# Expose the port Hermes server will run on
EXPOSE 8080

# Command to run your application
RUN mkdir -p ${BIN_DIR}

COPY --from=builder /hermes/target/release/${BINARY} ${BIN_DIR}/${BINARY}

WORKDIR ${BIN_DIR}

CMD ./${BINARY}
