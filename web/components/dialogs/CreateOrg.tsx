import { ComponentProps, useState } from "react";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { DialogProps } from "@/lib/dialogs/types";
import BaseDialog from "./Base";
import { $api, createOrgBody } from "@/lib/providers/api";
import { Input } from "../ui/input";
import { Label } from "../ui/label";
import InputWrapper from "../input-wrapper";
import { Textarea } from "../ui/textarea";
import { Button } from "../ui/button";
import { useRouter } from "next/navigation";
import { SelectedOrgAtom } from "@/lib/atoms/org";
import { useSetAtom } from "jotai";
import { QueryClient, useQueryClient } from "@tanstack/react-query";
import { useDialogs } from "@/lib/dialogs";

export default function CreateOrgDialog({
  ...props
}: ComponentProps<typeof DialogPrimitive.Root> & DialogProps<"CreateOrg">) {
  const router = useRouter();
  const queryClient = useQueryClient();
  const dialogs = useDialogs();

  const setSelectedOrg = useSetAtom(SelectedOrgAtom);
  const org = $api.useMutation("post", "/api/organizations");

  const [form, setform] = useState<createOrgBody>({
    name: "",
    description: "",
  });

  async function handleCreate() {
    await org.mutateAsync({ body: form }).then(
      (res) => {
        setSelectedOrg(res);
        dialogs.hide("CreateOrg");
        router.push("/dashboard");
        queryClient.invalidateQueries({
          queryKey: $api.queryOptions("get", "/api/user").queryKey,
        });
      },
      (e) => {
        alert(`Something went wrong: ${e.error}`);
      },
    );
  }

  return (
    <BaseDialog
      title="Create an organization"
      className="w-100"
      innerClassName="gap-4"
      {...props}>
      <InputWrapper className="w-full">
        <Label htmlFor="name">Name</Label>
        <Input
          id="name"
          type="text"
          placeholder="Enter a name"
          onChange={(c) =>
            setform((f) => ({ ...f, name: c?.currentTarget?.value }))
          }
        />
      </InputWrapper>
      <InputWrapper className="w-full">
        <Label htmlFor="description">Description</Label>
        <Textarea
          id="description"
          placeholder="Enter a description"
          className="min-h-40"
          onChange={(c) =>
            setform((f) => ({ ...f, description: c?.currentTarget?.value }))
          }
        />
      </InputWrapper>
      <InputWrapper className="w-full">
        <Label htmlFor="slug">Slug (optional)</Label>
        <Input
          id="slug"
          type="text"
          placeholder="Enter a slug or leave blank to generate automatically"
          onChange={(c) =>
            setform((f) => ({ ...f, slug: c?.currentTarget?.value }))
          }
        />
      </InputWrapper>
      <InputWrapper className="w-full">
        <Label htmlFor="avatar">Avatar (optional)</Label>
        <Input
          id="avatar"
          type="text"
          placeholder="Enter a link to avatar or leave blank"
          onChange={(c) =>
            setform((f) => ({ ...f, avatar_url: c?.currentTarget?.value }))
          }
        />
      </InputWrapper>
      <div className="flex w-full justify-end">
        <Button onClick={handleCreate}>Create</Button>
      </div>
    </BaseDialog>
  );
}
