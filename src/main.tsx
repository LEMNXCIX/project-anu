import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { BrowserRouter } from "react-router-dom";
import "./index.css";
import { DirectoryProvider } from "@/context/directoryContex";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <DirectoryProvider>
      <BrowserRouter>
        <App />
      </BrowserRouter>
    </DirectoryProvider>
  </React.StrictMode>
);
