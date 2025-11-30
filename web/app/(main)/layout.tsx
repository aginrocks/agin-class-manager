"use client";
import AtomProvider from "@/lib/providers/atom-provider";
import { ReactNode } from "react";

export default function Layout({ children }: { children: ReactNode }) {
  return <AtomProvider>{children}</AtomProvider>;
}
