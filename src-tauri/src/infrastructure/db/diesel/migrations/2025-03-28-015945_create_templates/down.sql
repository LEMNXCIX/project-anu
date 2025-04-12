-- This file should undo anything in `up.sql`
-- Eliminar un proyecto específico
DELETE FROM projects;

-- Eliminar un archivo específico
DELETE FROM files;

-- Eliminar un template específico
DELETE FROM templates;

-- Eliminar un tipo específico
DELETE FROM types;

-- Eliminar una relación entre templates y tipos específica
DELETE FROM templates_types;