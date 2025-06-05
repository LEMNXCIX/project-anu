import CreateNewProyect from "@/components/create_new_proyect";
import LastProjects from "@/components/list_last_proyects";
import { useApp } from "@/context/app_contex";

export default function MainPage() {
  const { state } = useApp();

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
