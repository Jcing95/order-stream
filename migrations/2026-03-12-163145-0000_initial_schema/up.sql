-- Enable uuid generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users (UUID pk for security)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'Visitor'
);

-- Categories
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL UNIQUE
);

-- Events
CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL
);

-- Products
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE RESTRICT,
    price DOUBLE PRECISION NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Orders
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    event_id INTEGER NOT NULL REFERENCES events(id) ON DELETE RESTRICT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Items (order line items)
CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    order_id INTEGER NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
    quantity INTEGER NOT NULL,
    price DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL DEFAULT 'Ordered'
);

-- Stations
CREATE TABLE stations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL UNIQUE,
    output_status TEXT NOT NULL
);

-- Station <-> Category junction (which categories this station handles)
CREATE TABLE station_categories (
    station_id INTEGER NOT NULL REFERENCES stations(id) ON DELETE CASCADE,
    category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    PRIMARY KEY (station_id, category_id)
);

-- Station input statuses junction (which item statuses feed into this station)
CREATE TABLE station_input_statuses (
    station_id INTEGER NOT NULL REFERENCES stations(id) ON DELETE CASCADE,
    status TEXT NOT NULL,
    PRIMARY KEY (station_id, status)
);

-- Settings (singleton row)
CREATE TABLE settings (
    id INTEGER PRIMARY KEY DEFAULT 1,
    active_event_id INTEGER REFERENCES events(id) ON DELETE SET NULL,
    CONSTRAINT single_row CHECK (id = 1)
);

INSERT INTO settings (id, active_event_id) VALUES (1, NULL);

-- Sessions
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expiry_date TIMESTAMPTZ NOT NULL
);

-- Indexes
CREATE INDEX idx_products_category ON products(category_id);
CREATE INDEX idx_items_order ON items(order_id);
CREATE INDEX idx_items_status ON items(status);
CREATE INDEX idx_sessions_expiry ON sessions(expiry_date);
