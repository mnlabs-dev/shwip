import { CheckCircle, WarningCircle } from "@phosphor-icons/react";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import type { ScanResult } from "../types";
import { formatSize, totalSize } from "../types";

interface Props {
	items: ScanResult[];
	onDone: () => void;
	onClose: () => void;
}

export function CleanFlow({ items, onDone, onClose }: Props) {
	const [phase, setPhase] = useState<"preview" | "cleaning" | "done">(
		"preview",
	);
	const [cleaned, setCleaned] = useState(0);
	const [error, setError] = useState<string | null>(null);

	const safeItems = items.filter((i) => i.confidence === "Safe");
	const total = totalSize(safeItems);

	async function handleClean() {
		setPhase("cleaning");
		try {
			const paths = safeItems.map((i) => i.path);
			const result = await invoke<number>("clean_items", { paths });
			setCleaned(result);
			setPhase("done");
		} catch (e) {
			const msg = e instanceof Error ? e.message : String(e);
			setError(msg);
			setPhase("done");
		}
	}

	return (
		<div
			role="dialog"
			aria-modal="true"
			className="fixed inset-0 z-50 flex items-center justify-center bg-ink/40 backdrop-blur-sm"
			onClick={onClose}
			onKeyDown={(e) => e.key === "Escape" && onClose()}
		>
			<div
				role="document"
				className="bg-card border border-border rounded-2xl p-7 max-w-md w-[90%] shadow-xl"
				onClick={(e) => e.stopPropagation()}
				onKeyDown={() => {}}
			>
				{phase === "preview" && (
					<>
						<h3 className="font-serif text-lg font-semibold mb-2">
							Preview cleanup
						</h3>
						<p className="text-sm text-secondary mb-4">
							{safeItems.length} items, {formatSize(total)} reclaimable. Only
							SAFE items will be moved to trash.
						</p>
						<div className="max-h-48 overflow-y-auto mb-4 border border-border rounded-xl">
							{safeItems.map((item) => (
								<div
									key={item.path}
									className="flex justify-between px-3 py-2 text-xs border-b border-border/50 last:border-0"
								>
									<span className="truncate text-body">
										{item.path.split("/").pop()}
									</span>
									<span className="text-secondary tabular-nums ml-2">
										{formatSize(item.size_bytes)}
									</span>
								</div>
							))}
						</div>
						<div className="flex gap-2 justify-end">
							<button
								type="button"
								className="px-4 py-2 text-sm font-medium rounded-lg border border-border text-body hover:bg-bg2 transition-colors"
								onClick={onClose}
							>
								Cancel
							</button>
							<button
								type="button"
								className="px-4 py-2 text-sm font-semibold rounded-lg bg-teal text-white hover:brightness-110 transition-all"
								onClick={handleClean}
							>
								Clean {safeItems.length} items
							</button>
						</div>
					</>
				)}

				{phase === "cleaning" && (
					<div className="text-center py-8">
						<div className="text-sm text-secondary animate-pulse">
							Cleaning...
						</div>
					</div>
				)}

				{phase === "done" && error && (
					<>
						<div className="flex items-center gap-3 mb-2">
							<WarningCircle className="w-5 h-5 text-red" weight="bold" />
							<h3 className="font-serif text-lg font-semibold">
								Cleanup failed
							</h3>
						</div>
						<p className="text-sm text-secondary mb-4">{error}</p>
						<div className="flex gap-2 justify-end">
							<button
								type="button"
								className="px-4 py-2 text-sm font-medium rounded-lg border border-border text-body hover:bg-bg2 transition-colors"
								onClick={onClose}
							>
								Close
							</button>
							<button
								type="button"
								className="px-4 py-2 text-sm font-semibold rounded-lg bg-teal text-white hover:brightness-110 transition-all"
								onClick={() => {
									setError(null);
									handleClean();
								}}
							>
								Try Again
							</button>
						</div>
					</>
				)}

				{phase === "done" && !error && (
					<>
						<div className="flex items-center gap-3 mb-2">
							<CheckCircle className="w-5 h-5 text-green" weight="bold" />
							<h3 className="font-serif text-lg font-semibold">Done</h3>
						</div>
						<p className="text-sm text-secondary mb-4">
							{cleaned} items moved to trash. You can undo from Finder.
						</p>
						<div className="flex gap-2 justify-end">
							<button
								type="button"
								className="px-4 py-2 text-sm font-semibold rounded-lg bg-teal text-white hover:brightness-110 transition-all"
								onClick={() => {
									onDone();
									onClose();
								}}
							>
								OK
							</button>
						</div>
					</>
				)}
			</div>
		</div>
	);
}
