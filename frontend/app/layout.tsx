import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "BetSure Analytics - Football Predictions",
  description: "AI-powered football match predictions with premium insights",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="bg-gray-50">{children}</body>
    </html>
  );
}
