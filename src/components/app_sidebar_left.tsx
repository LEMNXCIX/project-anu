import * as React from "react";
import {
  AudioWaveform,
  Command,
  Home,
  MessageCircleQuestion,
  Settings2,
} from "lucide-react";
import { NavProjects } from "@/components/nav_projects";
import { NavMain } from "@/components/nav_main";
import { NavSecondary } from "@/components/nav_secondary";
import {
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarRail,
} from "@/components/ui/sidebar";
import { Button } from "./ui/button";
import { useApp } from "@/context/app_contex";
import { useMemo } from "react";

function SidebarLeftComponent({ ...props }: React.ComponentProps<typeof Sidebar>) {
  const { state: appContex } = useApp();

  const data = useMemo(
    () => ({
      navMain: [
        {
          title: "Home",
          url: "/",
          icon: Home,
          isActive: true,
        },
      ],
      navSecondary: [
        {
          title: "Settings",
          url: "#",
          icon: Settings2,
        },
        {
          title: "Help",
          url: "#",
          icon: MessageCircleQuestion,
        },
      ],
    }),
    []
  );

  const favorites = useMemo(() => appContex?.config_user?.proyectos || [], [
    appContex?.config_user?.proyectos,
  ]);

  console.log("SidebarLeft renderizado");
  return (
    <Sidebar variant="sidebar" className="border-r-0" {...props}>
      <SidebarHeader>
        <div className="flex flex-row items-center text-2xl space-x-2">
          <Button variant="ghost" size="icon" className="text-4xl m-2 font-bold">
            𒀭
          </Button>
          <h1>Project ANU</h1>
        </div>
        <NavMain items={data.navMain} />
      </SidebarHeader>
      <SidebarContent>
        <NavProjects favorites={favorites} />
        <NavSecondary items={data.navSecondary} className="mt-auto" />
      </SidebarContent>
      <SidebarRail />
    </Sidebar>
  );
}

export const SidebarLeft = React.memo(SidebarLeftComponent);