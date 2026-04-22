import { CheckCircle, Timer } from "@phosphor-icons/react";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import type { Settings } from "../types";

const INTERVALS = [
	{ label: "Daily", hours: 24 },
	{ label: "Weekly", hours: 168 },
	{ label: "Monthly", hours: 720 },
] as const;

export function AutomationView() {
	const [settings, setSettings] = useState<Settings | null>(null);

	useEffect(() => {
		invoke<Settings>("load_settings").then(setSettings);
	}, []);

	async function save(updated: Settings) {
		setSettings(updated);
		await invoke("save_settings", { settings: updated }).catch(() => {});
	}

	if (!settings) return null;

	return (
		<div className="h-full overflow-y-auto">
			<header className="px-6 py-5 border-b border-border">
				<h1 className="font-serif text-2xl font-semibold tracking-tight">
					Automation
				</h1>
				<p className="text-sm text-muted mt-0.5">Schedule automatic scans</p>
			</header>

			<div className="px-6 py-6 max-w-lg space-y-6">
				<div className="bg-card border border-border rounded-2xl p-5">
					<div className="flex items-center justify-between mb-4">
						<div className="flex items-center gap-3">
							<Timer className="w-5 h-5 text-teal" />
							<span className="text-sm font-medium text-ink">
								Scheduled scans
							</span>
							{settings.schedule_enabled && (
								<span className="text-[10px] px-2 py-0.5 rounded-full bg-green-subtle text-green font-medium">
									Active
								</span>
							)}
						</div>
						<button
							type="button"
							className={`relative w-11 h-6 rounded-full transition-colors ${
								settings.schedule_enabled ? "bg-teal" : "bg-border"
							}`}
							onClick={() =>
								save({
									...settings,
									schedule_enabled: !settings.schedule_enabled,
								})
							}
						>
							<span
								className={`absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform ${
									settings.schedule_enabled ? "translate-x-5" : ""
								}`}
							/>
						</button>
					</div>

					{settings.schedule_enabled && (
						<div className="space-y-3">
							<p className="text-xs text-muted">Scan frequency</p>
							<div className="flex gap-2">
								{INTERVALS.map((interval) => (
									<button
										key={interval.hours}
										type="button"
										className={`px-4 py-2 text-sm rounded-lg border transition-colors ${
											settings.schedule_interval_hours === interval.hours
												? "border-teal bg-teal-subtle text-ink font-medium"
												: "border-border text-muted hover:text-secondary"
										}`}
										onClick={() =>
											save({
												...settings,
												schedule_interval_hours: interval.hours,
											})
										}
									>
										{interval.label}
									</button>
								))}
							</div>
						</div>
					)}
				</div>

				{settings.schedule_enabled && (
					<div className="flex items-start gap-3 px-4 py-3 bg-green-subtle rounded-xl">
						<CheckCircle className="w-4 h-4 text-green shrink-0 mt-0.5" />
						<p className="text-xs text-secondary">
							Scans run automatically every{" "}
							{settings.schedule_interval_hours <= 24
								? "day"
								: settings.schedule_interval_hours <= 168
									? "week"
									: "month"}
							. Next scan based on last activity.
						</p>
					</div>
				)}
			</div>
		</div>
	);
}
