import { Bell, Brain, Palette, Power } from "@phosphor-icons/react";
import { invoke } from "@tauri-apps/api/core";
import { AnimatePresence, motion } from "motion/react";
import { useEffect, useRef, useState } from "react";
import type { Theme } from "../hooks/useDarkMode";
import { useDarkMode } from "../hooks/useDarkMode";
import type { Settings } from "../types";

interface Props {
	onClose: () => void;
}

export function SettingsPanel({ onClose }: Props) {
	const [settings, setSettings] = useState<Settings | null>(null);
	const [showSaved, setShowSaved] = useState(false);
	const { theme, setTheme } = useDarkMode();
	const saveTimer = useRef<ReturnType<typeof setTimeout>>(undefined);

	useEffect(() => {
		invoke<Settings>("load_settings").then(setSettings);
	}, []);

	async function save(updated: Settings) {
		setSettings(updated);
		try {
			await invoke("save_settings", { settings: updated });
			setShowSaved(true);
			clearTimeout(saveTimer.current);
			saveTimer.current = setTimeout(() => setShowSaved(false), 1500);
		} catch {
			// silent
		}
	}

	if (!settings) return null;

	return (
		<div className="flex flex-col h-full">
			<div className="flex items-center justify-between px-5 py-4 border-b border-border">
				<h2 className="font-serif text-base font-semibold">Settings</h2>
				<button
					type="button"
					className="text-secondary hover:text-ink text-sm"
					onClick={onClose}
				>
					Close
				</button>
			</div>

			<div className="flex-1 overflow-y-auto px-5 py-4 space-y-8">
				<div>
					<h3 className="text-xs font-semibold uppercase tracking-wider text-muted mb-3 flex items-center gap-2">
						<Power className="w-4 h-4" />
						Autostart
					</h3>
					<label className="flex items-center gap-3 cursor-pointer">
						<input
							type="checkbox"
							checked={settings.autostart}
							onChange={() =>
								save({ ...settings, autostart: !settings.autostart })
							}
							className="w-4 h-4 rounded accent-teal"
						/>
						<span className="text-sm text-body">Launch at login</span>
					</label>
				</div>

				<div>
					<h3 className="text-xs font-semibold uppercase tracking-wider text-muted mb-3 flex items-center gap-2">
						<Bell className="w-4 h-4" />
						Notifications
					</h3>
					<label className="flex items-center gap-3 cursor-pointer">
						<input
							type="checkbox"
							checked={settings.show_notifications}
							onChange={() =>
								save({
									...settings,
									show_notifications: !settings.show_notifications,
								})
							}
							className="w-4 h-4 rounded accent-teal"
						/>
						<span className="text-sm text-body">Show scan notifications</span>
					</label>
				</div>

				<div>
					<h3 className="text-xs font-semibold uppercase tracking-wider text-muted mb-3 flex items-center gap-2">
						<Palette className="w-4 h-4" />
						Theme
					</h3>
					<div className="flex gap-2">
						{(["system", "light", "dark"] as Theme[]).map((t) => (
							<button
								key={t}
								type="button"
								className={`px-3 py-1.5 text-sm rounded-lg border transition-colors ${
									theme === t
										? "border-teal bg-teal-subtle text-ink font-medium"
										: "border-border text-muted hover:text-secondary"
								}`}
								onClick={() => setTheme(t)}
							>
								{t === "system" ? "System" : t === "light" ? "Light" : "Dark"}
							</button>
						))}
					</div>
				</div>

				<div>
					<h3 className="text-xs font-semibold uppercase tracking-wider text-muted mb-3 flex items-center gap-2">
						<Brain className="w-4 h-4" />
						AI Analysis
					</h3>
					<p className="text-xs text-muted mb-3">
						Use local Ollama to explain scan results. No data leaves your Mac.
					</p>
					<label className="flex items-center gap-3 cursor-pointer">
						<input
							type="checkbox"
							checked={false}
							onChange={() => {}}
							className="w-4 h-4 rounded accent-teal"
							disabled
						/>
						<span className="text-sm text-body">Enable AI explanations</span>
						<span className="text-[10px] px-2 py-0.5 rounded-full bg-orange-subtle text-orange font-medium">
							Coming soon
						</span>
					</label>
				</div>
			</div>

			<AnimatePresence>
				{showSaved && (
					<motion.div
						className="px-5 py-2 text-xs text-green bg-green-subtle"
						initial={{ opacity: 0, y: 4 }}
						animate={{ opacity: 1, y: 0 }}
						exit={{ opacity: 0, y: 4 }}
						transition={{ duration: 0.2 }}
					>
						Settings saved
					</motion.div>
				)}
			</AnimatePresence>
		</div>
	);
}
