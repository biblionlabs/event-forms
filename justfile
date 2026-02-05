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
    wrangler dev --local

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
# Database (D1) - Local
# ============================================================================

# Run migrations on local database
db-migrate:
    wrangler d1 execute event-forms-db --local --file=migrations/001_initial_schema.sql

# List tables in local database
db-tables:
    wrangler d1 execute event-forms-db --local --command "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"

# Query local database
db-query query:
    wrangler d1 execute event-forms-db --local --command "{{query}}"

# Reset local database (delete and recreate)
db-reset:
    rm -rf .wrangler/state/v3/d1
    @echo "Local database deleted. Run 'just db-migrate' to recreate."

# ============================================================================
# Database (D1) - Production
# ============================================================================

# Create production D1 database (run once)
db-create-prod:
    wrangler d1 create event-forms-db
    @echo "Update wrangler.toml [env.production.d1_databases] database_id with the ID above"

# Run migrations on production database
db-migrate-prod:
    wrangler d1 execute event-forms-db --remote --file=migrations/001_initial_schema.sql

# List tables in production database
db-tables-prod:
    wrangler d1 execute event-forms-db --remote --command "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"

# Query production database
db-query-prod query:
    wrangler d1 execute event-forms-db --remote --command "{{query}}"

# ============================================================================
# Deployment
# ============================================================================

# Deploy to Cloudflare Workers (production)
deploy:
    wrangler deploy --env production

# Show deployment info
info:
    wrangler whoami

# Tail production logs
logs:
    wrangler tail --env production

# ============================================================================
# Secrets Management (Production)
# ============================================================================

# Set admin password secret
set-admin-password password:
    echo "{{password}}" | wrangler secret put ADMIN_PASSWORD --env production

# Set encryption key secret
set-encryption-key key:
    echo "{{key}}" | wrangler secret put ENCRYPTION_KEY --env production

# List all secrets
secrets-list:
    wrangler secret list --env production

# ============================================================================
# Quality & Utilities
# ============================================================================

# Run all quality checks
qa: fmt lint check
    @echo "All quality checks passed!"

# Clean build artifacts
clean:
    cargo clean
    rm -rf build/
    rm -rf .wrangler/

# Generate a random 32-character encryption key
gen-key:
    @openssl rand -base64 32 | tr -d '\n' | head -c 32
    @echo ""

# Initialize local development (reset DB and run migrations)
init: db-reset db-migrate
    @echo "Local development environment initialized!"

# Quick start: init and dev
start: init dev
