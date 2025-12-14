import { Separator } from "@/components/ui/separator";
import { SidebarTrigger } from "@/components/ui/sidebar";
import { Button } from "./ui/button";
import { Icon } from "@tabler/icons-react";

export interface HeaderButtonProps extends React.ComponentProps<typeof Button> {
  label: String;
  icon?: Icon;
}

interface SiteHeaderProps {
  title: string;
  buttons?: HeaderButtonProps[];
}

export function SiteHeader({ title, buttons }: SiteHeaderProps) {
  return (
    <header className="flex h-(--header-height) shrink-0 items-center gap-2 border-b transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-(--header-height)">
      <div className="flex w-full items-center justify-between px-2">
        <div className="flex w-fit items-center gap-1 lg:gap-2 px-3 lg:px-5">
          <SidebarTrigger className="-ml-1" />
          <Separator
            orientation="vertical"
            className="mx-2 data-[orientation=vertical]:h-4"
          />
          <h1 className="text-base font-medium">{title}</h1>
        </div>
        <div className="flex items-center gap-2">
          {buttons?.map(({ label, icon: Icon, ...props }, i) => (
            <Button key={i} variant={"outline"} {...props}>
              {Icon && <Icon />}
              {label}
            </Button>
          ))}
        </div>
      </div>
    </header>
  );
}
