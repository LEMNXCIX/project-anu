import { useLocation, useNavigate } from "react-router-dom";
import { AppSidebar } from "@/components/app-sidebar";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import {
  SidebarInset,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import { useDirectory } from "@/context/directory_contex";
import { useListDirectory } from "@/hooks/use_directory";

export default function Sidebar({ children }: { children: React.ReactNode }) {
  const { setCurrentDirectory, setHistorialPath } = useListDirectory();
  const navigate = useNavigate();
  const location = useLocation();
  const { state: directoryState } = useDirectory();
  const isRootPath = location.pathname === "/"; // Más semántico y legible
  const historialDirectorios = directoryState.historialPath;

  const goHomePage = () => {
    setCurrentDirectory();
    navigate("/");
  };
  return (
    <SidebarProvider
      style={
        {
          "--sidebar-width": "350px",
        } as React.CSSProperties
      }
    >
      <AppSidebar />
      {isRootPath ? (
        // Caso para la ruta raíz: solo contenido sin breadcrumb ni header
        <div className="flex flex-1 flex-col gap-4 p-4">{children}</div>
      ) : (
        // Caso para otras rutas: con breadcrumb y estructura completa
        <SidebarInset>
          <header className="sticky top-0 flex shrink-0 items-center gap-2 border-b bg-background p-4">
            {/* <SidebarTrigger className="-ml-1" /> */}
            <Breadcrumb>
              <BreadcrumbList>
                {historialDirectorios.map((item, key) => (
                  <>
                    <BreadcrumbItem onClick={() => setHistorialPath(item)}>
                      <BreadcrumbPage>{item.name}</BreadcrumbPage>
                    </BreadcrumbItem>
                    {key < historialDirectorios.length - 1 && (
                      <BreadcrumbSeparator className="hidden md:block" />
                    )}
                  </>
                ))}
              </BreadcrumbList>
            </Breadcrumb>
          </header>
          <div className="flex flex-1 flex-col gap-4 p-4">{children}</div>
        </SidebarInset>
      )}
    </SidebarProvider>
  );
}
