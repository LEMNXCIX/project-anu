import { useAppContex } from "./hooks/use_app";
import { useEffect, useState } from "react";
import { useApp } from "./context/app_contex";
import { FolderSelectPage } from "./pages/initial_setup_page";
import "./index";
import { Button } from "./components/ui/button";
import {
  X,
  Maximize2,
  Minimize2,
  Minus,
  PanelRightOpen,
  PanelRightClose,
} from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { VITE_ENVIRONMENT } from "./config/enviroment_config";
import { Separator } from "@radix-ui/react-dropdown-menu";
import { Routes, Route, useLocation } from "react-router-dom";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "./components/ui/sidebar";
import { SidebarLeft } from "./components/app_sidebar_left";
import MainPage from "./pages/main_page";
import DetailsProjects from "./pages/details_projects_page";
import { ScrollArea } from "@radix-ui/react-scroll-area";
import { CustomRightSidebar } from "./components/custom_sidebar";

function Application() {
  const { state } = useApp();
  const { userConfig, windowContex } = useAppContex();
  const appWindow = getCurrentWindow();
  const [isVisible, setIsVisible] = useState(false);
  const [loading, setLoading] = useState(true);
  const [isSidebarOpen, setIsSidebarOpen] = useState(false);
  const location = useLocation();

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

  // Depuramos el estado de isSidebarOpen
  useEffect(() => {
    console.log("isSidebarOpen cambió:", isSidebarOpen);
  }, [isSidebarOpen]);

  console.log("Application renderizado");
  return (
    <SidebarProvider className="mt-9">
      <SidebarLeft />
      <SidebarInset className="relative w-full">
        <div className="flex flex-col w-full">
          <header className="flex h-14 shrink-0 items-center gap-2 z-50">
            <div className="flex flex-1 items-center titlebar" id="titlebar">
              <SidebarTrigger className="titlebar-button" />
              <div data-tauri-drag-region className="titlebar-space" />
              {location.pathname.startsWith("/details-projects") && (
                <>
                  <Button
                    type="button"
                    variant="ghost"
                    size="sm"
                    className="titlebar-button"
                    onClick={() => setIsSidebarOpen(!isSidebarOpen)}
                    aria-label={
                      isSidebarOpen
                        ? "Close sidebar"
                        : "Open sidebar"
                    }
                  >
                    {isSidebarOpen ? <PanelRightClose /> : <PanelRightOpen />}
                  </Button>
                  <Separator className="mr-2 data-[orientation=vertical]:h-4" />
                </>
              )}
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
                  element={
                    <div className="relative w-full h-full">
                      {/* Contenedor para el blur y cerrar al hacer clic fuera */}
                      <div
                        className="transition-all duration-300"
                        style={{
                          filter: isSidebarOpen ? "blur(2px)" : "none", // Aplicamos blur directamente
                        }}
                        onClick={() => {
                          if (isSidebarOpen) setIsSidebarOpen(false);
                        }}
                      >
                        <DetailsProjects />
                      </div>
                      <CustomRightSidebar
                        isOpen={isSidebarOpen}
                        onToggle={() => setIsSidebarOpen(!isSidebarOpen)}
                      />
                    </div>
                  }
                />
              </Routes>
            </ScrollArea>
          )}
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}

export default Application;
