import { useCallback, useEffect } from "react";
import { useDirectory } from "@/context/directoryContex";
import { tauriService } from "@/services/tauriService";
import { TauriResponse } from "@/types/tauriResponse";
import { ListDirectory } from "@/types/directory";
import { useApp } from "@/context/app_contex";

export const useListDirectory = () => {
  const { dispatch } = useDirectory();
  const { state } = useApp();

  const listDirectory = useCallback(async () => {
    const res = (await tauriService.exec_tauri_command(
      "list_directory_command",
      { name: state.config_user.ruta_base}
    )) as TauriResponse<ListDirectory>;

    let entries = res.data.entries;
    dispatch({ type: "SET_ITEMS", payload: entries });
  }, [dispatch]);

  return { listDirectory };
};
