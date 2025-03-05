import { useCallback, useEffect } from "react";
import { useApp } from "@/context/app_contex";
import { tauriService } from "@/services/tauri_service";
import { TauriResponse } from "@/types/tauri_response_types.d";

export const useAppContex = () => {
  const { dispatch } = useApp();

  const userConfig = useCallback(async () => {
    const res = (await tauriService.exec_tauri_command(
      "load_config_command",
    )) as TauriResponse<any>;

    let entries = res.data;
    dispatch({ type: "SET_ITEMS", payload: entries });
  }, [dispatch]);

  return { userConfig };
};
