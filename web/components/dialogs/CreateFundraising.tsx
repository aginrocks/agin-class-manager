import { ComponentProps, useEffect, useMemo, useState } from "react";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { DialogProps } from "@/lib/dialogs/types";
import BaseDialog from "./Base";
import { $api } from "@/lib/providers/api";
import { Label } from "../ui/label";
import InputWrapper from "../input-wrapper";
import { Button } from "../ui/button";
import { SelectedOrgAtom, TOrgUSer } from "@/lib/atoms/org";
import { useAtomValue } from "jotai";
import { DatePicker } from "../date-picker";
import { components } from "@/types/api";
import { Input } from "../ui/input";
import { Textarea } from "../ui/textarea";
import ButtonTile from "../button-tile";
import { Checkbox } from "../ui/checkbox";
import { useDialogs } from "@/lib/dialogs";

export default function CreateFundraising({
  ...props
}: ComponentProps<typeof DialogPrimitive.Root> &
  DialogProps<"CreateFundraising">) {
  const dialogs = useDialogs();

  const org = useAtomValue(SelectedOrgAtom) as {
    avatar_url?: string | null;
    budget: number;
    description: string;
    id: number;
    members: components["schemas"]["OrgUser"][];
    name: string;
    slug: string;
  };

  const fundraisingsMut = $api.useMutation(
    "post",
    "/api/organizations/{org_id}/fundraising",
  );

  const [values, setValues] = useState<{ [key: number]: number | undefined }>(
    {},
  );

  useEffect(() => {
    if (!org?.members) {
      return;
    }

    setValues(
      org.members.reduce(
        (acc, m) => ({ ...acc, [m.id]: undefined }),
        {} as { [key: number]: number | undefined },
      ),
    );
  }, [org?.members]);

  const [startDate, setStartDate] = useState<Date>();
  const [endDate, setEndDate] = useState<Date>();
  const [name, setName] = useState<string>("");
  const [description, setDescription] = useState<string>("");
  const [amount, setAmount] = useState<string>("");
  const [useRemaining, setUseRemaining] = useState<boolean>(true);

  const isFormValid = useMemo(
    () => startDate && endDate && name != "" && description != "",
    [startDate, endDate, name, description],
  );

  async function handleCreate() {
    if (!startDate || !endDate || !name || !description) return;

    await fundraisingsMut.mutateAsync({
      params: {
        path: {
          org_id: org.id,
        },
      },
      body: {
        start_date: startDate.toISOString(),
        payers: Object.keys(values).map((val) => {
          const key = parseInt(val);

          return {
            comment: "",
            user_id: key,
            amount_to_pay: values[key],
            paid_amount: 0,
          };
        }),
        end_date: endDate.toISOString(),
        description,
        name,
        total_amount: parseInt(amount),
      },
    });
  }

  return (
    <BaseDialog
      title="Create new fundraising"
      className="w-fit"
      innerClassName="gap-4"
      {...props}>
      <div className="flex justify-between gap-10">
        <div className="flex flex-col gap-4 w-[400px]">
          <InputWrapper className="w-full">
            <Label>Name</Label>
            <Input
              value={name}
              placeholder="Enter a name"
              onChange={(e) => setName(e.currentTarget.value)}
            />
          </InputWrapper>
          <InputWrapper className="w-full">
            <Label>Description</Label>
            <Textarea
              value={description}
              placeholder="Describe the fundraising"
              onChange={(e) => setDescription(e.currentTarget.value)}
            />
          </InputWrapper>
          <div className="flex gap-3">
            <InputWrapper className="w-full">
              <Label>Amount</Label>
              <Input
                type="number"
                value={amount}
                placeholder="Enter amount to raise"
                onChange={(e) => setAmount(e.currentTarget.value)}
              />
            </InputWrapper>
            <div className="pt-5 w-full flex items-center gap-1">
              <Checkbox
                checked={useRemaining}
                onCheckedChange={(v) => setUseRemaining(v as boolean)}
              />
              <Label>Use remaining budget?</Label>
            </div>
          </div>
          <div className="flex gap-3">
            <InputWrapper className="w-full">
              <Label>Start date</Label>
              <DatePicker value={startDate} onChange={setStartDate} />
            </InputWrapper>
            <InputWrapper className="w-full">
              <Label>End date</Label>
              <DatePicker value={endDate} onChange={setEndDate} />
            </InputWrapper>
          </div>
        </div>
        <div className="flex flex-col gap-2 w-[300px]">
          <Label>Payers:</Label>
          <div className="max-h-full flex-1 overflow-y-auto border-border border rounded-lg p-2 bg-card flex flex-col gap-2">
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
                  <ButtonTile
                    key={member.id}
                    name={member.name}
                    amountToPay={values[member.id]}
                    onClick={async () => {
                      const amount = await dialogs.show("ChangeAmount", {
                        amount: values[member.id] ?? 0,
                      });

                      setValues((v) => ({ ...v, [member.id]: amount }));
                    }}
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
