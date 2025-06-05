import { DirEntry, ListDirectory } from "@/types/directory_types";
import { TauriResponse } from "@/types/tauri_response_types.d";
import { tauriService } from "@/services/tauri_service";

export const DirectoryService = {
  async createDirectory(name: string): Promise<any> {},

  async listItemsByDirectory(name?: string): Promise<DirEntry[]> {
    try {
      if (!name) {
        name = "";
      }
      console.log(name)
      let response = (await tauriService.exec_tauri_command(
        "list_directory_command",
        {
          name,
        }
      )) as TauriResponse<ListDirectory>;

      if (response.error) {
        return [];
      }
      return response.data.entries;
    } catch (error) {
      console.error("Error al listar los items del directorio");
    }
  },
};
