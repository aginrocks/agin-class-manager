import { useBindAtoms } from "../atoms/bind";

export default function AtomProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  useBindAtoms();

  return children;
}
