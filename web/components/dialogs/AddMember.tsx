import { ComponentProps, useState } from "react";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { DialogProps } from "@/lib/dialogs/types";
import BaseDialog from "./Base";
import InputWrapper from "../input-wrapper";
import { Label } from "../ui/label";
import { Input } from "../ui/input";
import { $api, addMemberBody } from "@/lib/providers/api";
import { useDialogs } from "@/lib/dialogs";
import { useQueryClient } from "@tanstack/react-query";
import { Button } from "../ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "../ui/select";

export default function AddMemberDialog({
  ...props
}: ComponentProps<typeof DialogPrimitive.Root> & DialogProps<"AddMember">) {
  const queryClient = useQueryClient();
  const dialogs = useDialogs();

  const [addMemberForm, setAddMemberForm] = useState<addMemberBody>({
    email: "",
    role: "member",
  });

  const members = $api.useMutation(
    "post",
    "/api/organizations/{org_id}/members",
  );

  async function handleAdd() {
    await members
      .mutateAsync({
        body: addMemberForm,
        params: { path: { org_id: props.payload.org_id } },
      })
      .then(
        (_res) => {
          dialogs.hide("AddMember");
          queryClient.invalidateQueries({
            queryKey: $api.queryOptions("get", "/api/organizations/{org_id}", {
              params: { path: { org_id: props.payload.org_id } },
            }).queryKey,
          });
          queryClient.invalidateQueries({
            queryKey: $api.queryOptions("get", "/api/organizations").queryKey,
          });
        },
        (e) => {
          alert(`Something went wrong: ${e.error}`);
        },
      );
  }

  return (
    <BaseDialog
      title="Add a member to organization"
      className="w-fit min-w-[400]"
      innerClassName="gap-4"
      {...props}>
      <InputWrapper className="w-full">
        <Label htmlFor="email">User email</Label>
        <Input
          id="email"
          type="text"
          placeholder="Enter an email"
          value={addMemberForm.email}
          onChange={(c) =>
            setAddMemberForm((f) => ({ ...f, email: c?.currentTarget?.value }))
          }
        />
      </InputWrapper>
      <InputWrapper className="w-full">
        <Label>Pick member role</Label>
        <Select
          value={addMemberForm.role}
          onValueChange={(c) =>
            setAddMemberForm((f) => ({ ...f, role: c as "admin" | "member" }))
          }>
          <SelectTrigger id="role">
            <SelectValue placeholder="Select role" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="member">Member</SelectItem>
            <SelectItem value="admin">Admin</SelectItem>
          </SelectContent>
        </Select>
      </InputWrapper>

      <div className="flex w-full justify-end">
        <Button onClick={handleAdd}>Add member</Button>
      </div>
    </BaseDialog>
  );
}
