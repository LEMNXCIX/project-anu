import { ThemeProvider, useTheme } from "@/components/theme-provider";
import { useAppContex } from "./hooks/use_app";
import { useEffect, useState, useMemo } from "react";
import { useApp } from "./context/app_contex";
import { FolderSelectPage } from "./pages/initial_setup_page";
import "./index";
import { Button } from "./components/ui/button";
import { X, Maximize2, Minimize2, Minus } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { VITE_ENVIRONMENT } from "./config/enviroment_config";
import { Label, Separator } from "@radix-ui/react-dropdown-menu";
import { useListDirectory } from "./hooks/use_directory";
import { useDirectory } from "./context/directory_contex";
import { Routes, Route, useLocation, useNavigate } from "react-router-dom";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "./components/ui/sidebar";
import { AppSidebar } from "./components/app-sidebar";
import { SidebarLeft } from "./components/app_sidebar_left";
import MainPage from "./pages/main_page";
import DetailsProjects from "./pages/details_projects_page";
import { ScrollArea } from "@radix-ui/react-scroll-area";

function Application() {
  const { state } = useApp();
  const { userConfig, windowContex } = useAppContex();
  const appWindow = getCurrentWindow();
  const [isVisible, setIsVisible] = useState(false);
  const [loading, setLoading] = useState(true);
  const { setCurrentDirectory, setHistorialPath } = useListDirectory();
  const navigate = useNavigate();
  const location = useLocation();
  const { state: directoryState } = useDirectory();
  const { setTheme, theme } = useTheme();

  // Memorizamos valores derivados
  const isRootPath = useMemo(() => location.pathname === "/", [location.pathname]);
  const historialDirectorios = useMemo(() => directoryState.historialPath, [directoryState.historialPath]);

  // Depuramos cambios en los contextos
  useEffect(() => {
    console.log("useApp state cambió:", state);
  }, [state]);

  useEffect(() => {
    console.log("useDirectory state cambió:", directoryState);
  }, [directoryState]);

  useEffect(() => {
    const devMode = VITE_ENVIRONMENT === "DEV";
    console.log("isDevMode establecido:", devMode);
    setLoading(false);
    userConfig();
    const titlebar = document.getElementById("titlebar");
    const btnMinimize = document.getElementById("titlebar-min");
    const btnMaximize = document.getElementById("titlebar-max");
    const btnClose = document.getElementById("titlebar-close");
    return () => {
      if (titlebar) titlebar.removeEventListener("mousedown", () => {});
      if (btnMinimize) btnMinimize.removeEventListener("click", () => {});
      if (btnMaximize) btnMaximize.removeEventListener("click", () => {});
      if (btnClose) btnClose.removeEventListener("click", () => {});
    };
  }, []);

  console.log("Application renderizado");
  return (
    <SidebarProvider className="mt-9">
      <SidebarLeft />
      <SidebarInset>
        <header className="flex h-14 shrink-0 items-center gap-2">
          <div className="flex flex-1 items-center titlebar" id="titlebar">
            <SidebarTrigger className="titlebar-button" />
            <div data-tauri-drag-region className="titlebar-space" />
            <Button
              type="button"
              variant="ghost"
              size="sm"
              id="titlebar-min"
              className="titlebar-button"
              onClick={() => {
                appWindow.minimize();
              }}
            >
              <Minus />
            </Button>
            <Button
              type="button"
              variant="ghost"
              size="sm"
              id="titlebar-max"
              className="titlebar-button"
              onClick={() => {
                windowContex();
                appWindow.toggleMaximize();
                setIsVisible(state.window.is_maximized);
              }}
            >
              {isVisible ? <Minimize2 /> : <Maximize2 />}
            </Button>
            <Button
              type="button"
              variant="ghost"
              size="sm"
              id="titlebar-close"
              className="titlebar-button"
              onClick={() => {
                appWindow.close();
              }}
            >
              <X />
            </Button>
            <Separator className="mr-2 data-[orientation=vertical]:h-4" />
          </div>
        </header>
        {loading && <>Cargando</>}
        {!state.config_user?.ruta_base ? (
          <FolderSelectPage />
        ) : (
          <ScrollArea
            className="flex w-full h-11 overflow-y-scroll"
            id="contendedor"
          >
            <Routes>
              <Route path="/" element={<MainPage />} />
              <Route
                path="/details-projects/:id"
                element={<DetailsProjects />}
              />
            </Routes>
          </ScrollArea>
        )}
      </SidebarInset>
    </SidebarProvider>
  );
}

export default Application;