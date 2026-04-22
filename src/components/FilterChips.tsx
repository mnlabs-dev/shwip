import { Package } from "@phosphor-icons/react";
import { CATEGORY_ICONS } from "../categoryIcons";
import type { ScanResult } from "../types";
import { groupByCategory } from "../types";

interface Props {
	results: ScanResult[];
	activeCategories: Set<string>;
	categories: string[];
	onToggle: (cat: string) => void;
}

export function FilterChips({
	results,
	activeCategories,
	categories,
	onToggle,
}: Props) {
	const grouped = groupByCategory(results);
	const allActive = activeCategories.size === categories.length;

	return (
		<div className="flex gap-2 flex-wrap px-6 py-3">
			<button
				type="button"
				className={`inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium rounded-full border transition-colors ${
					allActive
						? "border-teal bg-teal-subtle text-ink"
						: "border-border text-muted hover:text-secondary"
				}`}
				onClick={() => {
					for (const cat of categories) {
						if (!activeCategories.has(cat)) onToggle(cat);
					}
				}}
			>
				All
			</button>
			{categories.map((cat) => {
				const Icon = CATEGORY_ICONS[cat] ?? Package;
				const count = grouped[cat]?.length ?? 0;
				return (
					<button
						key={cat}
						type="button"
						className={`inline-flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium rounded-full border transition-colors ${
							activeCategories.has(cat)
								? "border-teal bg-teal-subtle text-ink"
								: "border-border text-muted hover:text-secondary"
						}`}
						onClick={() => onToggle(cat)}
					>
						<Icon className="w-3.5 h-3.5" />
						{cat}
						<span className="text-muted">({count})</span>
					</button>
				);
			})}
		</div>
	);
}
