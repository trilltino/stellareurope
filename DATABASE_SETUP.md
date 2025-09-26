# Database Setup Guide

## Option 1: Docker (Recommended)

If you have Docker installed, run:

```bash
# Start PostgreSQL with Docker Compose
docker-compose up postgres -d

# The database will be available at:
# Host: localhost
# Port: 5432
# Database: stellareurope
# Username: postgres
# Password: password
```

## Option 2: Local PostgreSQL Installation

### Windows
1. Download PostgreSQL from https://www.postgresql.org/download/windows/
2. Install PostgreSQL with default settings
3. Set password as 'password' during installation
4. Create a database named 'stellareurope'

### macOS
```bash
# Using Homebrew
brew install postgresql
brew services start postgresql

# Create database
createdb stellareurope
```

### Linux (Ubuntu/Debian)
```bash
# Install PostgreSQL
sudo apt update
sudo apt install postgresql postgresql-contrib

# Set password and create database
sudo -u postgres psql
ALTER USER postgres PASSWORD 'password';
CREATE DATABASE stellareurope;
\q
```

## Testing Database Connection

Once PostgreSQL is running, you can test the connection:

```bash
# Test connection
psql -h localhost -p 5432 -U postgres -d stellareurope

# Or using the connection string
psql postgresql://postgres:password@localhost:5432/stellareurope
```

## Environment Configuration

Make sure your `.env` file contains:
```
DATABASE_URL=postgresql://postgres:password@localhost:5432/stellareurope
RUST_LOG=info
```

## Running Migrations

The backend will automatically run migrations when started. You can also run them manually:

```bash
cd backend
cargo install sqlx-cli
sqlx migrate run
```

## Troubleshooting

### Authentication Failed
- Ensure PostgreSQL is running
- Verify the password is set correctly
- Check the connection string in `.env`

### Database Not Found
- Create the database: `CREATE DATABASE stellareurope;`
- Verify the database name in the connection string

### Connection Refused
- Check if PostgreSQL is running: `ps aux | grep postgres`
- Verify the port (default 5432) is not blocked
- Check firewall settings