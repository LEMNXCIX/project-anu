import { useEffect } from "react";
import {
  Accordion,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";

import { useDirectory } from "@/context/directory_contex";
import { useListDirectory } from "@/hooks/use_directory";
import { useTheme } from "@/components/theme-provider";
import { Button } from "@/components/ui/button";
import { useNavigate } from "react-router-dom";
import { File, Folder } from "lucide-react";

export default function LastProjects() {
  const { state, dispatch } = useDirectory();
  const { listDirectory, setCurrentDirectory } = useListDirectory();
  const { setTheme, theme } = useTheme();
  const navigate = useNavigate();

  useEffect(() => {
    listDirectory();
  }, []);

  return (
    <>
      <h1 className="mt-1 font-bold text-3xl p-10 text-center">
        Ultimos Proyectos
      </h1>
      <div className="gap-2 grid grid-cols-2 sm:grid-cols-1 mb-48">
        <Accordion
          type="single"
          collapsible
          className="w-full"
          orientation="vertical"
        >
          {state.items.slice(0, 5).map((item, index) => (
            <AccordionItem
              value={index.toString()}
              key={index}
              disabled={!item.is_directory}
              asChild={true}
            >
              <div>
                <AccordionTrigger
                  showArrow={false}
                  onClick={() => {
                    setCurrentDirectory(item);
                    navigate("/details-projects/" + item.name);
                  }}
                >
                  {item.is_directory ? <Folder /> : <File />}
                  <span>{item.name}</span>
                </AccordionTrigger>
              </div>
            </AccordionItem>
          ))}
        </Accordion>
      </div>
    </>
  );
}
