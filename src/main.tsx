import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { BrowserRouter } from "react-router-dom";
import "./index.css";
import { DirectoryProvider } from "@/context/directory_contex";
import { AppProvider } from "@/context/app_contex";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <AppProvider>
      <DirectoryProvider>
        <BrowserRouter>
          <App />
        </BrowserRouter>
      </DirectoryProvider>
    </AppProvider>
  </React.StrictMode>
);
