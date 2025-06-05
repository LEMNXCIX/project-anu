import React, { useState } from "react";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { FolderPlusIcon, WandSparklesIcon } from "lucide-react";
import { tauriService } from "@/services/tauri_service";
import {
  DefaultResult,
  TauriResponse,
  createTauriResponse,
} from "@/types/tauri_response_types.d";
import { useListDirectory } from "@/hooks/use_directory";
import { toast } from "sonner";
export default function CreateNewProyect() {
  type submitState = TauriResponse<any> & {
    submitted: boolean;
  };

  const tauriResponse = createTauriResponse();
  const defaultSubmitState: submitState = {
    ...tauriResponse,
    submitted: false,
  };
  const [submittedState, setSubmittedState] = useState(defaultSubmitState);

  const { listDirectory } = useListDirectory();
  const onSubmit = async (e: any) => {
    let response: TauriResponse<DefaultResult<string>>;
    try {
      e.preventDefault();
      const form = e.target;
      const formData = new FormData(form);

      const name = formData.get("projectName") as string;
      response = await tauriService.exec_tauri_command(
        "create_directory_command",
        { name: name }
      );

      //Se actualiza la lista de directorios
      if (response.success) {
      
        listDirectory();
      }
    } catch (error) {
      response.error = false;
      response.message = error;
    }
    if (!response.success) {
      if (response.success && !response.error) {
        toast.success("Exito", { description: response.message });
      } else if (!response.success && !response.error) {
        toast.warning("Alerta", { description: response.message });
      } else {
        toast.error("Error", { description: response.message });
      }
    }
  };
  const getNewName = async () => {
    let response: TauriResponse<DefaultResult<string>>;
    try {
      const form = document.querySelector("form");
      const formData = new FormData(form);
      const name = formData.get("projectName") as string;
      if (name === "") {
        return;
      }
      response = (await tauriService.exec_tauri_command(
        "format_name_project_command",
        { name: name }
      )) as TauriResponse<DefaultResult<string>>;

      if (response.success) {
        navigator.clipboard.writeText(response.data.result);
        response.message = "Se copio el nombre formateado al portapales";
      }
    } catch (error) {
      response.error = true;
      response.message = "Error al formatear el nombre del proyecto: " + error;
    }

    if (response.success && !response.error) {
      toast.success("Exito", { description: response.message });
    } else if (!response.success && !response.error) {
      toast.warning("Alerta", { description: response.message });
    } else {
      toast.error("Error", { description: response.message });
    }
  };
  return (
    <>
      <h4 className=" font-semibold m-1">Crea un nuevo proyecto</h4>
      <form className="flex flex-row items-center" onSubmit={onSubmit}>
        <Input
          name="projectName"
          placeholder="Nombre del proyecto en Gitlab"
          className="m-1"
        />

        <Button type="submit" variant="ghost" size="icon">
          <FolderPlusIcon />
        </Button>

        <Button type="button" variant="ghost" size="icon" onClick={getNewName}>
          <WandSparklesIcon />
        </Button>
      </form>

      <div className="relative inset-0 h-3"></div>
    </>
  );
}
