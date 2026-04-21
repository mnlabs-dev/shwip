import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

function App() {
	const [results, setResults] = useState<string | null>(null);
	const [scanning, setScanning] = useState(false);

	async function scan() {
		setScanning(true);
		try {
			const report = await invoke<string>("scan");
			setResults(report);
		} finally {
			setScanning(false);
		}
	}

	return (
		<main style={{ padding: "2rem", fontFamily: "system-ui" }}>
			<h1>shwip</h1>
			<p>Intelligent Mac cleanup for developers</p>
			<button type="button" onClick={scan} disabled={scanning}>
				{scanning ? "Scanning..." : "Scan"}
			</button>
			{results && (
				<pre style={{ marginTop: "1rem", whiteSpace: "pre-wrap" }}>
					{results}
				</pre>
			)}
		</main>
	);
}

export default App;
