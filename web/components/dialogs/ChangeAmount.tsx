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
import { SelectedOrgAtom } from "@/lib/atoms/org";
import { useSetAtom } from "jotai";
import { useQueryClient } from "@tanstack/react-query";
import { useDialogs } from "@/lib/dialogs";

export default function ChangeAmount({
  ...props
}: ComponentProps<typeof DialogPrimitive.Root> & DialogProps<"ChangeAmount">) {
  const dialogs = useDialogs();

  const [amount, setAmount] = useState("");

  useEffect(() => {
    setAmount(props.payload.amount.toString());
  }, [props?.payload?.amount]);

  return (
    <BaseDialog
      title="Create an organization"
      className="w-100"
      innerClassName="gap-4"
      {...props}>
      <InputWrapper className="w-full">
        <Label htmlFor="avatar">Amount to pay</Label>
        <Input
          id="avatar"
          type="number"
          placeholder="Enter amount"
          value={amount}
          onChange={(c) => setAmount(c.currentTarget.value)}
        />
      </InputWrapper>
      <div className="flex w-full justify-end">
        <Button onClick={() => dialogs.hide("ChangeAmount", parseInt(amount))}>
          Create
        </Button>
      </div>
    </BaseDialog>
  );
}
