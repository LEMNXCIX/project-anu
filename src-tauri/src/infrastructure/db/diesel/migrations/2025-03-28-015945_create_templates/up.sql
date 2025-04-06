-- Your SQL goes here
CREATE TABLE templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT, -- Identificador único para cada plantilla
    name TEXT NOT NULL,                   -- Nombre de la plantilla
    path TEXT NOT NULL,                   -- Ruta en el sistema de archivos
    type TEXT NOT NULL,                   -- Tipo de la plantilla
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Fecha de creación
    modified_at TIMESTAMP                 -- Fecha de última modificación
);