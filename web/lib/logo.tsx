import { IconBook } from "@tabler/icons-react";
import Image from "next/image";
import { useMemo } from "react";

interface LogoProps {
  size?: number;
}

export default function Logo({ size = 3 }: LogoProps) {
  const { width, height } = useMemo(() => {
    return { width: 50 * size, height: 3.172619048 * size };
  }, [size]);

  return (
    <div>
      <Image
        src="/logo.svg"
        alt="Logo"
        width={width}
        height={height}
        className=""
      />
    </div>
  );
}
