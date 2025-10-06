import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import Logo from "@/components/logo";
import { Field, FieldGroup } from "./ui/field";
import Image from "next/image";
import Link from "next/link";

export function LoginForm({
  className,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div className={cn("flex flex-col gap-6", className)} {...props}>
      <form>
        <div className="flex flex-col gap-6">
          <div className="flex flex-col items-center gap-2">
            <a
              href="#"
              className="flex flex-col items-center gap-2 font-medium">
              <Logo size={10} />
            </a>
          </div>
          <FieldGroup>
            {/* <div className="flex flex-col items-center gap-1 text-center">
              <h1 className="text-2xl font-bold">Login to your account</h1>
              <p className="text-muted-foreground text-sm text-balance">
                Enter your email below to login to your account
              </p>
            </div>
            <div className="flex flex-col gap-3">
              <Field>
                <FieldLabel htmlFor="email">Email</FieldLabel>
                <Input
                  id="email"
                  type="email"
                  placeholder="m@example.com"
                  required
                />
              </Field>
              <Field>
                <div className="flex items-center">
                  <FieldLabel htmlFor="password">Password</FieldLabel>
                   <a
                    href="#"
                    className="ml-auto text-sm underline-offset-4 hover:underline">
                    Forgot your password?
                  </a>
                </div>
                <Input id="password" type="password" required />
              </Field>
            </div>
            <Field>
              <Button type="submit">Login</Button>
            </Field> */}
            {/* <FieldSeparator>Or continue with</FieldSeparator> */}
            <Field>
              <Link
                href="/api/login"
                className="flex justify-center items-center gap-4">
                <Button variant="outline" type="button">
                  <Image
                    alt="agin.rocks logo"
                    src={"/agin-logo.png"}
                    width={40}
                    height={40}
                  />
                  Login with agin.rocks
                </Button>
              </Link>
              {/* <FieldDescription className="text-center">
                Don&apos;t have an account?{" "}
                <a href="#" className="underline underline-offset-4">
                  Sign up
                </a>
              </FieldDescription> */}
            </Field>
          </FieldGroup>
        </div>
      </form>
      <div className="text-muted-foreground *:[a]:hover:text-primary text-center text-xs text-balance *:[a]:underline *:[a]:underline-offset-4">
        By clicking continue, you agree to our <a href="#">Terms of Service</a>{" "}
        and <a href="#">Privacy Policy</a>.
      </div>
    </div>
  );
}
