import { DirEntry, ListDirectory } from "@/types/directory.d";
import { TauriResponse } from "@/types/tauriResponse";
import { tauriService } from "@/services/tauriService";

export const DirectoryService = {
  async createDirectory(name: string): Promise<any> {},

  async listItemsByDirectory(name?: string): Promise<DirEntry[]> {
    try {
      if (!name) {
        name = "";
      }
      let response = await tauriService.exec_tauri_command("list_items_by_directory", {
        name,
      }) as TauriResponse<ListDirectory>;

      if (response.error) {
        console.error(response.message);
        return [];
      }
      return response.data.entries;

    } catch (error) {
      console.error("Error al listar los items del directorio");
    }
  },
};
