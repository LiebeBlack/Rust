#!/bin/bash
# EduCore Ultra - Linux Startup Script
# Start the server with cluster support

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Print banner
echo -e "${CYAN}=========================================="
echo -e "${CYAN}   EduCore Ultra - Academic Management"
echo -e "${CYAN}==========================================${NC}"
echo ""

# Check if Rust is installed
echo "Checking Rust installation..."
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}✓${NC} Rust installed: $RUST_VERSION"
else
    echo -e "${RED}✗${NC} Rust not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if Cargo is available
echo "Checking Cargo installation..."
if command -v cargo &> /dev/null; then
    CARGO_VERSION=$(cargo --version)
    echo -e "${GREEN}✓${NC} Cargo installed: $CARGO_VERSION"
else
    echo -e "${RED}✗${NC} Cargo not found"
    exit 1
fi

# Create necessary directories
echo "Creating directories..."
mkdir -p db/backups
mkdir -p uploads
echo -e "${GREEN}✓${NC} Directories created"

# Check if .env exists, create from example if not
if [ ! -f .env ]; then
    echo "Creating .env from .env.example..."
    if [ -f .env.example ]; then
        cp .env.example .env
        echo -e "${YELLOW}⚠${NC} .env created from .env.example. Please review and update configuration."
    else
        echo -e "${RED}✗${NC} .env.example not found"
        exit 1
    fi
else
    echo -e "${GREEN}✓${NC} .env exists"
fi

# Build the project
echo ""
echo "Building EduCore Ultra..."
echo "This may take a few minutes on first build..."
if cargo build --release; then
    echo -e "${GREEN}✓${NC} Build successful"
else
    echo -e "${RED}✗${NC} Build failed"
    exit 1
fi

# Check cluster configuration
ENABLE_CLUSTER=false
if [ -f .env ]; then
    if grep -q "ENABLE_CLUSTER=true" .env; then
        ENABLE_CLUSTER=true
        echo -e "${CYAN}⚠${NC} Cluster mode enabled"
    fi
fi

# Start the server
echo ""
echo "Starting EduCore Ultra server..."
echo "Press Ctrl+C to stop"
echo ""

if [ "$ENABLE_CLUSTER" = true ]; then
    echo -e "${CYAN}Starting in CLUSTER mode...${NC}"
    echo "Make sure NODE_ID and SEED_NODES are configured in .env"
    ./target/release/educore-ultra
else
    echo -e "${CYAN}Starting in STANDALONE mode...${NC}"
    ./target/release/educore-ultra
fi

# Handle exit
echo ""
echo -e "${YELLOW}Server stopped${NC}"
