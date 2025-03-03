import React, { useState } from "react";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { FolderPlusIcon, WandSparklesIcon } from "lucide-react";
import { useToast } from "@/hooks/use-toast";
import { tauriService } from "@/services/tauriService";
import {
  DefaultResult,
  TauriResponse,
  createTauriResponse,
} from "@/types/tauriResponse.d";
import { useListDirectory } from "@/hooks/useDirectory";

export default function CreateNewProyect() {
  type submitState = TauriResponse<any> & {
    submitted: boolean;
  };
  const { toast } = useToast();
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
      toast({
        title: response.success && !response.error ? "Éxito" : "Alerta",
        variant: response.error
          ? "destructive"
          : response.success
          ? "success"
          : "warning",
        description: response.message,
      });
    }
  };
  const getNewName = async () => {
    let response: TauriResponse<DefaultResult<string>>;
    try {
      const form = document.querySelector("form");
      const formData = new FormData(form);
      const name = formData.get("projectName") as string;

      response = (await tauriService.exec_tauri_command(
        "format_name_project_command",
        { name: name }
      )) as TauriResponse<DefaultResult<string>>;
      console.log(response);
      if (response.success) {
        navigator.clipboard.writeText(response.data.result);
        response.message = "Se copio el nombre formateado al portapales";
      }
    } catch (error) {
      response.error = true;
      response.message = "Error al formatear el nombre del proyecto: " + error;
    }
    toast({
      title: response.success && !response.error ? "Éxito" : "Alerta",
      variant: response.error
        ? "destructive"
        : response.success
        ? "success"
        : "warning",
      description: response.message,
    });
  };
  return (
    <>
      <form className="flex flex-row items-center p-10" onSubmit={onSubmit}>
        <Input
          name="projectName"
          placeholder="Nombre del proyecto en Gitlab"
          className="m-2"
        />

        <Button type="submit" variant="ghost" size="default">
          <FolderPlusIcon />
        </Button>

        <Button
          type="button"
          variant="ghost"
          size="default"
          onClick={getNewName}
        >
          <WandSparklesIcon />
        </Button>
      </form>

      <div className="relative inset-0 h-3"></div>
    </>
  );
}
