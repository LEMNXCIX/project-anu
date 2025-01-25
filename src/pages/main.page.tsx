import { Accordion, AccordionItem } from "@heroui/accordion";
import React, { useState, useEffect } from "react";
import { readDir, BaseDirectory, exists } from "@tauri-apps/plugin-fs";
import { Card, CardBody, CardFooter } from "@heroui/react";
import CreateNewProyect from "../components/createNewProyect";
import { invoke } from "@tauri-apps/api/core";
import { tauriService } from "../services/tauriService";
import { TauriResponse } from "../types/tauriResponse";
import { useDirectory } from "../context/directoryContex";
import { useListDirectory } from "../hooks/useDirectory";

export default function MainPage() {
  const { state, dispatch } = useDirectory();
  const { listDirectory } = useListDirectory();

  useEffect(() => {
    listDirectory();
  }, []);
  return (
    <>
      <div className="w-2/3 justify-center mx-auto">
        <h1 className="mt-4 font-bold text-4xl p-14 text-center">
          𒀭 Project ANU
        </h1>
        <CreateNewProyect />
        <h1 className="mt-1 font-bold text-3xl p-10 text-center">Proyectos</h1>
        <div className="gap-2 grid grid-cols-2 sm:grid-cols-1 mb-48">
          {state.items.map((item, index) => (
            <Card
              key={index}
              shadow="sm"
              title={(item.is_directory ? "📁" : "📝") + " " + item.name}
              isPressable={item.is_directory}
              radius="sm"
            >
              <CardBody>
                <span>
                  {(item.is_directory ? "📁" : "📝") + " " + item.name}
                </span>
              </CardBody>
            </Card>
          ))}
        </div>
      </div>
    </>
  );
}
