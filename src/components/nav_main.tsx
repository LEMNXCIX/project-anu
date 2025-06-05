import { type LucideIcon } from "lucide-react";
import {
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { useNavigate } from "react-router-dom";
import { memo } from "react";

// Memorizamos el componente para evitar re-renderizaciones innecesarias
const NavMain = memo(
  ({
    items,
  }: {
    items: {
      title: string;
      url: string;
      icon: LucideIcon;
      isActive?: boolean;
    }[];
  }) => {
    const navigate = useNavigate();
    const goHomePage = () => {
      if (window.location.pathname !== "/") {
        navigate("/");
      }
    };
    console.log("nav_mains");
    return (
      <SidebarMenu>
        {items.map((item) => (
          <SidebarMenuItem key={item.title}>
            <SidebarMenuButton asChild isActive={item.isActive}>
              <a onClick={goHomePage}>
                <item.icon />
                <span>{item.title}</span>
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        ))}
      </SidebarMenu>
    );
  }
);

// Agregamos una comparación personalizada para las props
NavMain.displayName = "NavMain";

export { NavMain };