import { ComponentProps, HTMLAttributes, ReactNode } from "react";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { Dialog, DialogContent, DialogTitle } from "../ui/dialog";
import { cn } from "@/lib/utils";

interface BaseDilogProps extends ComponentProps<typeof DialogPrimitive.Root> {
  title: string;
  children: ReactNode;
  className?: HTMLAttributes<HTMLDivElement>["className"];
  innerClassName?: HTMLAttributes<HTMLDivElement>["className"];
}

export default function BaseDialog({
  title,
  children,
  className,
  innerClassName,
  ...props
}: BaseDilogProps) {
  return (
    <Dialog {...props}>
      <DialogContent
        className={cn(
          "sm:max-w-3xl w-100 h-fit bg-background/70 dark:bg-background/50 backdrop-blur-sm overflow-hidden flex flex-col",
          className,
        )}>
        <DialogTitle>{title}</DialogTitle>
        <div className={cn("flex flex-col w-full flex-1", innerClassName)}>
          {children}
        </div>
      </DialogContent>
    </Dialog>
  );
}
