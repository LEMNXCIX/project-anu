import React, { useState } from "react";
import {
    Alert,
    AlertDescription,
    AlertTitle,
  } from "@/components/ui/alert"
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input"
import { FolderPlusIcon, WandSparklesIcon } from "lucide-react"

import { FixName, NewProyect } from "@/resources/svgIcons";
import { tauriService } from "@/services/tauriService";
import { DefaultResult, TauriResponse, createTauriResponse } from "@/types/tauriResponse.d";
import { useListDirectory } from "@/hooks/useDirectory";

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
    try {
      e.preventDefault();
      const form = e.target;
      const formData = new FormData(form);

      const name = formData.get("projectName") as string;
      const response = await tauriService.exec_tauri_command(
        "create_directory_command",
        { name: name }
      );

      if (response.error) {
        submittedState.message = response.message;
        setSubmittedState({ ...submittedState, submitted: true });
        return;
      }

      //Se actualiza la lista de directorios
      listDirectory();

      submittedState.success = response.success;
      submittedState.message = response.message;
      setSubmittedState({ ...submittedState, submitted: true });
    } catch (error) {
      submittedState.success = false;
      submittedState.message = error;
      setSubmittedState({ ...submittedState, submitted: true });
    }

    setTimeout(() => {
      setSubmittedState(defaultSubmitState);
    }, 5000);
  };
  const getNewName = async () => {
    try {
      const form = document.querySelector("form");
      const formData = new FormData(form);
      const name = formData.get("projectName") as string;
      
      const response = await tauriService.exec_tauri_command(
        "format_name_project_command",
        { name: name }
      ) as TauriResponse<DefaultResult<string>>;
      const data = response.data ;

      navigator.clipboard.writeText(response.data.result);
      submittedState.success = response.success;
      submittedState.message =  response.message +"Se copio el nombre formateado al protapales:";
      setSubmittedState({ ...submittedState, submitted: true });
     
    } catch (error) {
        submittedState.error = true;
        submittedState.message =  "Error al formatear el nombre del proyecto: " + error;
      setSubmittedState({ ...submittedState, submitted: true });

    }
    setTimeout(() => {
        setSubmittedState(defaultSubmitState);
      }, 5000);
  };
  return (
    <>
      <form
        className="flex flex-row items-center"
        //validationBehavior="native"
        onSubmit={onSubmit}
      >
        <Input
          name="projectName"
          placeholder="Nombre proyecto Gitlab"
        />

        
            <Button type="submit" variant="outline" size="icon">
              <FolderPlusIcon />
            </Button>
         
          
            <Button variant="secondary" size="icon" onClick={getNewName}>
              <WandSparklesIcon />
            </Button>
          

      </form>

      <div className="relative inset-0 h-3">
        {submittedState.submitted && (
            <Alert>
            
            <AlertTitle>Heads up!</AlertTitle>
            <AlertDescription>
              You can add components to your app using the cli.
            </AlertDescription>
          </Alert>
        //   <Alert
        //     color={
        //       submittedState.error
        //         ? "danger"
        //         : submittedState.success
        //         ? "success"
        //         : "warning"
        //     }
        //     description={submittedState.message}
        //     title={
        //       submittedState.error
        //         ? "Error"
        //         : submittedState.success
        //         ? "Éxito"
        //         : "Advertencia"
        //     }
        //     variant="faded"
        //   />
        )}
      </div>

    </>
  );
}
