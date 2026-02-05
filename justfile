# Event Forms - Development Commands
# Use: just <command>

# Default command
default:
    @just --list

# ============================================================================
# Development
# ============================================================================

# Start local development server
dev:
    wrangler dev --env local

# Build the project
build:
    cargo build --target wasm32-unknown-unknown --release

# Check code without building
check:
    cargo check --target wasm32-unknown-unknown

# Format code
fmt:
    cargo fmt

# Run clippy linter
lint:
    cargo clippy --target wasm32-unknown-unknown -- -D warnings

# ============================================================================
# Database (D1)
# ============================================================================

# Create a new D1 database
db-create name="event-forms-db":
    wrangler d1 create {{name}}

# Run all migrations on local database
db-migrate-local:
    wrangler d1 migrations apply event-forms --local

# Run all migrations on remote database
db-migrate-remote:
    wrangler d1 migrations apply event-forms --remote

# ============================================================================
# Deployment
# ============================================================================

# Deploy to Cloudflare Workers (production)
deploy:
    wrangler deploy

# Deploy to specific environment
deploy-env env:
    wrangler deploy --env {{env}}

# Show deployment info
info:
    wrangler whoami

# Tail production logs
logs:
    wrangler tail

# Tail logs for specific environment
logs-env env:
    wrangler tail --env {{env}}

# ============================================================================
# Secrets Management
# ============================================================================

# Set admin password secret
set-admin-password password:
    echo "{{password}}" | wrangler secret put ADMIN_PASSWORD

# Set encryption key secret
set-encryption-key key:
    echo "{{key}}" | wrangler secret put ENCRYPTION_KEY

# List all secrets
secrets-list:
    wrangler secret list

# ============================================================================
# Testing & Quality
# ============================================================================

# Run all quality checks
qa: fmt lint check
    @echo "All quality checks passed!"

# Clean build artifacts
clean:
    cargo clean
    rm -rf build/
    rm -rf .wrangler/

# ============================================================================
# Utilities
# ============================================================================

# Generate a random 32-character encryption key
gen-key:
    @openssl rand -base64 32 | head -c 32
    @echo ""

# Initialize local development (create DB and run migrations)
init-local: db-migrate-local
    @echo "Local development environment initialized!"

# Show current wrangler configuration
config:
    @cat wrangler.toml
