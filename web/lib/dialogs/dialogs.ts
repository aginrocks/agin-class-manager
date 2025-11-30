import CreateOrgDialog from "@/components/dialogs/CreateOrg";
import { DialogComponents, DialogDefinition } from "./types";

export type Dialogs = {
  CreateOrg: DialogDefinition<{
    payload: undefined;
    returnValue: undefined;
  }>;
};

export const dialogBindings: DialogComponents = {
  CreateOrg: CreateOrgDialog,
};
