"use client";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { SidebarGroup, SidebarGroupContent } from "@/components/ui/sidebar";
import OrgOption from "./oraganization/org-option";
import { useQuery } from "@tanstack/react-query";
import { $api } from "@/lib/providers/api";
import { SelectedOrgAtom, TSelectedOrgAtom } from "@/lib/atoms/org";
import { useAtom } from "jotai";

export function NavOrganization() {
  const { data: organizations } = useQuery(
    $api.queryOptions("get", "/api/organizations")
  );

  const [selecrtedOrg, setSelectedOrg] = useAtom(SelectedOrgAtom);
  function handleSelectOrg(org: TSelectedOrgAtom) {
    setSelectedOrg(org);
  }

  return (
    <SidebarGroup className="group-data-[collapsible=icon]:hidden">
      <SidebarGroupContent className="flex flex-col gap-2">
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <div className="p-2 border rounded-md hover:bg-accent cursor-pointer">
              <OrgOption
                name={selecrtedOrg?.name || "dupa"}
                image={selecrtedOrg?.avatar_url || undefined}
              />
            </div>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="w-56" align="start">
            <DropdownMenuLabel className="font-bold">
              Select organization
            </DropdownMenuLabel>
            <DropdownMenuGroup>
              {organizations?.map((org) => (
                <DropdownMenuItem
                  key={org._id}
                  onClick={() => handleSelectOrg(org)}>
                  <OrgOption
                    size="small"
                    name={org.name}
                    image={org?.avatar_url || undefined}
                  />
                </DropdownMenuItem>
              ))}
            </DropdownMenuGroup>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  );
}
