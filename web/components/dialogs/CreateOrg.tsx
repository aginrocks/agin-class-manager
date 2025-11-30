import { ComponentProps } from "react";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { DialogProps } from "@/lib/dialogs/types";
import BaseDialog from "./Base";
import { $api } from "@/lib/providers/api";
import { Input } from "../ui/input";
import { Label } from "../ui/label";
import InputWrapper from "../input-wrapper";
import { Textarea } from "../ui/textarea";

export default function CreateOrgDialog({
  ...props
}: ComponentProps<typeof DialogPrimitive.Root> & DialogProps<"CreateOrg">) {
  const org = $api.useMutation("post", "/api/organizations");

  return (
    <BaseDialog
      title="Create an organization"
      className="w-100"
      innerClassName="gap-4"
      {...props}>
      <InputWrapper className="w-full">
        <Label htmlFor="name">Name</Label>
        <Input id="name" type="text" placeholder="Enter a name" />
      </InputWrapper>
      <InputWrapper className="w-full">
        <Label htmlFor="name">Description</Label>
        <Textarea placeholder="Enter a description" className="min-h-40" />
      </InputWrapper>
    </BaseDialog>
  );
}
