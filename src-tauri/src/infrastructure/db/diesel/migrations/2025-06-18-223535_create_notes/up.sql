
-- Relación entre archivos y proyectos
CREATE TABLE project_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    file_id INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'activo' CHECK (status IN ('activo', 'inactivo')),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    modified_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (file_id) REFERENCES files(id)
);