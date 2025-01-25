import React, { useState } from "react";
import {
  Alert,
  Button,
  ButtonGroup,
  Input,
  Spacer,
  Tooltip,
} from "@heroui/react";
import { FixName, NewProyect } from "../resources/svgIcons";
import { tauriService } from "../services/tauriService";
import { invoke } from "@tauri-apps/api/core";
import { DefaultResult, TauriResponse, createTauriResponse } from "../types/tauriResponse.d";
import { useListDirectory } from "../hooks/useDirectory";

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
      console.log(data.result);
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
          radius="sm"
        />
        <ButtonGroup className="pl-2" radius="sm">
          <Tooltip showArrow content="Crear nuevo proyecto">
            <Button type="submit" isIconOnly>
              <NewProyect />
            </Button>
          </Tooltip>
          <Tooltip showArrow content="Formatear nombre de proyecto">
            <Button isIconOnly onPress={getNewName}>
              <FixName />
            </Button>
          </Tooltip>
        </ButtonGroup>
      </form>
      <Spacer y={5} />
      <div className="relative inset-0 h-3">
        {submittedState.submitted && (
          <Alert
            color={
              submittedState.error
                ? "danger"
                : submittedState.success
                ? "success"
                : "warning"
            }
            description={submittedState.message}
            title={
              submittedState.error
                ? "Error"
                : submittedState.success
                ? "Éxito"
                : "Advertencia"
            }
            variant="faded"
          />
        )}
      </div>
      <Spacer y={4} />
    </>
  );
}
