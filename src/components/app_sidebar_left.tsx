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

// This is sample data.
const data = {
  teams: [
    {
      name: "Acme Inc",
      logo: Command,
      plan: "Enterprise",
    },
    {
      name: "Acme Corp.",
      logo: AudioWaveform,
      plan: "Startup",
    },
    {
      name: "Evil Corp.",
      logo: Command,
      plan: "Free",
    },
  ],
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
};

export function SidebarLeft({
  ...props
}: React.ComponentProps<typeof Sidebar>) {
  const { state: appContex } = useApp();
  return (
    <Sidebar variant="sidebar" className="border-r-0 " {...props}>
      <SidebarHeader>
        <div className="flex flex-row items-center text-2xl space-x-2">
          <Button
            variant="ghost"
            size="icon"
            className="text-4xl m-2 font-bold"
          >
            𒀭
          </Button>{" "}
          <h1>Project ANU</h1>
        </div>
        <NavMain items={data.navMain} />
      </SidebarHeader>
      <SidebarContent>
        <NavProjects favorites={appContex?.config_user?.proyectos} />
        {/* <NavWorkspaces workspaces={data.workspaces} /> */}
        <NavSecondary items={data.navSecondary} className="mt-auto" />
      </SidebarContent>
      <SidebarRail />
    </Sidebar>
  );
}
