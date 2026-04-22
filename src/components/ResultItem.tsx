import { CheckCircle, Lock, Warning } from "@phosphor-icons/react";
import type { Confidence, ScanResult } from "../types";
import { formatSize } from "../types";

const badgeStyles: Record<Confidence, string> = {
	Safe: "bg-green-subtle text-green",
	Review: "bg-orange-subtle text-orange",
	Keep: "bg-[var(--color-bg2)] text-muted",
};

const badgeIcons: Record<
	Confidence,
	React.ComponentType<{ className?: string; weight?: "bold" | "regular" }>
> = {
	Safe: CheckCircle,
	Review: Warning,
	Keep: Lock,
};

interface Props {
	result: ScanResult;
	selected: boolean;
	onToggle: (path: string) => void;
}

export function ResultItem({ result, selected, onToggle }: Props) {
	return (
		<div className="flex items-center gap-3 px-4 py-3 border-b border-border last:border-0 hover:bg-bg2 transition-colors">
			{result.confidence !== "Keep" && (
				<input
					type="checkbox"
					checked={selected}
					onChange={() => onToggle(result.path)}
					className="w-4 h-4 rounded accent-teal"
				/>
			)}
			{result.confidence === "Keep" && <div className="w-4" />}

			<span
				className={`inline-flex items-center gap-1 px-2 py-0.5 text-[11px] font-semibold rounded-md ${badgeStyles[result.confidence]}`}
			>
				{(() => {
					const Icon = badgeIcons[result.confidence];
					return <Icon className="w-3.5 h-3.5" weight="bold" />;
				})()}
				{result.confidence}
			</span>

			<div className="flex-1 min-w-0">
				<div
					className="text-sm font-medium text-ink truncate"
					title={result.path}
				>
					{result.path.split("/").pop()}
				</div>
				<div className="text-xs text-secondary truncate">{result.reason}</div>
			</div>

			<div className="text-sm font-medium text-ink tabular-nums w-20 text-right">
				{formatSize(result.size_bytes)}
			</div>
		</div>
	);
}
