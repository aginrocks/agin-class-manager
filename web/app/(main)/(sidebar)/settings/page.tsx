import InputWrapper from "@/components/input-wrapper";
import { SiteHeader } from "@/components/site-header";
import { Button } from "@/components/ui/button";

export default function SettingsPage() {
  return (
    <>
      <SiteHeader title="Settings" />
      <div className="flex flex-1 flex-col p-4">
        <InputWrapper>
          <Button variant={"destructive"}>Delete organization</Button>
        </InputWrapper>
      </div>
    </>
  );
}
