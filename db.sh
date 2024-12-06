#!/bin/bash

# Database credentials
DB_NAME="chess"
DB_USER="chessadmin"
DB_PASSWORD="securepassword"
DB_HOST="localhost"

# SQL commands
SQL_COMMANDS=$(cat <<EOF
DROP TABLE IF EXISTS replies CASCADE;
DROP TABLE IF EXISTS threads CASCADE;

CREATE TABLE threads (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    last_updated BIGINT NOT NULL
);

CREATE TABLE replies (
    id SERIAL PRIMARY KEY,
    parent_id INT NOT NULL REFERENCES threads(id) ON DELETE CASCADE,
    message TEXT NOT NULL
);

-- Insert sample data
INSERT INTO threads (title, message, last_updated)
VALUES
('Welcome to 4Chess', 'This is a demo thread.', extract(epoch from now())),
('Chess Strategies', 'Share your best chess strategies here!', extract(epoch from now()));

INSERT INTO replies (parent_id, message)
VALUES
(1, 'This is a reply to the first thread.'),
(2, 'The Sicilian Defense is great for aggressive play.');
EOF
)

# Execute SQL commands
export PGPASSWORD="$DB_PASSWORD"

echo "Resetting database and creating tables..."
psql -h $DB_HOST -U $DB_USER -d $DB_NAME -c "$SQL_COMMANDS"

if [ $? -eq 0 ]; then
    echo "Database setup completed successfully."
else
    echo "Error setting up the database."
    exit 1
fi
