import { useState } from "react";
import { Dashboard } from "./components/Dashboard";
import { SettingsPanel } from "./components/SettingsPanel";

function App() {
	const [view, setView] = useState<"dashboard" | "settings">("dashboard");

	if (view === "settings") {
		return <SettingsPanel onClose={() => setView("dashboard")} />;
	}

	return <Dashboard onOpenSettings={() => setView("settings")} />;
}

export default App;
