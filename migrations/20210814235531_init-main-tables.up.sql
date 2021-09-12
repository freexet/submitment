CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(24) PRIMARY KEY,
    username VARCHAR(256) UNIQUE NOT NULL,
    password_hash VARCHAR(128) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TRIGGER set_user_updated_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

CREATE TABLE IF NOT EXISTS roles (
    key VARCHAR(64) PRIMARY KEY,
    name VARCHAR(64) NOT NULL
);

CREATE TABLE IF NOT EXISTS user_roles (
    user_id VARCHAR(24) REFERENCES users(id),
    role_key VARCHAR(64) REFERENCES roles(key),
    PRIMARY KEY (user_id, role_key)
);

CREATE TABLE IF NOT EXISTS reviewers (
    user_id VARCHAR(24) PRIMARY KEY REFERENCES users(id),
    name VARCHAR(256),
    email VARCHAR(256) NOT NULL,
    phone_number VARCHAR(16),
    is_verified BOOLEAN DEFAULT FALSE,
    profile_pict_url TEXT
);

CREATE TABLE IF NOT EXISTS submissions (
    id VARCHAR(24) PRIMARY KEY,
    user_id VARCHAR(24) REFERENCES users(id),
    reviewer_id VARCHAR(24) REFERENCES reviewers(user_id),
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    score DOUBLE PRECISION,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE TRIGGER set_submission_updated_timestamp
BEFORE UPDATE ON submissions
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();