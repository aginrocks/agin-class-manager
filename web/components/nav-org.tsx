"use client";

import {
  SidebarGroup,
  SidebarGroupContent,
  useSidebar,
} from "@/components/ui/sidebar";

export function NavOrganization() {
  return (
    <SidebarGroup className="group-data-[collapsible=icon]:hidden">
      <SidebarGroupContent className="flex flex-col gap-2"></SidebarGroupContent>
    </SidebarGroup>
  );
}
