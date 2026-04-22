import { act, renderHook } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { useFilter } from "../hooks/useFilter";
import type { ScanResult } from "../types";

const mockResults: ScanResult[] = [
	{
		category: "NVM",
		path: "/a",
		size_bytes: 500_000_000,
		confidence: "Safe",
		reason: "test",
	},
	{
		category: "npm",
		path: "/b",
		size_bytes: 100_000_000,
		confidence: "Safe",
		reason: "test",
	},
	{
		category: "NVM",
		path: "/c",
		size_bytes: 300_000_000,
		confidence: "Review",
		reason: "test",
	},
];

describe("useFilter", () => {
	it("returns all results when no filter active", () => {
		const { result } = renderHook(() => useFilter(mockResults));
		expect(result.current.filtered).toHaveLength(3);
	});

	it("extracts unique categories", () => {
		const { result } = renderHook(() => useFilter(mockResults));
		expect(result.current.categories).toEqual(["NVM", "npm"]);
	});

	it("filters by category", () => {
		const { result } = renderHook(() => useFilter(mockResults));
		act(() => {
			result.current.toggleCategory("NVM");
		});
		expect(result.current.filtered).toHaveLength(2);
		expect(result.current.filtered.every((r) => r.category === "NVM")).toBe(
			true,
		);
	});

	it("sorts by size descending", () => {
		const { result } = renderHook(() => useFilter(mockResults));
		expect(result.current.filtered[0].size_bytes).toBe(500_000_000);
	});

	it("sorts by confidence", () => {
		const { result } = renderHook(() => useFilter(mockResults));
		act(() => {
			result.current.setSortBy("confidence");
		});
		expect(result.current.filtered[0].confidence).toBe("Safe");
	});
});
