-- SQLite doesn't support adding a UNIQUE constraint directly in ALTER TABLE
-- So we need to create a new table with the email column and copy data over

-- 1. Create a new table with the email column
CREATE TABLE admin_user_new (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    username      TEXT NOT NULL UNIQUE,
    email         TEXT UNIQUE,
    password_hash TEXT NOT NULL,
    user_type_id  INTEGER NOT NULL REFERENCES user_type (id) ON DELETE RESTRICT,
    is_active     BOOLEAN DEFAULT TRUE NOT NULL,
    last_login_at DATETIME,
    created_at    DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at    DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 2. Copy data from old table to new table
-- For existing users, set a default email based on username
INSERT INTO admin_user_new 
SELECT 
    id, 
    username, 
    username || '@example.com' as email,
    password_hash, 
    user_type_id, 
    is_active, 
    last_login_at, 
    created_at, 
    updated_at 
FROM admin_user;

-- 3. Drop the old table
DROP TABLE admin_user;

-- 4. Rename new table to original name
ALTER TABLE admin_user_new RENAME TO admin_user;

-- 5. Recreate indexes
CREATE INDEX idx_admin_user_username ON admin_user(username);
CREATE INDEX idx_admin_user_email ON admin_user(email);
CREATE INDEX idx_admin_user_user_type_id ON admin_user(user_type_id);

-- 6. Update the admin user's email to a proper one
UPDATE admin_user SET email = 'admin@example.com' WHERE username = 'admin';

-- 7. Make email required for new users
-- Note: SQLite doesn't support ALTER COLUMN, so we handle this in the application layer
