import { Route, Routes } from "react-router-dom";
import MainPage from "./pages/Main";
import { config } from "@/config/enviromentConfig";
import { ThemeProvider } from "@/components/theme-provider";
import Page from "./pages/page";

function App() {
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
      <Page>
        <Routes>
          <Route path="/" element={<MainPage />} />
        </Routes>
      </Page>
    </ThemeProvider>
  );
}

export default App;
