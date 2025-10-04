"use client";
import { Button } from "@/components/ui/button";
import Logo from "@/lib/logo";
import { useRouter } from "next/navigation";

export default function Home() {
  const router = useRouter();

  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-gradient-to-br to-[#01791188] from-[#15162c8f] ">
      <div className="flex min-h-max flex-col items-center justify-center p-24 gap-4.5">
        <Logo size={12} />
        <div className="max-w-xl text-center text-lg font-medium">
          Have you ever had a problem with managing finances of your school
          class? Not anymore! This project helps with organizing your classe's
          buget and expenses.
        </div>
        <Button
          size={"lg"}
          className="font-semibold bg-white hover:bg-gray-200 text"
          onClick={() => router.push("/dashboard")}>
          Try now
        </Button>
      </div>
    </div>
  );
}
