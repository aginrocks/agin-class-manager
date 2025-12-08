import { ComponentProps } from "react";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { DialogProps } from "@/lib/dialogs/types";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "../ui/alert-dialog";
import { useDialogs } from "@/lib/dialogs";

export default function ConfirmDialog({
  payload,
  ...props
}: ComponentProps<typeof DialogPrimitive.Root> & DialogProps<"Confirm">) {
  const dialogs = useDialogs();

  return (
    <AlertDialog {...props}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>{payload?.title || "Confirm"}</AlertDialogTitle>
          <AlertDialogDescription>
            {payload?.description || "Are you sure you want to do that?"}
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel onClick={() => dialogs.hide("Confirm", false)}>
            {payload?.cancelText || "Cancel"}
          </AlertDialogCancel>
          <AlertDialogAction
            variant={payload?.destructive ? "destructive" : undefined}
            onClick={() => dialogs.hide("Confirm", true)}>
            {payload?.confirmText || "Confirm"}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
