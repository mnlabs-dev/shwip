import { invoke } from "@tauri-apps/api/core";
import { useCallback, useState } from "react";
import type { SortKey } from "../hooks/useFilter";
import { useFilter } from "../hooks/useFilter";
import type { ScanResult } from "../types";
import { formatSize, totalSize } from "../types";
import { CleanFlow } from "./CleanFlow";
import { ResultItem } from "./ResultItem";

interface Props {
	onOpenSettings: () => void;
}

export function Dashboard({ onOpenSettings }: Props) {
	const [results, setResults] = useState<ScanResult[]>([]);
	const [scanning, setScanning] = useState(false);
	const [selected, setSelected] = useState<Set<string>>(new Set());
	const [showClean, setShowClean] = useState(false);

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
		} finally {
			setScanning(false);
		}
	}, []);

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
		<div className="flex h-screen">
			{/* Sidebar */}
			<aside className="w-56 border-r border-border flex flex-col py-5 px-4 shrink-0">
				<div className="flex items-center gap-2.5 px-2 mb-7">
					<div className="w-2.5 h-2.5 rounded bg-teal" />
					<span className="font-serif text-lg font-semibold tracking-tight">
						shwip
					</span>
				</div>

				<div className="text-[10px] font-semibold uppercase tracking-widest text-muted mb-3 px-2">
					Categories
				</div>

				<div className="space-y-0.5 flex-1 overflow-y-auto">
					{categories.map((cat) => (
						<button
							key={cat}
							type="button"
							className={`w-full text-left px-3 py-2 rounded-lg text-sm transition-colors ${
								activeCategories.has(cat)
									? "bg-card text-ink font-medium shadow-sm"
									: "text-muted hover:text-secondary hover:bg-card"
							}`}
							onClick={() => toggleCategory(cat)}
						>
							{cat}
						</button>
					))}
					{categories.length === 0 && (
						<p className="text-xs text-placeholder px-3">
							Run a scan to see categories
						</p>
					)}
				</div>

				<div className="border-t border-border pt-3 mt-3">
					<button
						type="button"
						className="w-full text-left px-3 py-2 rounded-lg text-sm text-muted hover:text-secondary hover:bg-card transition-colors"
						onClick={onOpenSettings}
					>
						Settings
					</button>
				</div>
			</aside>

			{/* Main */}
			<main className="flex-1 flex flex-col min-w-0">
				{/* Header */}
				<header className="flex items-center justify-between px-6 py-5 border-b border-border">
					<div>
						<h1 className="font-serif text-2xl font-semibold tracking-tight">
							Dashboard
						</h1>
						<p className="text-sm text-muted mt-0.5">Intelligent Mac cleanup</p>
					</div>
					<div className="flex gap-2">
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
							className="px-4 py-2 text-sm font-semibold rounded-lg bg-blue text-white hover:brightness-110 transition-all disabled:opacity-40"
							onClick={scan}
							disabled={scanning}
						>
							{scanning ? "Scanning..." : "Scan"}
						</button>
					</div>
				</header>

				{/* Stats */}
				{results.length > 0 && (
					<div className="grid grid-cols-4 gap-3 px-6 py-4">
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
				<div className="flex-1 overflow-y-auto">
					{results.length === 0 && !scanning && (
						<div className="flex items-center justify-center h-full text-sm text-placeholder">
							Click Scan to analyze your Mac
						</div>
					)}
					{scanning && (
						<div className="flex items-center justify-center h-full text-sm text-muted animate-pulse">
							Scanning your Mac...
						</div>
					)}
					{filtered.map((r) => (
						<ResultItem
							key={r.path}
							result={r}
							selected={selected.has(r.path)}
							onToggle={toggleItem}
						/>
					))}
				</div>
			</main>

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
