import { ComponentProps, useEffect, useState } from "react";
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
import { SelectedOrgAtom, TOrgUSer } from "@/lib/atoms/org";
import { useAtomValue, useSetAtom } from "jotai";
import { QueryClient, useQuery, useQueryClient } from "@tanstack/react-query";
import { useDialogs } from "@/lib/dialogs";
import { DatePicker } from "../date-picker";
import CheckboxTile from "../checkbox-tile";

export default function CreateSantaDialog({
  ...props
}: ComponentProps<typeof DialogPrimitive.Root> & DialogProps<"CreateSanta">) {
  const router = useRouter();
  const queryClient = useQueryClient();
  const dialogs = useDialogs();

  const org = useAtomValue(SelectedOrgAtom);

  const santamut = $api.useMutation(
    "post",
    "/api/organizations/{org_id}/santa",
  );
  const [selectedMembers, setSelectedMembers] = useState<Set<string>>(
    new Set(),
  );

  useEffect(() => {
    if (org?.members) {
      const memberIds = org.members
        .filter(
          (member): member is TOrgUSer =>
            typeof member === "object" &&
            "name" in member &&
            "email" in member &&
            "id" in member,
        )
        .map((member) => member.id);
      setSelectedMembers(new Set(memberIds));
    }
  }, [org?.members]);
  const [startDate, setStartDate] = useState<Date>();
  const [propositionsDue, setPropositionsDue] = useState<Date>();
  const [endDate, setEndDate] = useState<Date>();

  const handleMemberToggle = (memberId: string, checked: boolean) => {
    setSelectedMembers((prev) => {
      const newSet = new Set(prev);
      if (checked) {
        newSet.add(memberId);
      } else {
        newSet.delete(memberId);
      }
      return newSet;
    });
  };

  async function handleCreate() {
    if (!startDate || !propositionsDue || !endDate || !org) {
      return;
    }

    const participants = Array.from(selectedMembers).map((memberId) => ({
      user_id: memberId,
      present_reciever: memberId,
      proposition: "",
    }));

    try {
      await santamut.mutateAsync({
        params: {
          //@ts-expect-error undefined type for some reason
          path: {
            org_id: org._id,
          },
        },
        body: {
          start_date: startDate.toISOString(),
          participants,
          end_date: endDate.toISOString(),
          propositions_due: propositionsDue.toISOString(),
        },
      });

      queryClient.invalidateQueries({
        queryKey: ["/api/organizations/{org_id}/santa"],
      });

      props.onOpenChange?.(false);
    } catch (error) {
      alert(`Failed to create santa event: ${error}`);
    }
  }

  const isFormValid =
    startDate && propositionsDue && endDate && selectedMembers.size > 0;

  return (
    <BaseDialog
      title="Create an event"
      className="w-fit"
      innerClassName="gap-4"
      {...props}>
      <div className="flex justify-between gap-10">
        <div className="flex flex-col gap-4 w-[200px]">
          <InputWrapper className="w-full">
            <Label>Start date</Label>
            <DatePicker value={startDate} onChange={setStartDate} />
          </InputWrapper>
          <InputWrapper className="w-full">
            <Label>Propositions due</Label>
            <DatePicker value={propositionsDue} onChange={setPropositionsDue} />
          </InputWrapper>
          <InputWrapper className="w-full">
            <Label>End date</Label>
            <DatePicker value={endDate} onChange={setEndDate} />
          </InputWrapper>
        </div>
        <div className="flex flex-col gap-2 w-[300px]">
          <Label>Participants:</Label>
          <div className="max-h-[300px] flex-1 overflow-y-auto border-border border rounded p-2 bg-card flex flex-col gap-2">
            {org?.members && org.members.length > 0 ? (
              org.members
                .filter(
                  (member): member is TOrgUSer =>
                    typeof member === "object" &&
                    "name" in member &&
                    "email" in member &&
                    "id" in member,
                )
                .map((member) => (
                  <CheckboxTile
                    key={member.id}
                    name={member.name}
                    checked={selectedMembers.has(member.id)}
                    onCheckedChange={(checked) =>
                      handleMemberToggle(member.id, checked === true)
                    }
                  />
                ))
            ) : (
              <p className="text-muted-foreground text-sm">
                No members found. Please ensure user-details is enabled.
              </p>
            )}
          </div>
        </div>
      </div>
      <div className="flex w-full justify-end">
        <Button onClick={handleCreate} disabled={!isFormValid}>
          Create
        </Button>
      </div>
    </BaseDialog>
  );
}
