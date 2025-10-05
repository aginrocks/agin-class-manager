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
      className=" rounded-md cursor-pointer flex justify-start items-center">
      {image ? (
        <Image
          src={image}
          alt="organization logo"
          width={size == "large" ? 50 : 30}
          height={size == "large" ? 50 : 30}
          objectFit="cover"
          className={size == "large" ? "rounded-2xl" : "rounded-md"}
        />
      ) : (
        <div className="w-[50px] h-[50px]">{name.charAt(0)}</div>
      )}
      <div className="ml-4 font-semibold">{name}</div>
    </div>
  );
}
