# Multi-stage Dockerfile for EduCore Ultra
# Optimized for production deployment

# Stage 1: Build
FROM rust:1.75-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev sqlite-dev

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Stage 2: Runtime
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache sqlite ca-certificates tzdata

# Create non-root user
RUN addgroup -g 1000 academia && \
    adduser -D -u 1000 -G academia academia

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/educore-ultra /app/educore-ultra

# Create necessary directories
RUN mkdir -p db/backups uploads assets && \
    chown -R academia:academia /app

# Switch to non-root user
USER academia

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3000/health || exit 1

# Run the application
CMD ["/app/educore-ultra"]
