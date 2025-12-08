"use client";

import * as React from "react";
import {
  IconBuildingBroadcastTower,
  IconDashboard,
  IconGift,
  IconListDetails,
  IconPlus,
  IconSettings,
} from "@tabler/icons-react";

import { NavMain } from "@/components/nav-main";
import { NavSecondary } from "@/components/nav-secondary";
import { NavUser } from "@/components/nav-user";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import Logo from "@/components/logo";
import { NavOrganization } from "./nav-org";
import { useDialogs } from "@/lib/dialogs";
import { useAtomValue } from "jotai";
import { SelectedOrgAtom } from "@/lib/atoms/org";

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  const dialogs = useDialogs();

  const org = useAtomValue(SelectedOrgAtom);

  const data = {
    user: {
      name: "shadcn",
      email: "m@example.com",
      avatar: "/avatars/shadcn.jpg",
    },
    navMain: [
      {
        title: "Dashboard",
        url: "/dashboard",
        icon: IconDashboard,
      },
      {
        title: "Budget",
        url: "/budget",
        icon: IconListDetails,
      },
      // {
      //   title: "Announcements",
      //   url: "#",
      //   icon: IconBuildingBroadcastTower,
      // },
      {
        title: "Secret Santa",
        url: "/secret-santa",
        icon: IconGift,
      },
    ],
    navSecondary: [
      {
        title: "Add Member",
        icon: IconPlus,
        onClick: () => {
          if (org?.id) {
            dialogs.show("AddMember", { org_id: org.id });
          } else {
            alert("Org id is not present, this is probably a bug");
          }
        },
      },
      {
        title: "Settings",
        icon: IconSettings,
        url: "/settings",
      },
      // {
      //   title: "Search",
      //   url: "#",
      //   icon: IconSearch,
      // },
    ],
  };

  return (
    <Sidebar collapsible="offcanvas" {...props}>
      <SidebarHeader>
        <SidebarMenu>
          <SidebarMenuItem>
            <SidebarMenuButton
              asChild
              className="data-[slot=sidebar-menu-button]:!p-1.5 flex justify-start items-center min-w-max">
              <a href="#">
                <Logo size={3} />
              </a>
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarHeader>
      <SidebarContent>
        <NavOrganization />
        <NavMain items={data.navMain} />
        <NavSecondary items={data.navSecondary} className="mt-auto" />
      </SidebarContent>
      <SidebarFooter>
        <NavUser />
      </SidebarFooter>
    </Sidebar>
  );
}
