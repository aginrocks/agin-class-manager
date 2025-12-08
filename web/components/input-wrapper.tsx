import { cn } from "@/lib/utils";
import { HTMLAttributes, ReactNode } from "react";

type InputWrapperProps = {
  children: ReactNode;
  className?: HTMLAttributes<HTMLDivElement>["className"];
};

export default function InputWrapper({
  children,
  className,
}: InputWrapperProps) {
  return (
    <div
      className={cn(
        "flex flex-col items-start justify-center gap-2 w-fit",
        className,
      )}>
      {children}
    </div>
  );
}
