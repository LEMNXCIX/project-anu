
-- Tabla projects
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    url TEXT,
    path TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    status TEXT NOT NULL, -- Ej.: 'active', 'completed'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users (id)
);

-- Tabla files
CREATE TABLE files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    project_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    template_type_id INTEGER,
    status TEXT NOT NULL, -- Ej.: 'active', 'inactive'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects (id),
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (template_type_id) REFERENCES templates_types (id)
);

-- Tabla templates
CREATE TABLE templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Tabla types
CREATE TABLE types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Tabla templates_types
CREATE TABLE templates_types (
    template_id INTEGER NOT NULL,
    type_id INTEGER NOT NULL,
    FOREIGN KEY (template_id) REFERENCES templates (id),
    FOREIGN KEY (type_id) REFERENCES types (id),
    PRIMARY KEY (template_id, type_id)
);