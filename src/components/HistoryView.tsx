import { ClockClockwise } from "@phosphor-icons/react";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import type { ScanHistory, ScanHistoryEntry } from "../types";
import { formatSize } from "../types";
import { Sparkline } from "./Sparkline";

function timeAgo(iso: string): string {
	const diff = Date.now() - new Date(iso).getTime();
	const minutes = Math.floor(diff / 60000);
	if (minutes < 60) return `${minutes}m ago`;
	const hours = Math.floor(minutes / 60);
	if (hours < 24) return `${hours}h ago`;
	const days = Math.floor(hours / 24);
	return `${days}d ago`;
}

export function HistoryView() {
	const [entries, setEntries] = useState<ScanHistoryEntry[]>([]);

	useEffect(() => {
		invoke<ScanHistory>("scan_history")
			.then((h) => setEntries(h.entries))
			.catch(() => {});
	}, []);

	if (entries.length === 0) {
		return (
			<div className="flex flex-col items-center justify-center h-full gap-4 text-placeholder">
				<ClockClockwise className="w-12 h-12" />
				<p className="text-sm">
					No scans yet. Run your first scan from the Dashboard.
				</p>
			</div>
		);
	}

	return (
		<div className="h-full overflow-y-auto">
			<header className="px-6 py-5 border-b border-border">
				<h1 className="font-serif text-2xl font-semibold tracking-tight">
					History
				</h1>
				<p className="text-sm text-muted mt-0.5">Past scan results</p>
			</header>

			{entries.length >= 2 && (
				<div className="px-6 pt-4">
					<Sparkline values={entries.map((e) => e.total_bytes)} />
				</div>
			)}

			<div className="px-6 py-4">
				<table className="w-full">
					<thead>
						<tr className="text-xs text-muted uppercase tracking-wider border-b border-border">
							<th className="text-left py-2 font-semibold">Date</th>
							<th className="text-right py-2 font-semibold">Items</th>
							<th className="text-right py-2 font-semibold">Size</th>
							<th className="text-left py-2 pl-6 font-semibold">Categories</th>
						</tr>
					</thead>
					<tbody>
						{[...entries].reverse().map((entry) => (
							<tr
								key={entry.timestamp}
								className="border-b border-border/50 hover:bg-bg2 transition-colors"
							>
								<td className="py-3 text-sm text-ink">
									{timeAgo(entry.timestamp)}
								</td>
								<td className="py-3 text-sm text-ink text-right tabular-nums">
									{entry.results_count}
								</td>
								<td className="py-3 text-sm text-ink text-right tabular-nums">
									{formatSize(entry.total_bytes)}
								</td>
								<td className="py-3 pl-6">
									<div className="flex gap-1.5 flex-wrap">
										{entry.categories.slice(0, 4).map((cat) => (
											<span
												key={cat}
												className="text-[10px] px-2 py-0.5 rounded-full bg-card border border-border text-secondary"
											>
												{cat}
											</span>
										))}
										{entry.categories.length > 4 && (
											<span className="text-[10px] text-muted">
												+{entry.categories.length - 4}
											</span>
										)}
									</div>
								</td>
							</tr>
						))}
					</tbody>
				</table>
			</div>
		</div>
	);
}
