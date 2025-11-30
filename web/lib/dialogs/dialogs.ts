import CreateOrgDialog from "@/components/dialogs/CreateOrg";
import { DialogComponents, DialogDefinition } from "./types";
import ConfirmDialog from "@/components/dialogs/Confirm";

export type Dialogs = {
  CreateOrg: DialogDefinition<{
    payload: undefined;
    returnValue: undefined;
  }>;
  Confirm: DialogDefinition<{
    payload: {
      title?: string;
      description?: string;
      confirmText?: string;
      cancelText?: string;
      destructive?: boolean;
    };
    returnValue: boolean;
  }>;
};

export const dialogBindings: DialogComponents = {
  CreateOrg: CreateOrgDialog,
  Confirm: ConfirmDialog,
};
