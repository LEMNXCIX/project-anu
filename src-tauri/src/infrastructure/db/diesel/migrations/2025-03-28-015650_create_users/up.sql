-- Your SQL goes here
-- Tabla users
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    main_path TEXT,
    user_type TEXT NOT NULL, -- 'user' o 'representative'
    status TEXT NOT NULL, -- Ej.: 'active', 'inactive'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Tabla user_representatives
CREATE TABLE user_representatives (
    user_id INTEGER NOT NULL,
    representative_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (representative_id) REFERENCES users (id),
    PRIMARY KEY (user_id, representative_id)
);
