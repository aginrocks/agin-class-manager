"use client";
import { createContext, FC, ReactNode, useCallback, useState } from "react";
import {
  DialogName,
  DialogPayload,
  DialogProps,
  DialogReturnValue,
  DialogsStates,
  TDialogContext,
} from "./types";
import { dialogBindings } from "./dialogs";

export const DialogContext = createContext<TDialogContext>({
  show: async () => undefined,
  hide: () => undefined,
});

export function DialogManager({ children }: { children: ReactNode }) {
  const [dialogsState, setDialogsState] = useState<DialogsStates>({});

  const show = useCallback(
    <T extends DialogName>(
      dialogName: T,
      dialogPayload: DialogPayload<T>,
    ): Promise<DialogReturnValue<T>> => {
      console.log("opening dialog");
      return new Promise((resolve) => {
        setDialogsState((old) => ({
          ...old,
          [dialogName]: {
            name: dialogName,
            payload: dialogPayload,
            visibilityState: "open",
            resolve,
          },
        }));
      });
    },
    [],
  );

  const hide = useCallback(
    <T extends DialogName>(
      dialogName: T,
      returnValue?: DialogReturnValue<T>,
    ) => {
      setDialogsState((old) => {
        const dialog = old[dialogName];

        if (!dialog || dialog.visibilityState == "closed") {
          return old;
        }

        requestAnimationFrame(() => {
          dialog.resolve(returnValue);
        });

        return {
          ...old,
          [dialogName]: { ...dialog, visibilityState: "closed" },
        };
      });
    },
    [],
  );

  return (
    <DialogContext.Provider value={{ show, hide }}>
      {Object.values(dialogsState).map((d) => {
        const DialogComponent = dialogBindings[d.name] as FC<
          DialogProps<typeof d.name>
        >;

        return (
          <DialogComponent
            name={d.name}
            payload={d.payload}
            open={d.visibilityState == "open"}
            onOpenChange={(open) => {
              if (!open) {
                hide(d.name);
              }
            }}
            key={d.name}
          />
        );
      })}
      {children}
    </DialogContext.Provider>
  );
}

export * from "./hook";
