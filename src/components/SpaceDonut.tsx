import type { ScanResult } from "../types";
import { formatSize, groupByCategory, totalSize } from "../types";

const CATEGORY_COLORS: Record<string, string> = {
	"App residual": "var(--color-coral)",
	"npm cache": "var(--color-blue)",
	"bun cache": "var(--color-blue)",
	"pnpm cache": "var(--color-blue)",
	NVM: "var(--color-blue)",
	Cargo: "var(--color-orange)",
	"uv cache": "var(--color-violet)",
	Ollama: "var(--color-violet)",
	Playwright: "var(--color-teal)",
	"Docker Local Volumes": "var(--color-violet)",
	Xcode: "var(--color-teal)",
	Homebrew: "var(--color-green)",
};

interface Props {
	results: ScanResult[];
}

export function SpaceDonut({ results }: Props) {
	const total = totalSize(results);
	if (total === 0) {
		return (
			<svg
				viewBox="0 0 200 200"
				className="w-48 h-48"
				role="img"
				aria-label="No space data"
			>
				<circle
					cx="100"
					cy="100"
					r="70"
					fill="none"
					stroke="var(--color-border)"
					strokeWidth="24"
				/>
				<text
					x="100"
					y="100"
					textAnchor="middle"
					dominantBaseline="central"
					className="fill-muted text-xs"
				>
					No data
				</text>
			</svg>
		);
	}

	const grouped = groupByCategory(results);
	const entries = Object.entries(grouped)
		.map(([cat, items]) => ({
			category: cat,
			size: totalSize(items),
			color: CATEGORY_COLORS[cat] ?? "var(--color-muted)",
		}))
		.sort((a, b) => b.size - a.size);

	if (entries.length === 1) {
		return (
			<svg
				viewBox="0 0 200 200"
				className="w-48 h-48"
				role="img"
				aria-label="Space breakdown chart"
			>
				<circle
					cx="100"
					cy="100"
					r="70"
					fill="none"
					stroke={entries[0].color}
					strokeWidth="24"
				/>
				<text
					x="100"
					y="94"
					textAnchor="middle"
					dominantBaseline="central"
					className="fill-ink font-serif text-lg font-semibold"
					style={{ fontSize: "18px" }}
				>
					{formatSize(total)}
				</text>
				<text
					x="100"
					y="114"
					textAnchor="middle"
					dominantBaseline="central"
					className="fill-muted"
					style={{ fontSize: "10px" }}
				>
					reclaimable
				</text>
			</svg>
		);
	}

	const arcs: { d: string; color: string; category: string }[] = [];
	let startAngle = -90;

	for (const entry of entries) {
		const sweep = (entry.size / total) * 360;
		if (sweep < 0.5) continue;

		const start = (startAngle * Math.PI) / 180;
		const end = ((startAngle + sweep) * Math.PI) / 180;
		const largeArc = sweep > 180 ? 1 : 0;

		const x1 = 100 + 70 * Math.cos(start);
		const y1 = 100 + 70 * Math.sin(start);
		const x2 = 100 + 70 * Math.cos(end);
		const y2 = 100 + 70 * Math.sin(end);

		arcs.push({
			d: `M ${x1} ${y1} A 70 70 0 ${largeArc} 1 ${x2} ${y2}`,
			color: entry.color,
			category: entry.category,
		});

		startAngle += sweep;
	}

	return (
		<svg
			viewBox="0 0 200 200"
			className="w-48 h-48"
			role="img"
			aria-label="Space breakdown chart"
		>
			{arcs.map((arc) => (
				<path
					key={arc.category}
					d={arc.d}
					fill="none"
					stroke={arc.color}
					strokeWidth="24"
					strokeLinecap="butt"
				/>
			))}
			<text
				x="100"
				y="94"
				textAnchor="middle"
				dominantBaseline="central"
				className="fill-ink font-serif font-semibold"
				style={{ fontSize: "18px" }}
			>
				{formatSize(total)}
			</text>
			<text
				x="100"
				y="114"
				textAnchor="middle"
				dominantBaseline="central"
				className="fill-muted"
				style={{ fontSize: "10px" }}
			>
				reclaimable
			</text>
		</svg>
	);
}
