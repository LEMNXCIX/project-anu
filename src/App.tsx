import { Route, Routes } from "react-router-dom";
import MainPage from "./pages/Main";
import { config } from "./config/enviromentConfig";


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
    <>
      <Routes>
        <Route path="/" element={<MainPage />} />
      </Routes>
    </>
  );
}

export default App;
