import { Route, Routes } from "react-router-dom";
import MainPage from "./pages/Main";
import { config } from "@/config/enviromentConfig";
import { ThemeProvider } from "@/components/theme-provider";
import Page from "./pages/page";
import { Toaster } from "./components/ui/toaster";
import { useAppContex } from "./hooks/use_app";
import { useEffect } from "react";
import { useApp } from "./context/app_contex";
import { FolderSelectPage } from "./pages/initial_page";

function App() {
  const { state, dispatch } = useApp();
  const { userConfig } = useAppContex();
  useEffect(() => {
    userConfig();
  }, []);

  // Deshabilitar esto cuando se produzca el build
  if (config.ENVIROMENT != "DEV") {
    document.addEventListener(
      "contextmenu",
      function (event) {
        event.preventDefault();
      },
      true
    );
    document.body.onselectstart = function () {
      return false;
    };
    document.body.oncontextmenu = function () {
      return false;
    };
  }

  return (
    <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
      {
      !state.config_user.ruta_base ? (
       <FolderSelectPage />
      ) : (
        <Page>
          <Routes>
            <Route path="/" element={<MainPage />} />
          </Routes>
        </Page>
      )}

      <Toaster />
    </ThemeProvider>
  );
}

export default App;
