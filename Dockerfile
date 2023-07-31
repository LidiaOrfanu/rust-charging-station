# Use the official Rust image as the base image
FROM rust:1.67

# Set the working directory for the application
WORKDIR /usr/src/app

# Install PostgreSQL client libraries and other system dependencies
RUN apt-get update && \
    apt-get install -y libpq-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the entire project to the container
COPY . .

# Build the application and its dependencies
RUN cargo build --release

# Set the environment variable for the PostgreSQL connection URL
ENV DATABASE_URL=${DATABASE_URL}

# Expose any necessary ports (if your Rust application listens on a specific port)
EXPOSE 8080

# Run the application when the container starts
CMD ["app"]
