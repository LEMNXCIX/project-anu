import { useEffect } from "react";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "../components/ui/accordion";

import CreateNewProyect from "../components/createNewProyect";
import { useDirectory } from "../context/directoryContex";
import { useListDirectory } from "../hooks/useDirectory";
import { checkForAppUpdates } from "../services/updateService";
import { useTheme } from "../components/theme-provider";
import { Button } from "../components/ui/button";

export default function MainPage() {
  const { state, dispatch } = useDirectory();
  const { listDirectory } = useListDirectory();
  const { setTheme, theme } = useTheme();
  const toggleTheme = () => {
    setTheme(theme === "dark" ? "light" : "dark");
  };
  useEffect(() => {
    listDirectory();
  }, []);
  return (
    <>
      <div className="w-2/3 justify-center mx-auto">
        <CreateNewProyect />
        <h1 className="mt-1 font-bold text-3xl p-10 text-center">Proyectos</h1>
        <div className="gap-2 grid grid-cols-2 sm:grid-cols-1 mb-48">
          <Accordion
            type="single"
            collapsible
            className="w-full"
            orientation="vertical"
          >
            {state.items.map((item, index) => (
              <AccordionItem
                value={index.toString()}
                key={index}
                disabled={!item.is_directory}
                asChild={true}
              >
                <AccordionTrigger showArrow={false}>
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
