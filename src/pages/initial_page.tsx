// pages/FolderSelectPage.js
import { Button } from "@/components/ui/button";
import { Label } from "@radix-ui/react-label"; // Corregimos la importación
import React, { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog"; // Importamos la función open
import { tauriService } from "@/services/tauriService";
import { data } from "react-router-dom";
import { toast } from "@/hooks/use-toast";
// Para invocar comandos de Rust si es necesario

export function FolderSelectPage() {
  // Renombré a FolderSelectPage para que coincida con el comentario
  const [folderPath, setFolderPath] = useState("");

  // Función para abrir el diálogo de selección de carpeta
  const handleFolderSelect = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Selecciona un directorio para tus proyectos",
      });

      if (selected) {
        setFolderPath(selected);
        let ruta = { ruta_base: selected };
        const res = await tauriService.exec_tauri_command(
          "save_config_command",
          { data: ruta }
        );
        //recargar la pagina
        if (res.success) {
          window.location.reload();
        } else {
          toast({
            title: "Error",
            variant: "destructive",
            description: res.message,
          });
        }
      }
    } catch (error) {
      console.error("Error al seleccionar carpeta:", error);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen">
      <h1 className="mt-4 font-bold text-4xl p-14 text-center">
        <Button variant="ghost" size="icon" className="text-4xl m-2">
          𒀭
        </Button>
        Project ANU
      </h1>
      <h1 className="text-2xl font-bold mb-4 p-5">Empecemos</h1>
      <div className="grid w-full max-w-sm items-center gap-1.5">
        <Label htmlFor="folder-select">
          Directorio raiz para tus proyectos
        </Label>
        <div className="flex gap-2">
          <input
            id="folder-select"
            type="text"
            value={folderPath}
            readOnly
            placeholder="No se ha seleccionado ninguna carpeta"
            className="flex-1 p-2 border rounded"
          />
          <Button onClick={handleFolderSelect}>Seleccionar Carpeta</Button>
        </div>
      </div>
      {folderPath && (
        <p className="mt-4 text-green-600">
          Carpeta seleccionada: {folderPath}
        </p>
      )}
    </div>
  );
}

export default FolderSelectPage;
