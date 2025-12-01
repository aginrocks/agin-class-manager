import { cn } from "@/lib/utils";
import Image from "next/image";

interface OrgOptionProps extends React.HTMLAttributes<HTMLDivElement> {
  image?: string;
  name: string;
  size?: "small" | "large";
}

export default function OrgOption({
  image,
  name,
  size = "large",
  ...props
}: OrgOptionProps) {
  return (
    <div
      {...props}
      className=" rounded-md cursor-pointer flex justify-start items-center gap-2">
      {image ? (
        <img
          src={image}
          alt="organization logo"
          width={size == "large" ? 50 : 30}
          height={size == "large" ? 50 : 30}
          className={cn(
            size == "large" ? "rounded-md" : "rounded-md",
            " max-h-50, max-w-50 aspect-square object-cover",
          )}
        />
      ) : (
        <div
          className={cn(
            "flex justify-center items-center",
            size == "large" ? "w-[50] h-[50]" : "w-[30] h-[30]",
          )}>
          <div className="font-bold text-2xl">{name.charAt(0)}</div>
        </div>
      )}
      <div>
        {size == "large" && (
          <div className="text-muted-foreground text-sm">Organization:</div>
        )}
        <div className="font-semibold">{name}</div>
      </div>
    </div>
  );
}
