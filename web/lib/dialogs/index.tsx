"use client";
import { createContext, FC, ReactNode, useCallback, useState } from "react";
import { DialogName, DialogPayload, DialogProps, DialogReturnValue, DialogsStates, TDialogContext } from "./types";
import { dialogBindings } from "./dialogs";

export const DialogContext = createContext<TDialogContext>({
  show: async () => undefined,
  hide: () => undefined,
});

export default function DialogManager({ children }: { children: ReactNode }) {
  const [dialogsState, setDialogsState] = useState<DialogsStates>({});

  const show = useCallback(<T extends DialogName>(dialogName: T, dialogPayload: DialogPayload<T>): Promise<DialogReturnValue<T>> => {
    return new Promise((resolve) => {
      setDialogsState((old) => ({
        ...old,
        [dialogName]: {
          name: dialogName,
          dialog: dialogPayload,
          visibilityState: 'opened',
          resolve,

        }
      })
    })
}, [])

  const hide = useCallback(<T extends DialogName>(dialogName: T, returnValue: DialogReturnValue<T>,) => {
    setDialogsState((old) => {
      const dialog = old[dialogName];

      if (!dialog || dialog.visibilityState == "closed") {
        return old;
      }

      requestAnimationFrame(() => {
        (dialog.resolve)(returnValue)
      })

      return { ...old, [dialogName]: { ...dialog, visibilityState: "closed" } }
    })
  }, []);


  return <DialogContext.Provider value={{show, hide}}>
    {Object.values(dialogsState).map((d) => {
      const DialogComponent = dialogBindings[d.name] as FC<DialogProps<typeof d.name>>;

      return (
        <DialogComponent
          name={d.name}
          open={d.visibilityState == "open"}
          onOpen
          payload={d.payload}
        />
      )
    })}
  </DialogContext.Provider>
}
