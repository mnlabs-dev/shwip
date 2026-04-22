import { CheckCircle, Info, Lock, Warning } from "@phosphor-icons/react";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
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
	const [explanation, setExplanation] = useState<string | null>(null);
	const [loading, setLoading] = useState(false);

	async function explain() {
		if (explanation) {
			setExplanation(null);
			return;
		}
		setLoading(true);
		try {
			const text = await invoke<string>("explain_item", { item: result });
			setExplanation(text);
		} catch {
			setExplanation("Unable to analyze this item.");
		} finally {
			setLoading(false);
		}
	}

	const Icon = badgeIcons[result.confidence];

	return (
		<div>
			<div className="group flex items-center gap-3 px-4 py-3 border-b border-border last:border-0 hover:bg-bg2 transition-colors">
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
					<Icon className="w-3.5 h-3.5" weight="bold" />
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

				<button
					type="button"
					className="p-1 rounded text-muted opacity-0 group-hover:opacity-100 hover:text-ink hover:bg-card transition-all"
					onClick={explain}
					title="What is this?"
				>
					{loading ? (
						<span className="w-4 h-4 block border-2 border-muted border-t-teal rounded-full animate-spin" />
					) : (
						<Info className="w-4 h-4" />
					)}
				</button>

				<div className="text-sm font-medium text-ink tabular-nums w-20 text-right">
					{formatSize(result.size_bytes)}
				</div>
			</div>

			{explanation && (
				<div className="px-12 py-2 text-xs text-secondary bg-card border-b border-border">
					{explanation}
				</div>
			)}
		</div>
	);
}
