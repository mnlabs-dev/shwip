import { CaretDown, CaretRight } from "@phosphor-icons/react";
import { useState } from "react";
import type { ScanResult } from "../types";
import { formatSize, totalSize } from "../types";
import { ResultItem } from "./ResultItem";

interface Props {
	category: string;
	results: ScanResult[];
	selected: Set<string>;
	onToggle: (path: string) => void;
	icon: React.ComponentType<{ className?: string }>;
}

export function CategoryGroup({
	category,
	results,
	selected,
	onToggle,
	icon: Icon,
}: Props) {
	const [expanded, setExpanded] = useState(true);
	const groupSize = totalSize(results);

	return (
		<div>
			<button
				type="button"
				className="w-full flex items-center gap-2 px-4 py-2.5 text-sm font-medium text-ink hover:bg-bg2 transition-colors"
				onClick={() => setExpanded((v) => !v)}
			>
				{expanded ? (
					<CaretDown className="w-3.5 h-3.5 text-muted" />
				) : (
					<CaretRight className="w-3.5 h-3.5 text-muted" />
				)}
				<Icon className="w-4 h-4 text-secondary" />
				<span>{category}</span>
				<span className="text-xs text-muted ml-1">({results.length})</span>
				<span className="ml-auto text-xs text-secondary tabular-nums">
					{formatSize(groupSize)}
				</span>
			</button>
			{expanded &&
				results.map((r) => (
					<ResultItem
						key={r.path}
						result={r}
						selected={selected.has(r.path)}
						onToggle={onToggle}
					/>
				))}
		</div>
	);
}
