"use client";
import { components, paths } from "@/types/api";
import { useQuery } from "@tanstack/react-query";
import { atom, useAtom, useSetAtom } from "jotai";
import { $api } from "../providers/api";
import { useEffect } from "react";

export const SelectedOrgAtom = atom<TSelectedOrgAtom | null>(null);

export type TSelectedOrgAtom =
  paths["/api/organizations"]["get"]["responses"]["200"]["content"]["application/json"][0];

export type TOrgUSer = components["schemas"]["OrgUser"];

export function useBindSelectedOrg() {
  const [selectedOrg, setSelectedOrg] = useAtom(SelectedOrgAtom);

  const { data: organizations, isFetched } = useQuery(
    $api.queryOptions("get", "/api/organizations", {
      params: {
        query: { "user-details": true },
      },
    }),
  );

  useEffect(() => {
    if (organizations && !selectedOrg) {
      setSelectedOrg(organizations?.[0] || null);
    } else if (isFetched && !organizations) {
      console.log("No organizations found");
    }
  }, [organizations]);
}
