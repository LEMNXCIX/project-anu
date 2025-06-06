import * as React from "react";
import { Plus, X } from "lucide-react";
import { Button } from "@/components/ui/button";

interface CustomRightSidebarProps {
  isOpen: boolean;
  onToggle: () => void;
}

export function CustomRightSidebar({
  isOpen,
  onToggle,
}: CustomRightSidebarProps) {
  const handleSidebarClick = (e: React.MouseEvent) => {
    e.stopPropagation(); // Evitamos que el clic se propague al contenedor padre
  };

  return (
    <div
      className={`fixed top-9 right-0 rounded-l-lg h-[calc(100svh-36px)] w-64 border-l text-accent-foreground/70  bg-sidebar shadow-lg transform transition-transform duration-300 ease-in-out z-40 ${
        isOpen ? "translate-x-0" : "translate-x-full"
      }`}
      onClick={handleSidebarClick} // Manejamos los clics dentro del sidebar
    >
      <div className="flex flex-col h-full">
        <div className="h-16 p-4 flex items-center justify-between border-b  shrink-0">
          <h2 className="text-lg font-semibold">Calendars</h2>
          <Button
            variant="ghost"
            size="sm"
            onClick={onToggle}
            aria-label={isOpen ? "Close sidebar" : "Open sidebar"}
          >
            <X className="h-4 w-4" />
          </Button>
        </div>
        <div className="flex-1 overflow-y-auto p-4">
          <p>Ejemplo de contenido que puede hacer scroll si es muy largo.</p>
          {Array.from({ length: 50 }).map((_, index) => (
            <p key={index}>Línea {index + 1}</p>
          ))}
        </div>
        <div className="p-4 border-t shrink-0">
          <Button
            onClick={() => console.log("Crear nuevo calendario")}
            className="w-full"
            aria-label="Create new calendar"
          >
            <Plus className="mr-2 h-4 w-4" />
            New Calendar
          </Button>
        </div>
      </div>
    </div>
  );
}
