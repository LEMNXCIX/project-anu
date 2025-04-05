// pages/FolderSelectPage.js
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import React, { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { tauriService } from "@/services/tauri_service";
import { toast } from "@/hooks/use-toast";
import * as path from "@tauri-apps/api/path";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { Separator } from "@/components/ui/separator";

export function FolderSelectPage() {
  const [formData, setFormData] = useState({
    folderPath: "",
    files: [
      { ruta: "", tipo: [] },
      { ruta: "", tipo: [] },
      { ruta: "", tipo: [] },
      { ruta: "", tipo: [] },
    ],
    userName: "",
    userLastName: "",
    leaderName: "",
    leaderLastName: "",
  });

  const isWithinDownloads = async (selectedPath) => {
    try {
      const downloadsPath = await path.downloadDir();
      const normalizedDownloads = downloadsPath
        .replace(/\\+/g, "/")
        .toLowerCase();
      const normalizedSelected = selectedPath
        .replace(/\\+/g, "/")
        .toLowerCase();
      return normalizedSelected.startsWith(normalizedDownloads);
    } catch (error) {
      console.error("Error al validar la ruta:", error);
      return false;
    }
  };

  const handleFolderSelect = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Selecciona un directorio dentro de Descargas",
      });
      //if (selected /*&& (await isWithinDownloads(selected))*/) {
      if(selected) {
        setFormData((prev) => ({ ...prev, folderPath: selected }));
      } else {
        toast({
          title: "Ruta inválida",
          variant: "destructive",
          description: "Selecciona una carpeta dentro de Descargas.",
        });
      }
    } catch (error) {
      toast({
        title: "Error",
        variant: "destructive",
        description: "Error al seleccionar la carpeta.",
      });
    }
  };

  const handleFileSelect = async (index) => {
    try {
      const selected = await open({
        directory: false,
        multiple: false,
        title: "Selecciona un archivo",
      });
      
      //if (selected /*&& (await isWithinDownloads(selected))*/) {
      if(selected) {
        setFormData((prev) => {
          const newFiles = [...prev.files];
          newFiles[index].ruta = selected;
          return { ...prev, files: newFiles };
        });
      } else {
        toast({
          title: "Ruta inválida",
          variant: "destructive",
          description: "Selecciona un archivo dentro de Descargas.",
        });
      }
    } catch (error) {
      toast({
        title: "Error",
        variant: "destructive",
        description: "Error al seleccionar el archivo.",
      });
    }
  };

  const handleToggleChange = (index, value) => {
    setFormData((prev) => {
      const newFiles = [...prev.files];
      newFiles[index].tipo = value; // Actualizamos el array tipo con los valores seleccionados
      return { ...prev, files: newFiles };
    });
  };

  const validateForm = () => {
    // Validar ruta base
    if (!formData.folderPath) {
      toast({
        title: "Campo requerido",
        variant: "warning",
        description: "Por favor, selecciona un directorio base.",
      });
      return false;
    }

    // Validar que todas las rutas de archivos estén completas
    for (let i = 0; i < formData.files.length; i++) {
      if (!formData.files[i].ruta) {
        toast({
          title: "Campo requerido",
          variant: "warning",
          description: `Por favor, selecciona un archivo para el Archivo ${
            i + 1
          }.`,
        });
        return false;
      }
    }

    // Validar nombre y apellido del usuario
    if (!formData.userName || !formData.userLastName) {
      toast({
        title: "Campo requerido",
        variant: "destructive",
        description: "Por favor, ingresa el nombre y apellido del usuario.",
      });
      return false;
    }

    // Validar nombre y apellido del líder
    if (!formData.leaderName || !formData.leaderLastName) {
      toast({
        title: "Campo requerido",
        variant: "destructive",
        description:
          "Por favor, ingresa el nombre y apellido del líder de equipo.",
      });
      return false;
    }

    return true;
  };

  const handleSubmit = async (e) => {
    e.preventDefault();

    // Validar todos los campos antes de enviar
    if (!validateForm()) {
      return;
    }

    try {
      const configData = {
        ruta_base: formData.folderPath,
        archivos_plantillas: formData.files,
        usuario: {
          nombre: formData.userName,
          apellido: formData.userLastName,
        },
        lider: {
          nombre: formData.leaderName,
          apellido: formData.leaderLastName,
        },
      };
      console.log(configData);
      const res = await tauriService.exec_tauri_command("save_config_command", {
        data: configData,
      });
      if (res.success) {
        window.location.reload();
      } else {
        toast({
          title: "Error",
          variant: "destructive",
          description: res.message,
        });
      }
    } catch (error) {
      toast({
        title: "Error",
        variant: "destructive",
        description: "Error al guardar la configuración.",
      });
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen p-4 w-2/3 mx-auto">
      <h1 className="mt-4 font-bold text-4xl p-14 text-center">
        𒀭 Project ANU
      </h1>
      <h1 className="text-3xl font-bold p-10">
        !Empecemos con las configuraciones básicas!
      </h1>
      <Separator className="m-5" />
      <form onSubmit={handleSubmit} className="w-full max-w-lg space-y-6">
        {/* Directorio Base */}
        <div className="grid gap-2 pt-10">
          <Label htmlFor="folder-select">Directorio Base:</Label>
          <div className="flex gap-2">
            <Input
              id="folder-select"
              value={formData.folderPath}
              readOnly
              placeholder="No se ha seleccionado ninguna carpeta"
            />
            <Button type="button" onClick={handleFolderSelect}>
              Seleccionar
            </Button>
          </div>
        </div>

        {/* Nombre y apellido usuario */}
        <div className="grid gap-2">
          <Label>Desarrollador:</Label>
          <div className="flex gap-2">
            <Input
              placeholder="Nombre"
              value={formData.userName}
              onChange={(e) =>
                setFormData((prev) => ({ ...prev, userName: e.target.value }))
              }
            />
            <Input
              placeholder="Apellido"
              value={formData.userLastName}
              onChange={(e) =>
                setFormData((prev) => ({
                  ...prev,
                  userLastName: e.target.value,
                }))
              }
            />
          </div>
        </div>

        {/* Nombre y apellido líder */}
        <div className="grid gap-2 pb-10">
          <Label>Líder de equipo:</Label>
          <div className="flex gap-2">
            <Input
              placeholder="Nombre"
              value={formData.leaderName}
              onChange={(e) =>
                setFormData((prev) => ({ ...prev, leaderName: e.target.value }))
              }
            />
            <Input
              placeholder="Apellido"
              value={formData.leaderLastName}
              onChange={(e) =>
                setFormData((prev) => ({
                  ...prev,
                  leaderLastName: e.target.value,
                }))
              }
            />
          </div>
        </div>
        <Label>Plantillas para los archivos de los proyectos:</Label>
        <hr className="my-2" />
        {/* Inputs para archivos */}
        {formData.files.map((file, index) => (
          <div key={index} className="grid gap-2">
            <Label htmlFor={`file-${index}`}>Archivo {index + 1}:</Label>
            <div className="flex gap-2 items-center">
              <Input
                id={`file-${index}`}
                value={file.ruta}
                readOnly
                placeholder="No se ha seleccionado ningún archivo"
                className="flex-1"
              />
              <Button type="button" onClick={() => handleFileSelect(index)}>
                Seleccionar
              </Button>
              <ToggleGroup
                type="multiple"
                value={file.tipo}
                onValueChange={(value) => handleToggleChange(index, value)}
                className="flex gap-2"
              >
                <ToggleGroupItem value="nuevos" variant="outline">
                  Nuevos
                </ToggleGroupItem>
                <ToggleGroupItem value="bug" variant="outline">
                  Bug
                </ToggleGroupItem>
              </ToggleGroup>
            </div>
          </div>
        ))}

        {/* Botón Submit */}
        <hr className="p-5" />
        <Button type="submit" className="w-full mt-1">
          Guardar Configuración
        </Button>
      </form>
    </div>
  );
}

export default FolderSelectPage;
