import { paths } from "@/types/api";
import { useQuery } from "@tanstack/react-query";
import { atom, useSetAtom } from "jotai";
import { useEffect } from "react";
import { $api } from "../providers/api";

export const UserAtom = atom<TUser | null>(null);

export type TUser =
  paths["/api/user"]["get"]["responses"]["200"]["content"]["application/json"];

export function useBindUser() {
  const setUser = useSetAtom(UserAtom);

  const { data: userData, isFetched } = useQuery(
    $api.queryOptions("get", "/api/user"),
  );

  useEffect(() => {
    if (userData) {
      setUser(userData || null);
    } else if (isFetched && !userData) {
      console.log("No organizations found");
    }
  }, [userData]);
}
