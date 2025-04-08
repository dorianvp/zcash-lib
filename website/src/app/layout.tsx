import "./global.css";
import { RootProvider } from "fumadocs-ui/provider";
import { Inter } from "next/font/google";
import type { ReactNode } from "react";
import { Analytics } from "@vercel/analytics/react";
import { SpeedInsights } from "@vercel/speed-insights/next";
import Head from "next/head";

const inter = Inter({
	subsets: ["latin"],
});

export default function Layout({ children }: { children: ReactNode }) {
	return (
		<html lang="en" className={inter.className} suppressHydrationWarning>
			<Head>
				<meta
					name="google-site-verification"
					content="N0KBKuwAzMbWMp-ll7zsxnxRfSi8-DUco79Wjhi7JJI"
				/>
			</Head>
			<body className="flex flex-col min-h-screen">
				<Analytics />
				<SpeedInsights />
				<RootProvider>{children}</RootProvider>
			</body>
		</html>
	);
}
