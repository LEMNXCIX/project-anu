import {
  Accordion,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { useDirectory } from "@/context/directory_contex";
import { useListDirectory } from "@/hooks/use_directory";
import { useNavigate, useParams } from "react-router-dom"; // Añadimos useParams
import MarkdownViewer from "@/components/md_editor";
import { useState } from "react";

export default function DetailsProjects() {
  const { state } = useDirectory();
  const { setCurrentDirectory } = useListDirectory();
  const navigate = useNavigate();
  const { id } = useParams(); // Obtenemos el :id de la ruta
  const [selectedFileContent, setSelectedFileContent] = useState<string | null>(null);

  const handleDirectoryClick = (item) => {
    if (item.is_directory) {
      console.log("setCurrentDirectory llamado con:", item);
      setCurrentDirectory(item);
      // Solo navegamos si el :id actual no coincide con el nombre del directorio
      if (id !== item.name) {
        navigate("/details-projects/" + item.name);
      }
    } else {
      const fileContent = `# Contenido de ${item.name}\nEste es un ejemplo de contenido del archivo.\n- Punto 1\n- Punto 2\n[Enlace de ejemplo](https://ejemplo.com)`;
      setSelectedFileContent(fileContent);
    }
  };

  return (
    <>
      <div className="w-11/12 justify-center mx-auto">
        <h1 className="mt-1 font-bold text-3xl p-10 text-center">
          {state.currentDirectory.name}
        </h1>
        <MarkdownViewer />
        <div className="gap-2 grid grid-cols-1 sm:grid-cols-2 mb-10">
          <Accordion
            type="single"
            collapsible
            className="w-full"
            orientation="horizontal"
          >
            {state.items.map((item, index) => (
              <AccordionItem
                value={index.toString()}
                key={index}
                disabled={!item.is_directory}
                asChild={true}
              >
                <AccordionTrigger
                  showArrow={false}
                  onClick={() => handleDirectoryClick(item)}
                  aria-label={
                    item.is_directory
                      ? `Abrir directorio ${item.name}`
                      : `Ver contenido de archivo ${item.name}`
                  }
                >
                  {(item.is_directory ? "📁" : "📝") + " " + item.name}
                </AccordionTrigger>
              </AccordionItem>
            ))}
          </Accordion>
        </div>
      </div>
    </>
  );
}