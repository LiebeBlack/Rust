# EduCore Ultra - Windows Startup Script
# Start the server with cluster support

# Configuration
$ErrorActionPreference = "Stop"

# Change to script directory
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $scriptPath

# Colors for output
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

# Print banner
Write-ColorOutput Cyan "=========================================="
Write-ColorOutput Cyan "   EduCore Ultra - Academic Management"
Write-ColorOutput Cyan "=========================================="
Write-Output ""

# Check if Rust is installed
Write-Output "Checking Rust installation..."
try {
    $rustVersion = rustc --version 2>$null
    if ($rustVersion) {
        Write-ColorOutput Green "[OK] Rust installed: $rustVersion"
    } else {
        Write-ColorOutput Red "[X] Rust not found. Please install Rust from https://rustup.rs/"
        exit 1
    }
} catch {
    Write-ColorOutput Red "[X] Rust not found. Please install Rust from https://rustup.rs/"
    exit 1
}

# Check if Cargo is available
Write-Output "Checking Cargo installation..."
try {
    $cargoVersion = cargo --version 2>$null
    if ($cargoVersion) {
        Write-ColorOutput Green "[OK] Cargo installed: $cargoVersion"
    } else {
        Write-ColorOutput Red "[X] Cargo not found"
        exit 1
    }
} catch {
    Write-ColorOutput Red "[X] Cargo not found"
    exit 1
}

# Create necessary directories
Write-Output "Creating directories..."
New-Item -ItemType Directory -Force -Path "db\backups" | Out-Null
New-Item -ItemType Directory -Force -Path "uploads" | Out-Null
Write-ColorOutput Green "[OK] Directories created"

# Check if .env exists, create from example if not
if (-not (Test-Path ".env")) {
    Write-Output "Creating .env from .env.example..."
    if (Test-Path ".env.example") {
        Copy-Item ".env.example" ".env"
        Write-ColorOutput Yellow "[!] .env created from .env.example. Please review and update configuration."
    } else {
        Write-ColorOutput Red "[X] .env.example not found"
        exit 1
    }
} else {
    Write-ColorOutput Green "[OK] .env exists"
}

# Build the project
Write-Output ""
Write-Output "Building EduCore Ultra..."
Write-Output "This may take a few minutes on first build..."
try {
    cargo build --release
    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput Green "[OK] Build successful"
    } else {
        Write-ColorOutput Red "[X] Build failed"
        exit 1
    }
} catch {
    Write-ColorOutput Red "[X] Build failed: $_"
    exit 1
}

# Check cluster configuration
$enableCluster = $false
if (Test-Path ".env") {
    $envContent = Get-Content ".env" | Where-Object { $_ -match "ENABLE_CLUSTER" }
    if ($envContent -match "true") {
        $enableCluster = $true
        Write-ColorOutput Cyan "[!] Cluster mode enabled"
    }
}

# Start the server
Write-Output ""
Write-Output "Starting EduCore Ultra server..."
Write-Output "Press Ctrl+C to stop"
Write-Output ""

if ($enableCluster) {
    Write-ColorOutput Cyan "Starting in CLUSTER mode..."
    Write-Output "Make sure NODE_ID and SEED_NODES are configured in .env"
    & .\target\release\educore-ultra.exe
} else {
    Write-ColorOutput Cyan "Starting in STANDALONE mode..."
    & .\target\release\educore-ultra.exe
}

# Handle exit
Write-Output ""
Write-ColorOutput Yellow "Server stopped"
