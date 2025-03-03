import { invoke } from "@tauri-apps/api/core";
import { createTauriResponse, TauriResponse } from "../types/tauriResponse.d";
import utilsService from "@/services/UtilsService";

/**
 * Servicio para ejecutar comandos Tauri y manejar las respuestas.
 */
export const tauriService = {
  /**
   * Ejecuta un comando Tauri con los argumentos proporcionados.
   *
   * @param command - El nombre del comando Tauri a ejecutar.
   * @param args - Argumentos opcionales para el comando (puede ser cualquier tipo serializable).
   * @returns Promesa con la respuesta TauriResponse<T>, que incluye estado, datos, y mensajes.
   */
  async exec_tauri_command<T>(
    command: string,
    args?: any
  ): Promise<TauriResponse<T>> {
    let tauriResponse = createTauriResponse<T>();

    try {
      console.log(`Ejecutando comando: ${command}`, args);

      // Invocar el comando Tauri
      const response = await invoke<T>(command, args);

      // Tauri devuelve objetos serializados directamente, así que asumimos que es un TauriResponse
      if (response && typeof response === "object") {
        tauriResponse = response as unknown as TauriResponse<T>;
      } else if (utilsService.isJSON(response as string)) {
        // Fallback para cadenas JSON (por compatibilidad con el backend actual)
        tauriResponse = JSON.parse(response as string) as TauriResponse<T>;
      } else if (response) {
        // Caso donde la respuesta es un valor simple (p.ej., string)
        tauriResponse.success = true;
        tauriResponse.data = { result: response } as T;
      }

      return tauriResponse;
    } catch (error: any) {
      console.error(`El comando: ${command} no pudo ser ejecutado: ${error.message || error}`);
      tauriResponse.error = true;
      tauriResponse.message = error.message || `Error al ejecutar el comando: ${command}`;
      tauriResponse.error_details = [error.message || error.toString()];

      return tauriResponse;
    }
  },
};