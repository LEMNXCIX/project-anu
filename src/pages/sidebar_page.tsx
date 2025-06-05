import { useLocation, useNavigate } from "react-router-dom";
import { AppSidebar } from "@/components/app-sidebar";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import {
  SidebarInset,
  SidebarProvider,
} from "@/components/ui/sidebar";
import { useDirectory } from "@/context/directory_contex";
import { useListDirectory } from "@/hooks/use_directory";
import { useEffect, useState } from "react";
import { VITE_ENVIRONMENT } from "@/config/enviroment_config";
import { Label } from "@/components/ui/label";
export default function Sidebar({ children }: { children: React.ReactNode }) {
  const { setHistorialPath } = useListDirectory();
  const navigate = useNavigate();
  const location = useLocation();
  const { state: directoryState } = useDirectory();
  const isRootPath = location.pathname === "/"; // Más semántico y legible
  const historialDirectorios = directoryState.historialPath;

  const goHomePage = () => {
    navigate("/");
  };
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    setIsVisible(VITE_ENVIRONMENT == "DEV"); // Show only if the VITE_ENVIRONMENT is development
  }, [VITE_ENVIRONMENT]);

  return (
    <>
      <div className="flex flex-col min-h-screen">
        <>dss</>
        {/* Contenedor principal para Sidebar y contenido */}
        <SidebarProvider>
          <AppSidebar />
          <div className="flex flex-1 flex-col">
            {isVisible && (
              <Label className="sticky top-0 w-full text-center text-xs bg-orange-600 p-1 text-current font-bold z-50">
                Developer Mode
              </Label>
            )}

            {isRootPath ? (
              // Caso para la ruta raíz: solo contenido sin breadcrumb ni header
              <div className="flex flex-1 flex-col gap-4 p-4">{children}</div>
            ) : (
              // Caso para otras rutas: con breadcrumb y estructura completa
              <SidebarInset>
                <header className="sticky top-0 flex shrink-0 items-center gap-2 border-b bg-background p-4">
                  <Breadcrumb>
                    <BreadcrumbList>
                      {historialDirectorios.map((item, key) => (
                        <>
                          <BreadcrumbItem
                            key={key}
                            onClick={() => setHistorialPath(item)}
                          >
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
          </div>
        </SidebarProvider>
      </div>
    </>
  );
}
