
-- Tabla projects
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    url TEXT,
    path TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    type_project_id INTEGER NOT NULL,
    priority_level INTEGER CHECK (priority_level BETWEEN 1 AND 10),
    status TEXT NOT NULL DEFAULT 'activo' CHECK (status IN ('activo', 'inactivo')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    start_date DATE DEFAULT CURRENT_DATE,
    end_date DATE,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (type_project_id) REFERENCES types_projects (id)
);
-- Tabla templates
CREATE TABLE templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'activo' CHECK (status IN ('activo', 'inactivo')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (file_id) REFERENCES files (id)
);

-- Tabla types
CREATE TABLE types_projects (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    alias TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'activo' CHECK (status IN ('activo', 'inactivo')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Tabla templates_types
INSERT INTO types_projects (alias, name, description)
VALUES
    ('bug', 'Bug', 'Plantilla destinada a reportar errores funcionales o técnicos dentro del proyecto.'),
    ('nuevo', 'Nuevo', 'Plantilla para registrar nuevas características, módulos o ideas a implementar.'),
    ('mant', 'Mantenimiento', 'Plantilla pensada para registrar actividades de mantenimiento preventivo, correctivo o mejoras internas no visibles.');
-- Tabla de formatos de archivo
CREATE TABLE file_formats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    extension TEXT NOT NULL,
    mime_type TEXT,
    editable BOOLEAN DEFAULT 1,
    previewable BOOLEAN DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'activo' CHECK (status IN ('activo', 'inactivo')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO file_formats (name, extension, mime_type, editable, previewable)
VALUES
    ('Markdown', '.md', 'text/markdown', 1, 1),
    ('PDF', '.pdf', 'application/pdf', 0, 1),
    ('DOCX', '.docx', 'application/vnd.openxmlformats-officedocument.wordprocessingml.document', 0, 0),
    ('Excel', '.xlsx', 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet', 0, 0);
-- Tabla de archivos
CREATE TABLE files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    filename TEXT NOT NULL,
    file_format_id INTEGER NOT NULL,
    relative_path TEXT NOT NULL,
    content TEXT,
    status TEXT NOT NULL DEFAULT 'activo' CHECK (status IN ('activo', 'inactivo')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (file_format_id) REFERENCES file_formats(id)
);

