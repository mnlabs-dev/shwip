import { useMemo, useState } from "react";
import type { ScanResult } from "../types";

export type SortKey = "size" | "confidence" | "category";

export function useFilter(results: ScanResult[]) {
	const [activeCategories, setActiveCategories] = useState<Set<string>>(
		new Set(),
	);
	const [sortBy, setSortBy] = useState<SortKey>("size");

	const categories = useMemo(() => {
		const cats = new Set<string>();
		for (const r of results) cats.add(r.category);
		return Array.from(cats).sort();
	}, [results]);

	const filtered = useMemo(() => {
		let items =
			activeCategories.size === 0
				? results
				: results.filter((r) => activeCategories.has(r.category));

		items = [...items].sort((a, b) => {
			switch (sortBy) {
				case "size":
					return b.size_bytes - a.size_bytes;
				case "confidence": {
					const order = { Safe: 0, Review: 1, Keep: 2 };
					return order[a.confidence] - order[b.confidence];
				}
				case "category":
					return a.category.localeCompare(b.category);
				default:
					return 0;
			}
		});

		return items;
	}, [results, activeCategories, sortBy]);

	function toggleCategory(cat: string) {
		setActiveCategories((prev) => {
			const next = new Set(prev);
			if (next.has(cat)) next.delete(cat);
			else next.add(cat);
			return next;
		});
	}

	return {
		filtered,
		categories,
		activeCategories,
		toggleCategory,
		sortBy,
		setSortBy,
	};
}
