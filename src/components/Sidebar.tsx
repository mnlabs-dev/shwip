import { ClockClockwise, Gear, House, Timer } from "@phosphor-icons/react";
import logoDark from "../assets/logo-dark.svg";
import logoLight from "../assets/logo-light.svg";

export type View = "dashboard" | "history" | "automation" | "settings";

const NAV_ITEMS: {
	id: View;
	icon: React.ComponentType<{
		className?: string;
		weight?: "fill" | "regular";
	}>;
	label: string;
}[] = [
	{ id: "dashboard", icon: House, label: "Dashboard" },
	{ id: "history", icon: ClockClockwise, label: "History" },
	{ id: "automation", icon: Timer, label: "Automation" },
	{ id: "settings", icon: Gear, label: "Settings" },
];

interface Props {
	activeView: View;
	onNavigate: (view: View) => void;
	isDark: boolean;
}

export function Sidebar({ activeView, onNavigate, isDark }: Props) {
	return (
		<aside className="group w-16 hover:w-48 border-r border-border flex flex-col py-5 shrink-0 transition-all duration-300 overflow-hidden">
			<div className="flex items-center gap-2 px-4 mb-8">
				<img
					src={isDark ? logoDark : logoLight}
					alt="shwip"
					className="w-8 h-8 rounded shrink-0"
				/>
				<span className="font-serif text-lg font-semibold tracking-tight whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity duration-300">
					shwip
				</span>
			</div>

			<nav className="flex-1 flex flex-col gap-1 px-2">
				{NAV_ITEMS.map((item) => {
					const Icon = item.icon;
					const active = activeView === item.id;
					return (
						<button
							key={item.id}
							type="button"
							title={item.label}
							className={`flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm transition-colors ${
								active
									? "bg-card text-ink font-medium shadow-sm"
									: "text-muted hover:text-secondary hover:bg-card"
							}`}
							onClick={() => onNavigate(item.id)}
						>
							<Icon
								className="w-5 h-5 shrink-0"
								weight={active ? "fill" : "regular"}
							/>
							<span className="whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity duration-300">
								{item.label}
							</span>
						</button>
					);
				})}
			</nav>
		</aside>
	);
}
