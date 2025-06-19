-- This file should undo anything in `up.sql`
-- Eliminar un proyecto específico
DELETE FROM projects;


-- Eliminar un template específico
DELETE FROM templates;

-- Eliminar un tipo específico
DELETE FROM types;

-- Eliminar un archivo específico
DELETE FROM files;

DROP TABLE IF EXISTS file_formats;
