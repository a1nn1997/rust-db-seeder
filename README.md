# DB Seeder

A Rust application for populating PostgreSQL databases with realistic test data for blog-style applications.

## Overview

DB Seeder generates and inserts test data into a PostgreSQL database, creating a realistic dataset that includes:

- Users with hashed passwords
- Posts with markdown content
- Comments with hierarchical structure (parent/child relationships)
- User interactions (likes, shares)
- Content recommendations with scores

The project is designed to be configurable and can be used to quickly populate development environments or testing databases.

## Features

- ✅ Generates consistent, realistic test data
- ✅ Creates hierarchical comment structures
- ✅ Populates TimescaleDB hypertables for time-series data (interactions)
- ✅ Securely hashes passwords using Argon2
- ✅ Customizable data generation volumes
- ✅ Works with existing database schemas

## Prerequisites

- Rust (latest stable version recommended)
- PostgreSQL 12+ with TimescaleDB extension
- A database with the required schema already created

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/db-seeder.git
   cd db-seeder
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

## Configuration

Create a `.env` file in the project root with the following settings:

```
DATABASE_URL=postgres://username:password@localhost:5432/your_db?options=--search_path%3Dglobal
```

Adjust the following parameters in `src/config/mod.rs`:

- `NUM_USERS`: Number of users to generate
- `NUM_POSTS`: Number of posts to generate
- `NUM_COMMENTS`: Number of comments to generate
- `NUM_INTERACTIONS`: Number of interactions to generate
- `NUM_RECOMMENDATIONS`: Number of recommendations to generate

## Usage

Run the application to populate the database:

```bash
cargo run
```

### Expected Database Schema

The application expects the following database schema in the `global` schema:

```sql
-- Users table
CREATE TABLE global.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'user',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT valid_role CHECK (role IN ('user', 'admin', 'moderator'))
);

-- Posts table with similar structure
-- Comments table with hierarchical structure
-- Interactions table (TimescaleDB hypertable)
-- Recommendations table
```

## Creating a Test User

To create a test user with a known password for login testing:

```sql
INSERT INTO global.users (id, username, email, password_hash, role)
VALUES (
    gen_random_uuid(),
    'testuser',
    'test@example.com',
    -- Password hash for "password123"
    '$argon2id$v=19$m=4096,t=3,p=1$somesaltvalue$hashedpasswordvalue',
    'admin'
);
```

## Project Structure

```
src/
├── config/          - Configuration settings
├── db/              - Database operations
├── generators/      - Data generators
├── models/          - Data models
└── query/           - SQL queries
```

## License

[MIT](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
