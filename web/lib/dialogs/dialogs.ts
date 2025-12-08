import CreateOrgDialog from "@/components/dialogs/CreateOrg";
import { DialogComponents, DialogDefinition } from "./types";
import ConfirmDialog from "@/components/dialogs/Confirm";
import CreateSantaDialog from "@/components/dialogs/CreateSanta";
import AddMemberDialog from "@/components/dialogs/AddMember";

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
  CreateSanta: DialogDefinition<{
    payload: undefined;
    returnValue: undefined;
  }>;
  AddMember: DialogDefinition<{
    payload: {
      org_id: number;
    };
    returnValue: undefined;
  }>;
};

export const dialogBindings: DialogComponents = {
  CreateOrg: CreateOrgDialog,
  Confirm: ConfirmDialog,
  CreateSanta: CreateSantaDialog,
  AddMember: AddMemberDialog,
};
