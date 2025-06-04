import { useEffect } from "react";
import {
  Accordion,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";

import { useDirectory } from "@/context/directory_contex";
import { useListDirectory } from "@/hooks/use_directory";
import { useNavigate } from "react-router-dom";

export default function DetailsProjects() {
  const { state } = useDirectory();
  const { listDirectory, setCurrentDirectory } = useListDirectory();
  const navigate = useNavigate();
  useEffect(() => {
    listDirectory(state.currentDirectory.path);
  }, []);
  return (
    <>
      <div className="w-2/3 justify-center mx-auto">
        <h1 className="mt-1 font-bold text-3xl p-10 text-center">
          {state.currentDirectory.name}
        </h1>
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
                <AccordionTrigger
                  showArrow={false}
                  onClick={() => {
                    setCurrentDirectory(item);
                    navigate("/details-projects/" + item.name);
                  }}
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
