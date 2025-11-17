# Build stage
FROM rust:1.91.1-alpine3.22 AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations
COPY static ./static
COPY build.rs ./build.rs

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache libgcc

# Create a non-root user
RUN addgroup -g 1000 appuser && \
    adduser -D -u 1000 -G appuser appuser

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/lab-rust-iam /app/lab-rust-iam

# Copy migrations
COPY --from=builder /app/migrations /app/migrations

# Copy static files
COPY --from=builder /app/static /app/static

# Change ownership to non-root user
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose the port the app runs on
EXPOSE 3000

# Run the binary
CMD ["/app/lab-rust-iam"]
