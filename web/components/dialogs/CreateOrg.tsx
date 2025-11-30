import { ComponentProps } from "react";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { DialogProps } from "@/lib/dialogs/types";

export default function CreateOrgDialog({}: ComponentProps<
  typeof DialogPrimitive.Root
> &
  DialogProps<"CreateOrg">) {
  return <div></div>;
}
