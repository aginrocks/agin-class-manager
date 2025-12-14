import { HTMLAttributes, MouseEventHandler } from "react";
import { Button } from "./ui/button";

interface TileProps extends HTMLAttributes<HTMLDivElement> {
  name: string;
  amountToPay?: number;
}

export default function ButtonTile({ name, amountToPay, ...props }: TileProps) {
  return (
    <div
      className="relative flex w-full max-w-full p-3 gap-2 border-1 border-sidebar-border rounded-lg items-center overflow-hidden"
      {...props}>
      <div className="cursor-pointer absolute top-0 bottom-0 left-0 right-0 bg-[#000000]/50 flex items-center justify-center opacity-0 hover:opacity-100 transition-opacity duration-300 z-10">
        <div className="text-sm font-bold select-none">Click to change</div>
      </div>
      <div className="flex flex-col gap-1 px-2">
        <div>{name}</div>
        <div className="text-sm text-muted-foreground">
          {amountToPay ? amountToPay : "Not changed"}
        </div>
      </div>
    </div>
  );
}
