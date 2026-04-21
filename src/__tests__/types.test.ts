import { describe, expect, it } from "vitest";
import type { ScanResult } from "../types";
import { formatSize, groupByCategory, totalSize } from "../types";

describe("formatSize", () => {
	it("formats bytes", () => {
		expect(formatSize(500)).toBe("500 B");
	});

	it("formats KB", () => {
		expect(formatSize(2048)).toBe("2 KB");
	});

	it("formats MB", () => {
		expect(formatSize(10_485_760)).toBe("10.0 MB");
	});

	it("formats GB", () => {
		expect(formatSize(2_147_483_648)).toBe("2.0 GB");
	});
});

describe("totalSize", () => {
	it("sums all size_bytes", () => {
		const results: ScanResult[] = [
			{
				category: "a",
				path: "/a",
				size_bytes: 100,
				confidence: "Safe",
				reason: "",
			},
			{
				category: "b",
				path: "/b",
				size_bytes: 200,
				confidence: "Review",
				reason: "",
			},
		];
		expect(totalSize(results)).toBe(300);
	});

	it("returns 0 for empty array", () => {
		expect(totalSize([])).toBe(0);
	});
});

describe("groupByCategory", () => {
	it("groups results by category", () => {
		const results: ScanResult[] = [
			{
				category: "NVM",
				path: "/a",
				size_bytes: 100,
				confidence: "Safe",
				reason: "",
			},
			{
				category: "npm",
				path: "/b",
				size_bytes: 200,
				confidence: "Safe",
				reason: "",
			},
			{
				category: "NVM",
				path: "/c",
				size_bytes: 300,
				confidence: "Review",
				reason: "",
			},
		];
		const groups = groupByCategory(results);
		expect(Object.keys(groups)).toHaveLength(2);
		expect(groups.NVM).toHaveLength(2);
		expect(groups.npm).toHaveLength(1);
	});
});
