"use client";
import InputWrapper from "@/components/input-wrapper";
import { SiteHeader } from "@/components/site-header";
import { Button } from "@/components/ui/button";
import { useDialogs } from "@/lib/dialogs";

export default function SecretSanta() {
  const dialogs = useDialogs();

  return (
    <>
      <SiteHeader title="Secret santa" />
      <div className="flex flex-1 flex-col p-4 w-full h-full justify-center items-center">
        <InputWrapper className="justify-center items-center">
          <div className="text-lg font-bold ">
            No secret santa event is being held in this organization
          </div>
          <Button onClick={() => dialogs.show("CreateSanta")}>
            Create event
          </Button>
        </InputWrapper>
      </div>
    </>
  );
}
