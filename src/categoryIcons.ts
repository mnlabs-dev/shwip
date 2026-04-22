import {
	BeerBottle,
	Brain,
	Browsers,
	Flask,
	Hammer,
	HardDrives,
	Package,
	Trash,
	Wrench,
} from "@phosphor-icons/react";

export const CATEGORY_ICONS: Record<
	string,
	React.ComponentType<{ className?: string }>
> = {
	"App residual": Trash,
	npm: Package,
	"npm cache": Package,
	"bun cache": Package,
	"pnpm cache": Package,
	NVM: Package,
	Cargo: Wrench,
	"uv cache": Flask,
	Ollama: Brain,
	Playwright: Browsers,
	"Docker Local Volumes": HardDrives,
	Xcode: Hammer,
	Homebrew: BeerBottle,
};
