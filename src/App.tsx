import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { AutomationView } from "./components/AutomationView";
import { Dashboard } from "./components/Dashboard";
import { HistoryView } from "./components/HistoryView";
import { SettingsPanel } from "./components/SettingsPanel";
import type { View } from "./components/Sidebar";
import { Sidebar } from "./components/Sidebar";
import { useDarkMode } from "./hooks/useDarkMode";

function App() {
	const [view, setView] = useState<View>("dashboard");
	const { isDark } = useDarkMode();

	useEffect(() => {
		const unlisten = listen("tray-settings", () => {
			setView("settings");
		});
		return () => {
			unlisten.then((f) => f());
		};
	}, []);

	return (
		<div className="flex h-screen">
			<Sidebar activeView={view} onNavigate={setView} isDark={isDark} />
			<div className="flex-1 min-w-0">
				{view === "dashboard" && <Dashboard />}
				{view === "history" && <HistoryView />}
				{view === "automation" && <AutomationView />}
				{view === "settings" && (
					<SettingsPanel onClose={() => setView("dashboard")} />
				)}
			</div>
		</div>
	);
}

export default App;
