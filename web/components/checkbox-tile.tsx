import { HTMLAttributes } from "react";
import { Checkbox } from "./ui/checkbox";
import { CheckedState } from "@radix-ui/react-checkbox";

interface TileProps extends HTMLAttributes<HTMLDivElement> {
  name: string;
  onCheckedChange?: (checked: CheckedState) => void;
  checked?: CheckedState;
}

export default function CheckboxTile({
  name,
  onCheckedChange,
  checked,
}: TileProps) {
  return (
    <div className="flex w-full max-w-full p-3 gap-2 border-1 border-sidebar-border rounded-lg items-center">
      <Checkbox checked={checked} onCheckedChange={onCheckedChange} />
      <div>{name}</div>
    </div>
  );
}
