import { Bell, Database, Palette, Power } from "@phosphor-icons/react";
import { invoke } from "@tauri-apps/api/core";
import { AnimatePresence, motion } from "motion/react";
import { useEffect, useRef, useState } from "react";
import type { Theme } from "../hooks/useDarkMode";
import { useDarkMode } from "../hooks/useDarkMode";
import type { Settings } from "../types";

const ALL_PROFILES = [
	"app_residuals",
	"nvm",
	"npm",
	"bun",
	"pnpm",
	"uv",
	"cargo",
	"ollama",
	"playwright",
	"docker",
	"xcode",
	"homebrew",
];

const PROFILE_LABELS: Record<string, string> = {
	app_residuals: "App residuals",
	nvm: "NVM (Node)",
	npm: "npm cache",
	bun: "bun cache",
	pnpm: "pnpm cache",
	uv: "uv / pip",
	cargo: "Cargo / Rustup",
	ollama: "Ollama models",
	playwright: "Playwright",
	docker: "Docker / OrbStack",
	xcode: "Xcode",
	homebrew: "Homebrew",
};

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

	function toggleProfile(profile: string) {
		if (!settings) return;
		const profiles = settings.profiles.includes(profile)
			? settings.profiles.filter((p) => p !== profile)
			: [...settings.profiles, profile];
		save({ ...settings, profiles });
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
						<Database className="w-4 h-4" />
						Ecosystems to scan
					</h3>
					<div className="grid grid-cols-3 gap-2">
						{ALL_PROFILES.map((p) => (
							<label
								key={p}
								className={`flex items-center gap-2.5 px-3 py-2.5 rounded-xl border cursor-pointer transition-colors ${
									settings.profiles.includes(p)
										? "border-teal bg-teal-subtle"
										: "border-border hover:border-secondary"
								}`}
							>
								<input
									type="checkbox"
									checked={settings.profiles.includes(p)}
									onChange={() => toggleProfile(p)}
									className="w-4 h-4 rounded accent-teal"
								/>
								<span className="text-sm text-body">
									{PROFILE_LABELS[p] || p}
								</span>
							</label>
						))}
					</div>
				</div>

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
