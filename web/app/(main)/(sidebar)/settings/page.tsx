"use client";
import InputWrapper from "@/components/input-wrapper";
import { SiteHeader } from "@/components/site-header";
import { Button } from "@/components/ui/button";
import { SelectedOrgAtom } from "@/lib/atoms/org";
import { useDialogs } from "@/lib/dialogs";
import { $api } from "@/lib/providers/api";
import { useQueryClient } from "@tanstack/react-query";
import { useAtomValue } from "jotai";

export default function SettingsPage() {
  const dialogs = useDialogs();
  const queryClient = useQueryClient();

  const org = useAtomValue(SelectedOrgAtom);

  const deleteOrg = $api.useMutation("delete", "/api/organizations/{org_id}");

  async function handleDelete() {
    if (!org?.id) {
      return;
    }

    const confirm = await dialogs.show("Confirm", {
      confirmText: "Delete",
      description: `Are you sure you want to delete organization?`,
      destructive: true,
    });

    if (confirm) {
      await deleteOrg
        .mutateAsync({
          params: {
            path: { org_id: org.id },
          },
        })
        .then(
          async () => {
            await queryClient.invalidateQueries({
              queryKey: $api.queryOptions("get", "/api/user").queryKey,
              refetchType: "active",
            });
            await queryClient.invalidateQueries({
              queryKey: $api.queryOptions("get", "/api/organizations").queryKey,
              refetchType: "active",
            });
          },
          (e) => {
            alert(`Error occured when trying to delete that organization ${e}`);
          },
        );
    }
  }

  return (
    <>
      <SiteHeader title="Settings" />
      <div className="flex flex-1 flex-col p-4">
        <InputWrapper>
          <Button onClick={handleDelete} variant={"destructive"}>
            Delete organization
          </Button>
        </InputWrapper>
      </div>
    </>
  );
}
