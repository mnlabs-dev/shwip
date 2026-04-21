export type Confidence = "Safe" | "Review" | "Keep";

export interface ScanResult {
	category: string;
	path: string;
	size_bytes: number;
	confidence: Confidence;
	reason: string;
}

export interface Settings {
	profiles: string[];
	exclusions: string[];
	schedule_enabled: boolean;
	schedule_interval_hours: number;
	autostart: boolean;
	show_notifications: boolean;
}

export function formatSize(bytes: number): string {
	if (bytes >= 1_073_741_824) return `${(bytes / 1_073_741_824).toFixed(1)} GB`;
	if (bytes >= 1_048_576) return `${(bytes / 1_048_576).toFixed(1)} MB`;
	if (bytes >= 1_024) return `${(bytes / 1_024).toFixed(0)} KB`;
	return `${bytes} B`;
}

export function totalSize(results: ScanResult[]): number {
	return results.reduce((sum, r) => sum + r.size_bytes, 0);
}

export interface ScanHistoryEntry {
	timestamp: string;
	results_count: number;
	total_bytes: number;
	categories: string[];
}

export interface ScanHistory {
	entries: ScanHistoryEntry[];
}

export function groupByCategory(
	results: ScanResult[],
): Record<string, ScanResult[]> {
	const groups: Record<string, ScanResult[]> = {};
	for (const r of results) {
		if (!groups[r.category]) groups[r.category] = [];
		groups[r.category].push(r);
	}
	return groups;
}
