import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { Dashboard } from "./components/Dashboard";
import { SettingsPanel } from "./components/SettingsPanel";

function App() {
	const [view, setView] = useState<"dashboard" | "settings">("dashboard");

	useEffect(() => {
		const unlisten = listen("tray-settings", () => {
			setView("settings");
		});
		return () => {
			unlisten.then((f) => f());
		};
	}, []);

	if (view === "settings") {
		return <SettingsPanel onClose={() => setView("dashboard")} />;
	}

	return <Dashboard onOpenSettings={() => setView("settings")} />;
}

export default App;
