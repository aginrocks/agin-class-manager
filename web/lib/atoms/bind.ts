"use client";

import { useBindSelectedOrg } from "./org";
import { useBindUser } from "./user";

export function useBindAtoms() {
  useBindSelectedOrg();
  useBindUser();
}
