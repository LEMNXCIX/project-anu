import { useEffect } from "react";
import CreateNewProyect from "@/components/create_new_proyect";
import { useListDirectory } from "@/hooks/use_directory";
import { useTheme } from "@/components/theme-provider";
import { useNavigate } from "react-router-dom";
import LastProjects from "@/components/list-last-proyects";
import { useApp } from "@/context/app_contex";

export default function MainPage() {
  const { listDirectory, setCurrentDirectory } = useListDirectory();
  const { setTheme, theme } = useTheme();
  const navigate = useNavigate();

  const toggleTheme = () => {
    setTheme(theme === "dark" ? "light" : "dark");
  };
  const { state } = useApp();
  useEffect(() => {
    listDirectory();
  }, []);

  return (
    <>
      <div className="w-11/12 justify-center mx-auto mt-5">
        <h1 className="text-3xl font-bold text-center m-5">
          Bienvenido, {state.config_user.usuario.nombre} 👋
        </h1>
        <CreateNewProyect />
        <LastProjects />
      </div>
    </>
  );
}
