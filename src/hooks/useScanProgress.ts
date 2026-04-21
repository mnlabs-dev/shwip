import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useRef, useState } from "react";

interface ScanProgress {
	scanner: string;
	ok: boolean;
}

export function useScanProgress(scanning: boolean) {
	const [completed, setCompleted] = useState<string[]>([]);
	const unlistenRef = useRef<(() => void) | null>(null);

	useEffect(() => {
		if (!scanning) {
			setCompleted([]);
			return;
		}

		let cancelled = false;

		listen<ScanProgress>("scan-progress", (event) => {
			if (!cancelled) {
				setCompleted((prev) => [...prev, event.payload.scanner]);
			}
		}).then((unlisten) => {
			if (cancelled) {
				unlisten();
			} else {
				unlistenRef.current = unlisten;
			}
		});

		return () => {
			cancelled = true;
			unlistenRef.current?.();
			unlistenRef.current = null;
		};
	}, [scanning]);

	const reset = useCallback(() => setCompleted([]), []);

	return { completed, reset };
}
