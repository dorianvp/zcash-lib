import { ShineBorder } from "@/components/magicui/shine-border";
import Dither from "@/components/ui/Dither/Dither";
import Link from "next/link";

export default function HomePage() {
	return (
		<main className="flex flex-1 flex-col justify-center text-center">
			<div className="relative border rounded-4xl overflow-hidden m-2 lg:w-1/3 w-2/3 h-[400px] lg:h-[600px] self-center">
				<ShineBorder
					shineColor={["#ffae39", "#F4B728", "#ff8342"]}
					borderWidth={2}
					style={{
						zIndex: 1,
					}}
					duration={6}
				/>
				<Dither
					waveColor={[0.8568, 0.6176, 0.0568]}
					disableAnimation={false}
					enableMouseInteraction={false}
					colorNum={4}
					waveAmplitude={0.4}
					waveFrequency={3.7}
					waveSpeed={0.03}
				/>
			</div>
			<h1 className="m-4 text-2xl font-bold">Under construction 🚧</h1>
			<p className="text-fd-muted-foreground">
				Open{" "}
				<Link
					href="/docs"
					className="text-fd-foreground font-semibold underline"
				>
					/docs
				</Link>{" "}
				to see the documentation.
			</p>
		</main>
	);
}
