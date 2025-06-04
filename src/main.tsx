import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter } from "react-router-dom";
import "./index.css";
import { DirectoryProvider } from "@/context/directory_contex";
import { AppProvider } from "@/context/app_contex";
import Application from "./App";
import { Toaster } from "sonner";
import { ThemeProvider, useTheme } from "./components/theme-provider";

const RootComponent = () => {
  const { theme } = useTheme(); // Ahora dentro de un componente funcional
  console.log(theme);
 
  return<Toaster richColors theme={theme} />;
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <AppProvider>
      <DirectoryProvider>
        <BrowserRouter>
          <ThemeProvider defaultTheme="light" storageKey="vite-ui-theme">
            <Application />
           <RootComponent/>
          </ThemeProvider>
        </BrowserRouter>
      </DirectoryProvider>
    </AppProvider>
  </React.StrictMode>
);
