import { useCallback, useEffect } from "react";
import { useApp } from "@/context/app_contex";
import { tauriService } from "@/services/tauri_service";
import { TauriResponse } from "@/types/tauri_response_types.d";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { DirectoryService } from "@/services/directory_service";
import { c } from "vite/dist/node/moduleRunnerTransport.d-CXw_Ws6P";

export const useAppContex = () => {
  const { dispatch } = useApp();
  const appWindow = getCurrentWindow();
  const userConfig = useCallback(async () => {
    const res = (await tauriService.exec_tauri_command(
      "load_config_command"
    )) as TauriResponse<any>;

    let entries = res.data;
    let proyects = await DirectoryService.listItemsByDirectory(
      entries.ruta_base
    );
    if (proyects) {
      entries = { ...entries, proyectos: proyects };
    }
    
    dispatch({ type: "SET_ITEMS", payload: entries });
  }, [dispatch]);

  const windowContex = async () => {
    const estaMaximizada = await appWindow.isMaximized();
    dispatch({
      type: "SET_WINDOW_PROP",
      payload: { is_maximized: estaMaximizada },
    });
  };
  return { userConfig, windowContex };
};
