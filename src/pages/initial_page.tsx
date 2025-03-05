// pages/FolderSelectPage.js
import { Button } from "@/components/ui/button";
import { Label } from "@radix-ui/react-label";
import React, { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { tauriService } from "@/services/tauri_service";
import { toast } from "@/hooks/use-toast";
import * as path from "@tauri-apps/api/path"; // Importamos la API de path

export function FolderSelectPage() {
  const [folderPath, setFolderPath] = useState("");

  // Función para validar si la ruta está dentro del directorio de descargas
  const isWithinDownloads = async (selectedPath) => {
    try {
      // Obtenemos la ruta del directorio de Descargas
      const downloadsPath = await path.downloadDir();
      // Normalizamos las rutas para comparación
      const normalizedDownloads = downloadsPath
        .replace(/\\+/g, "/")
        .toLowerCase();
      const normalizedSelected = selectedPath
        .replace(/\\+/g, "/")
        .toLowerCase();

      // Verificamos si la ruta seleccionada está dentro de Descargas
      return normalizedSelected.startsWith(normalizedDownloads);
    } catch (error) {
      console.error("Error al validar la ruta:", error);
      return false;
    }
  };

  // Función para abrir el diálogo de selección de carpeta
  const handleFolderSelect = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Selecciona un directorio dentro de Descargas",
      });

      if (selected) {
        // Validamos si la carpeta está dentro de Descargas
        const isValid = await isWithinDownloads(selected);

        if (isValid) {
          setFolderPath(selected);
          let ruta = { ruta_base: selected };
          const res = await tauriService.exec_tauri_command(
            "save_config_command",
            { data: ruta }
          );

          if (res.success) {
            window.location.reload();
          } else {
            toast({
              title: "Error",
              variant: "destructive",
              description: res.message,
            });
          }
        } else {
          toast({
            title: "Ruta inválida",
            variant: "destructive",
            description:
              "Por favor, selecciona una carpeta dentro del directorio de Descargas.",
          });
        }
      }
    } catch (error) {
      toast({
        title: "Error",
        variant: "destructive",
        description: "Ocurrió un error al seleccionar la carpeta.",
      });
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen">
      <h1 className="mt-4 font-bold text-4xl p-14 text-center">
        𒀭 Project ANU
      </h1>
      <h1 className="text-2xl font-bold mb-4 p-5">Empecemos seleccionado un directorio para tus proyectos</h1>
      <div className="grid w-full max-w-sm items-center gap-1.5">
        <Label htmlFor="folder-select">
            <span className="text-lg">Directorio:</span>
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
          <Button className="flex-1 p-2 border rounded h-auto" onClick={handleFolderSelect}>Seleccionar</Button>
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
