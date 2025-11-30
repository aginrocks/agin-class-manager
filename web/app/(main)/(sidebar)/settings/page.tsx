"use client";
import InputWrapper from "@/components/input-wrapper";
import { SiteHeader } from "@/components/site-header";
import { Button } from "@/components/ui/button";
import { useDialogs } from "@/lib/dialogs";

export default function SettingsPage() {
  const dialogs = useDialogs();

  async function handleDelete() {
    await dialogs.show("Confirm", {
      confirmText: "Delete",
      description: `Are you sure you want to delete organization?`,
      destructive: true,
    });
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
