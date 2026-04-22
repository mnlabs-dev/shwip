interface Props {
	values: number[];
}

export function Sparkline({ values }: Props) {
	if (values.length < 2) return null;

	const max = Math.max(...values);
	const min = Math.min(...values);
	const range = max - min || 1;
	const width = 100;
	const height = 60;
	const padding = 4;

	const points = values
		.map((v, i) => {
			const x = (i / (values.length - 1)) * (width - 2 * padding) + padding;
			const y = height - padding - ((v - min) / range) * (height - 2 * padding);
			return `${x},${y}`;
		})
		.join(" ");

	const fillPoints = `${padding},${height - padding} ${points} ${width - padding},${height - padding}`;

	return (
		<svg
			viewBox={`0 0 ${width} ${height}`}
			className="w-full h-16"
			preserveAspectRatio="none"
			role="img"
			aria-label="Scan history trend"
		>
			<polygon
				points={fillPoints}
				fill="var(--color-teal-subtle)"
				stroke="none"
			/>
			<polyline
				points={points}
				fill="none"
				stroke="var(--color-teal)"
				strokeWidth="1.5"
				strokeLinejoin="round"
				strokeLinecap="round"
			/>
		</svg>
	);
}
