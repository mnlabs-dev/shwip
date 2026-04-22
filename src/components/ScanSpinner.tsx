import { CheckCircle } from "@phosphor-icons/react";
import { AnimatePresence, motion } from "motion/react";

interface Props {
	completed: string[];
	total: number;
}

export function ScanSpinner({ completed, total }: Props) {
	const pct = total > 0 ? completed.length / total : 0;
	const circumference = 2 * Math.PI * 38;
	const offset = circumference * (1 - pct);

	return (
		<div className="flex flex-col items-center justify-center h-full gap-6">
			<div className="relative w-24 h-24">
				<motion.svg
					className="w-full h-full -rotate-90"
					viewBox="0 0 96 96"
					animate={{ rotate: 360 }}
					transition={{
						duration: 8,
						repeat: Number.POSITIVE_INFINITY,
						ease: "linear",
					}}
					style={{ rotate: -90 }}
				>
					<circle
						cx="48"
						cy="48"
						r="38"
						fill="none"
						stroke="var(--color-bg2)"
						strokeWidth="4"
					/>
					<circle
						cx="48"
						cy="48"
						r="38"
						fill="none"
						stroke="var(--color-teal)"
						strokeWidth="4"
						strokeLinecap="round"
						strokeDasharray={circumference}
						strokeDashoffset={offset}
						className="transition-all duration-300"
					/>
				</motion.svg>

				<div className="absolute inset-0 flex flex-col items-center justify-center">
					<AnimatePresence mode="popLayout">
						<motion.span
							key={completed.length}
							className="font-serif text-xl font-semibold text-ink"
							initial={{ opacity: 0, y: -6 }}
							animate={{ opacity: 1, y: 0 }}
							exit={{ opacity: 0, y: 6 }}
							transition={{ duration: 0.2 }}
						>
							{completed.length}
						</motion.span>
					</AnimatePresence>
					<span className="text-[10px] text-muted">/ {total}</span>
				</div>
			</div>

			<div className="flex flex-col items-center gap-1.5 min-h-[80px]">
				<AnimatePresence>
					{completed.slice(-4).map((name) => (
						<motion.div
							key={name}
							className="flex items-center gap-1.5 text-xs text-secondary"
							initial={{ opacity: 0, y: -4 }}
							animate={{ opacity: 1, y: 0 }}
							exit={{ opacity: 0 }}
							transition={{ duration: 0.2 }}
						>
							<CheckCircle className="w-3.5 h-3.5 text-green" weight="bold" />
							{name}
						</motion.div>
					))}
				</AnimatePresence>
			</div>
		</div>
	);
}
