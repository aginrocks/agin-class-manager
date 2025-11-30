"use client";
import { AppSidebar } from "@/components/app-sidebar";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { Spinner } from "@/components/ui/spinner";
import { UserAtom } from "@/lib/atoms/user";
import { $api } from "@/lib/providers/api";
import AtomProvider from "@/lib/providers/atom-provider";
import { useQuery } from "@tanstack/react-query";
import { useAtomValue } from "jotai";
import { useRouter } from "next/navigation";
import { CSSProperties, ReactNode, useEffect } from "react";

export default function Layout({
  children,
}: Readonly<{
  children: ReactNode;
}>) {
  return <AtomProvider>{children}</AtomProvider>;
}
