import { useContext } from "react";
import { DialogContext } from ".";

export function useDialogs() {
  const context = useContext(DialogContext);

  if (!context) {
    throw new Error("useDialogs hook must be used within DialogProvider");
  }

  return context;
}
