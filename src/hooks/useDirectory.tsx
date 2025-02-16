import { useCallback, useEffect } from "react";
import { useDirectory } from "../context/directoryContex";
import { tauriService } from "../services/tauriService";
import { TauriResponse } from "../types/tauriResponse";
import { ListDirectory } from "../types/directory";
// import { setTimeout } from "node:timers/promises";

export const useListDirectory = () => {
  const { dispatch } = useDirectory();

  const listDirectory = useCallback(async () => {
    const res = (await tauriService.exec_tauri_command(
      "list_directory_command",
      { name: "" }
    )) as TauriResponse<ListDirectory>;
    const update = (await tauriService.exec_tauri_command(
      "check_updates_comand"
    )) as TauriResponse<string>;
    console.log(update);
    if (update.data) {
        console.log("Hay una actualización disponible");
        console.log(update.data);
        window.setTimeout(async () => {
            const applyUpdate = await tauriService.exec_tauri_command(
              "apply_update_comand"
            );
            console.log(applyUpdate);
          }, 1000);
        
    }
    let entries = res.data.entries;
    dispatch({ type: "SET_ITEMS", payload: entries });
  }, [dispatch]);

  return { listDirectory };
};
