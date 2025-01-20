import React, { useState } from "react";
import {
  Alert,
  Button,
  ButtonGroup,
  Form,
  Input,
  Spacer,
  Tooltip,
} from "@heroui/react";
import { FixName, NewProyect } from "../resources/svgIcons";
import { error } from "console";
import { invoke } from "@tauri-apps/api/core";

export default function CreateNewProyect() {
  type submitState = {
    submitted: false;
    success: false;
    error: false;
    message: "";
  };

  const defaultSubmitState = {
    submitted: false,
    success: false,
    error: false,
    message: "",
  };
  const [submitted, setSubmitted] = useState(defaultSubmitState);

  const onSubmit = async (e: any) => {
    e.preventDefault();
    var newSubmitState = defaultSubmitState;

    const form = e.target;
    const formData = new FormData(form);

    const name = formData.get("projectName") as string;
    const res = await invoke("create_directory", { path: name });
    if (name === "") {
      newSubmitState.success = false;
      newSubmitState.message = "El nombre del proyecto no puede estar vacío";
    }
    newSubmitState.success = true;
    newSubmitState.message = "Se creo el directorio para el proyecto";
    setSubmitted({ ...newSubmitState, submitted: true });
    console.log("Nombre del proyecto formateado:", res);
    setTimeout(() => {
      setSubmitted(defaultSubmitState);
    }, 5000);
  };
  const getNewName = async () => {
    try {
      const form = document.querySelector("form");
      const formData = new FormData(form);
      const name = formData.get("projectName") as string;
      const res = await invoke("format_name_project", { name: name });

      navigator.clipboard.writeText(res.toString());
      var newSubmitState = defaultSubmitState;
      newSubmitState.success = true;
      newSubmitState.message = "Se copio el nombre formateado al protapales:";
      setSubmitted({ ...newSubmitState, submitted: true });
      setTimeout(() => {
        setSubmitted(defaultSubmitState);
      }, 5000);
    } catch (error) {
      var newSubmitState = defaultSubmitState;
      newSubmitState.error = true;
      newSubmitState.message = "Error al formatear el nombre del proyecto: " + error;
      setSubmitted({ ...newSubmitState, submitted: true });
      setTimeout(() => {
        setSubmitted(defaultSubmitState);
      }, 5000);
    }
  };
  return (
    <>
      <form
        className="flex flex-row items-center"
        //validationBehavior="native"
        onSubmit={onSubmit}
      >
        <Input name="projectName" placeholder="Nombre proyecto Gitlab" />
        <ButtonGroup className="pl-2">
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
      <div className="relative inset-0  h-3">
        {submitted.submitted && (
          <Alert
            color={
              submitted.error
                ? "danger"
                : submitted.success
                ? "success"
                : "warning"
            }
            description={submitted.message}
            title={
              submitted.error
                ? "Error"
                : submitted.success
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
