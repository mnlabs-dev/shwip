import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import App from "../App";

vi.mock("@tauri-apps/api/core", () => ({
	invoke: vi.fn().mockResolvedValue({ entries: [] }),
}));

vi.mock("@tauri-apps/api/event", () => ({
	listen: vi.fn().mockResolvedValue(() => {}),
}));

describe("App", () => {
	it("renders the title", () => {
		render(<App />);
		expect(screen.getByText("shwip")).toBeInTheDocument();
	});

	it("renders the scan button", () => {
		render(<App />);
		expect(screen.getByRole("button", { name: /scan/i })).toBeInTheDocument();
	});
});
