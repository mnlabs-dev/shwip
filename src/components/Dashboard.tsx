import { MagnifyingGlass, Moon, Package, Sun } from "@phosphor-icons/react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useState } from "react";
import { CATEGORY_ICONS } from "../categoryIcons";
import { useDarkMode } from "../hooks/useDarkMode";
import type { SortKey } from "../hooks/useFilter";
import { useFilter } from "../hooks/useFilter";
import { useScanProgress } from "../hooks/useScanProgress";
import type { ScanHistory, ScanHistoryEntry, ScanResult } from "../types";
import { formatSize, groupByCategory, totalSize } from "../types";
import { CategoryGroup } from "./CategoryGroup";
import { CleanFlow } from "./CleanFlow";
import { FilterChips } from "./FilterChips";
import { ScanSpinner } from "./ScanSpinner";
import { SpaceDonut } from "./SpaceDonut";

export function Dashboard() {
	const [results, setResults] = useState<ScanResult[]>([]);
	const [scanning, setScanning] = useState(false);
	const [selected, setSelected] = useState<Set<string>>(new Set());
	const [showClean, setShowClean] = useState(false);
	const { completed: scanProgress } = useScanProgress(scanning);
	const totalScanners = 12;
	const { isDark, setTheme } = useDarkMode();
	const [history, setHistory] = useState<ScanHistoryEntry[]>([]);

	const loadHistory = useCallback(() => {
		invoke<ScanHistory>("scan_history")
			.then((h) => setHistory(h.entries))
			.catch(() => {});
	}, []);

	useEffect(() => {
		loadHistory();
	}, [loadHistory]);

	const {
		filtered,
		categories,
		activeCategories,
		toggleCategory,
		sortBy,
		setSortBy,
	} = useFilter(results);

	const scan = useCallback(async () => {
		setScanning(true);
		try {
			const data = await invoke<ScanResult[]>("scan");
			setResults(data);
			setSelected(new Set());
			loadHistory();
		} finally {
			setScanning(false);
		}
	}, [loadHistory]);

	useEffect(() => {
		const unlisten = listen("tray-scan", () => {
			scan();
		});
		return () => {
			unlisten.then((f) => f());
		};
	}, [scan]);

	function toggleItem(path: string) {
		setSelected((prev) => {
			const next = new Set(prev);
			if (next.has(path)) next.delete(path);
			else next.add(path);
			return next;
		});
	}

	function selectAllSafe() {
		const safePaths = filtered
			.filter((r) => r.confidence === "Safe")
			.map((r) => r.path);
		setSelected(new Set(safePaths));
	}

	const selectedItems = results.filter((r) => selected.has(r.path));
	const safeCount = results.filter((r) => r.confidence === "Safe").length;
	const reviewCount = results.filter((r) => r.confidence === "Review").length;
	const total = totalSize(results);

	return (
		<div className="flex-1 overflow-y-auto min-w-0 h-screen">
			{/* Header */}
			<header className="flex items-center justify-between px-6 py-5 border-b border-border sticky top-0 bg-bg z-10">
				<div>
					<h1 className="font-serif text-2xl font-semibold tracking-tight">
						Dashboard
					</h1>
					<p className="text-sm text-muted mt-0.5">Intelligent Mac cleanup</p>
				</div>
				<div className="flex gap-2 items-center">
					<button
						type="button"
						className="p-2 rounded-lg text-muted hover:text-ink hover:bg-card transition-colors"
						onClick={() => setTheme(isDark ? "light" : "dark")}
						title={isDark ? "Switch to light mode" : "Switch to dark mode"}
					>
						{isDark ? (
							<Sun className="w-5 h-5" />
						) : (
							<Moon className="w-5 h-5" />
						)}
					</button>
					{selected.size > 0 && (
						<button
							type="button"
							className="px-4 py-2 text-sm font-semibold rounded-lg bg-teal text-white hover:brightness-110 transition-all"
							onClick={() => setShowClean(true)}
						>
							Clean {selected.size} items
						</button>
					)}
					<button
						type="button"
						className="flex items-center gap-1.5 px-4 py-2 text-sm font-semibold rounded-lg bg-blue text-white hover:brightness-110 transition-all disabled:opacity-40"
						onClick={scan}
						disabled={scanning}
					>
						<MagnifyingGlass className="w-4 h-4" />
						{scanning ? "Scanning..." : "Scan"}
					</button>
				</div>
			</header>

			{/* Stats */}
			{results.length > 0 && (
				<div className="flex items-start gap-4 px-6 py-4">
					<div className="grid grid-cols-2 gap-3 flex-1">
						<div className="bg-card border border-border rounded-2xl p-4">
							<div className="text-xs text-muted mb-1">Total found</div>
							<div className="font-serif text-2xl font-medium">
								{results.length}
							</div>
						</div>
						<div className="bg-card border border-border rounded-2xl p-4">
							<div className="text-xs text-muted mb-1">Reclaimable</div>
							<div className="font-serif text-2xl font-medium">
								{formatSize(total)}
							</div>
						</div>
						<div className="bg-card border border-border rounded-2xl p-4">
							<div className="text-xs text-muted mb-1">Safe</div>
							<div className="font-serif text-2xl font-medium text-green">
								{safeCount}
							</div>
						</div>
						<div className="bg-card border border-border rounded-2xl p-4">
							<div className="text-xs text-muted mb-1">Review</div>
							<div className="font-serif text-2xl font-medium text-orange">
								{reviewCount}
							</div>
						</div>
					</div>
					<SpaceDonut results={results} />
				</div>
			)}

			{/* History */}
			{history.length > 0 && (
				<div className="px-6 py-3">
					<div className="text-[10px] font-semibold uppercase tracking-widest text-muted mb-2">
						Recent scans
					</div>
					<div className="flex gap-2 overflow-x-auto pb-1">
						{history
							.slice(-5)
							.reverse()
							.map((h) => (
								<div
									key={h.timestamp}
									className="bg-card border border-border rounded-xl px-3 py-2 text-xs shrink-0"
								>
									<span className="font-medium">{h.results_count} items</span>
									<span className="text-muted ml-2">
										{formatSize(h.total_bytes)}
									</span>
								</div>
							))}
					</div>
				</div>
			)}

			{/* Category chips */}
			{results.length > 0 && !scanning && (
				<FilterChips
					results={results}
					activeCategories={activeCategories}
					categories={categories}
					onToggle={toggleCategory}
				/>
			)}

			{/* Toolbar */}
			{results.length > 0 && (
				<div className="flex items-center gap-3 px-6 py-2">
					<button
						type="button"
						className="text-xs text-blue hover:underline"
						onClick={selectAllSafe}
					>
						Select all safe
					</button>
					<span className="text-border">|</span>
					<select
						className="text-xs bg-transparent text-secondary outline-none cursor-pointer"
						value={sortBy}
						onChange={(e) => setSortBy(e.target.value as SortKey)}
					>
						<option value="size">Sort by size</option>
						<option value="confidence">Sort by confidence</option>
						<option value="category">Sort by category</option>
					</select>
				</div>
			)}

			{/* Results list */}
			<div>
				{results.length === 0 && !scanning && (
					<div className="flex items-center justify-center min-h-[60vh] text-sm text-placeholder">
						Click Scan to analyze your Mac
					</div>
				)}
				{scanning && (
					<ScanSpinner completed={scanProgress} total={totalScanners} />
				)}
				{results.length > 0 &&
					!scanning &&
					Object.entries(groupByCategory(filtered))
						.sort(([, a], [, b]) => totalSize(b) - totalSize(a))
						.map(([cat, items]) => (
							<CategoryGroup
								key={cat}
								category={cat}
								results={items}
								selected={selected}
								onToggle={toggleItem}
								icon={CATEGORY_ICONS[cat] ?? Package}
							/>
						))}
			</div>

			{showClean && (
				<CleanFlow
					items={selectedItems}
					onDone={scan}
					onClose={() => setShowClean(false)}
				/>
			)}
		</div>
	);
}
