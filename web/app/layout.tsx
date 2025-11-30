import type { Metadata } from "next";
import { Geist, Geist_Mono, Poppins } from "next/font/google";
import "./globals.css";
import { ThemeProvider } from "next-themes";
import { TAGLINE, TITLE } from "@/lib/config";
import Query from "@/lib/providers/Query";
import { ReactNode } from "react";
import { DialogManager } from "@/lib/dialogs";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: TITLE,
  description: TAGLINE,
};

const poppins = Poppins({
  variable: "--font-poppins",
  subsets: ["latin"],
  weight: ["400", "500", "600", "700"],
});

export default function RootLayout({
  children,
}: Readonly<{
  children: ReactNode;
}>) {
  return (
    <Query>
      <html lang="en" suppressHydrationWarning>
        <body
          className={`${geistSans.variable} ${geistMono.variable} ${poppins.variable} antialiased`}>
          <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
            <DialogManager>
              <div className="w-screen h-screen">{children}</div>
            </DialogManager>
          </ThemeProvider>
        </body>
      </html>
    </Query>
  );
}
