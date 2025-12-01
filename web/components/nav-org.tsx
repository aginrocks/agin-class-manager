"use client";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { SidebarGroup, SidebarGroupContent } from "@/components/ui/sidebar";
import OrgOption from "./oraganization/org-option";
import { useQuery } from "@tanstack/react-query";
import { $api } from "@/lib/providers/api";
import { SelectedOrgAtom, TSelectedOrgAtom } from "@/lib/atoms/org";
import { useAtom } from "jotai";
import { IconPlus } from "@tabler/icons-react";
import { useDialogs } from "@/lib/dialogs";

export function NavOrganization() {
  const dialogs = useDialogs();

  const { data: organizations } = useQuery(
    $api.queryOptions("get", "/api/organizations", {
      params: {
        query: { "user-details": true },
      },
    }),
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
                name={selecrtedOrg?.name || "Please log in again"}
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
              <DropdownMenuItem onClick={() => dialogs.show("CreateOrg")}>
                <div className="p-2 flex justify-center items-center w-full gap-4">
                  <IconPlus className="[&_svg:not([class*='size-'])]:size-4 text-white" />
                  <div className="text-md font-bold">
                    Create an organization
                  </div>
                </div>
              </DropdownMenuItem>
            </DropdownMenuGroup>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  );
}
