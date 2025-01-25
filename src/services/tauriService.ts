import { invoke } from "@tauri-apps/api/core";
import { createTauriResponse, TauriResponse } from "../types/tauriResponse.d";
import UtilsService from "../services/UtilsService";

export const tauriService = {
  async exec_tauri_command<T>(
    command: string,
    args: any
  ): Promise<TauriResponse<T>> {
    try {
      console.log("Ejecutando comando: " + command);
      console.log(args);
      let tauriResponse = createTauriResponse();
      const response = (await invoke(command, args)) as string;
      //Validar si la respuesta es un json
      if (UtilsService.isJSON(response)) {
        tauriResponse = JSON.parse(response) as TauriResponse<T>;
      } else {
        if (response) {
          tauriResponse.success = true;
          tauriResponse.data = { result: response };
        }
      }

      return tauriResponse;
    } catch (error) {
      console.error("El comando: " + command + " no pudo ser ejecutado");
      let tauriResponse = createTauriResponse();
      tauriResponse.error = true;
      tauriResponse.message = "Error al ejecutar el comando: " + command;
      return tauriResponse;
    }
  },
};
