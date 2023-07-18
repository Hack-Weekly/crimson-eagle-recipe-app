import { Inter } from "next/font/google";
import { UserContextProvider } from "@/context/user-state"
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
          <UserContextProvider>
            {children}
          </UserContextProvider>
      </body>
    </html>
  );
}
