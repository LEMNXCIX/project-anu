import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { BrowserRouter } from "react-router-dom";
import { NextUIProvider } from "@heroui/react";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
     <BrowserRouter>
        <NextUIProvider>
          <App />
        </NextUIProvider>
      </BrowserRouter>
  </React.StrictMode>,
);
