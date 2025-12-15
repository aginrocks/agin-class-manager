"use client";
import { AppSidebar } from "@/components/app-sidebar";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import { Spinner } from "@/components/ui/spinner";
import { $api } from "@/lib/providers/api";
import { useQuery } from "@tanstack/react-query";
import { useRouter } from "next/navigation";
import { CSSProperties, ReactNode, useEffect } from "react";

export default function Layout({
  children,
}: Readonly<{
  children: ReactNode;
}>) {
  const router = useRouter();

  const {
    data: user,
    isLoading,
    status,
  } = useQuery($api.queryOptions("get", "/api/user"));

  useEffect(() => {
    if (!isLoading && status == "error") {
      router.push("/login");
    }
    if (!user || isLoading) {
      return;
    }

    if (user?.organizations.length == 0) {
      router.push("/no_org");
    }
  }, [user?.organizations, status, isLoading]);

  return isLoading || user?.organizations.length == 0 ? (
    <div className="w-full h-full justify-center items-center flex flex-col gap-2">
      <Spinner className="size-10" aria-label="Loading..." />
      <div className="font-bold text-2xl">Loading...</div>
    </div>
  ) : (
    <SidebarProvider
      style={
        {
          "--sidebar-width": "calc(var(--spacing) * 72)",
          "--header-height": "calc(var(--spacing) * 12)",
        } as CSSProperties
      }>
      <AppSidebar variant="inset" />
      <SidebarInset>{children}</SidebarInset>
    </SidebarProvider>
  );
}
