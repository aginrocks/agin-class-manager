"use client";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useDialogs } from "@/lib/dialogs";
import { useAvatar } from "@/lib/hooks/use-avatar";
import { useIsMobile } from "@/lib/hooks/use-mobile";
import { $api } from "@/lib/providers/api";
import { IconDotsVertical, IconLogout } from "@tabler/icons-react";
import { useQuery } from "@tanstack/react-query";
import { useRouter } from "next/navigation";

export default function Page() {
  const router = useRouter();
  const isMobile = useIsMobile();
  const dialogs = useDialogs();

  const { data: userData } = useQuery($api.queryOptions("get", "/api/user"));

  const avatar = useAvatar(userData?.email);
  const avatarFallbackText = userData?.name?.charAt(0)?.toUpperCase() ?? "";

  return (
    <div className="w-full h-full flex flex-col items-center justify-center gap-4 relative">
      <div className="absolute bottom-4 left-4 flex border-border border-2 rounded-md cursor-pointer">
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <div className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground flex p-2 gap-2 items-center ">
              <Avatar className="h-8 w-8 rounded-lg">
                <AvatarImage src={avatar} alt={userData?.name} />
                <AvatarFallback className="rounded-lg">
                  {avatarFallbackText}
                </AvatarFallback>
              </Avatar>
              <div className="grid flex-1 text-left text-sm leading-tight">
                <span className="truncate font-medium">{userData?.name}</span>
                <span className="text-muted-foreground truncate text-xs">
                  {userData?.email}
                </span>
              </div>
              <IconDotsVertical className="ml-auto size-4" />
            </div>
          </DropdownMenuTrigger>
          <DropdownMenuContent
            className="w-(--radix-dropdown-menu-trigger-width) min-w-56 rounded-lg"
            side={isMobile ? "bottom" : "right"}
            align="end"
            sideOffset={4}>
            <DropdownMenuLabel className="p-0 font-normal">
              <div className="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                <Avatar className="h-8 w-8 rounded-lg">
                  <AvatarImage src={avatar} alt={userData?.name} />
                  <AvatarFallback className="rounded-lg">
                    {avatarFallbackText}
                  </AvatarFallback>
                </Avatar>
                <div className="grid flex-1 text-left text-sm leading-tight">
                  <span className="truncate font-medium">{userData?.name}</span>
                  <span className="text-muted-foreground truncate text-xs">
                    {userData?.email}
                  </span>
                </div>
              </div>
            </DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem onClick={() => router.push("/api/logout")}>
              <IconLogout />
              Log out
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      <div className="text-xl font-bold">
        Looks like you don&apos;t have any organization yet
      </div>
      <Button onClick={() => dialogs.show("CreateOrg")}>
        Create an organization
      </Button>
    </div>
  );
}
