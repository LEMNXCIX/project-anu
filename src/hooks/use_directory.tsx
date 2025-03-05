import { useCallback, useEffect } from "react";
import { useDirectory } from "@/context/directory_contex";
import { tauriService } from "@/services/tauri_service";
import { TauriResponse } from "@/types/tauri_response_types.d";
import { DirEntry, ListDirectory } from "@/types/directory_types";
import { useApp } from "@/context/app_contex";

export const useListDirectory = () => {
  const { state: directoryState, dispatch } = useDirectory();
  const { state } = useApp();

  const listDirectory = useCallback(
    async (path?: string) => {

      const res = (await tauriService.exec_tauri_command(
        "list_directory_command",
        { name: path ? path : state.config_user.ruta_base }
      )) as TauriResponse<ListDirectory>;

      let entries = res.data && res.data.entries ? res.data.entries : [];
      dispatch({ type: "SET_ITEMS", payload: entries });
    },
    [dispatch]
  );

  const setCurrentDirectory = (directory?: DirEntry) => {
    if (directory) {
      dispatch({ type: "SET_CURRENT_DIRECTORY", payload: directory });
      dispatch({ type: "ADD_HISTORIAL_PATH", payload: directory });
      listDirectory(directory.path);
    } else {
      dispatch({ type: "SET_INITIAL_STATE" });
      listDirectory();
    }
  };

  const setHistorialPath = (directory: DirEntry): void => {
    const currentHistorial = directoryState.historialPath;
    const index = currentHistorial.findIndex(item => item.name === directory.name);
    
    const newCurrentHistorial: DirEntry[] = index === -1 
        ? [...currentHistorial] 
        : currentHistorial.slice(0, index );
    dispatch({ type: "SET_HISTORIAL_PATH", payload: newCurrentHistorial });
    setCurrentDirectory(directory)
};
  return { listDirectory, setCurrentDirectory, setHistorialPath };
};
