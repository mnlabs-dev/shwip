import { Gear, Package } from "@phosphor-icons/react";
import { motion } from "motion/react";
import logoDark from "../assets/logo-dark.svg";
import logoLight from "../assets/logo-light.svg";
import { CATEGORY_ICONS } from "../categoryIcons";

interface Props {
	collapsed: boolean;
	isDark: boolean;
	categories: string[];
	activeCategories: Set<string>;
	onToggleCategory: (cat: string) => void;
	onOpenSettings: () => void;
}

export function Sidebar({
	collapsed,
	isDark,
	categories,
	activeCategories,
	onToggleCategory,
	onOpenSettings,
}: Props) {
	return (
		<motion.aside
			animate={{ width: collapsed ? 64 : 224 }}
			transition={{ duration: 0.3, ease: "easeInOut" }}
			className="border-r border-border flex flex-col py-5 px-4 shrink-0 overflow-hidden"
		>
			<div className="flex items-center gap-2 px-2 mb-7">
				<img
					src={isDark ? logoDark : logoLight}
					alt="shwip logo"
					className="w-7 h-7 rounded shrink-0"
				/>
				{!collapsed && (
					<span className="font-serif text-lg font-semibold tracking-tight whitespace-nowrap">
						shwip
					</span>
				)}
			</div>

			{!collapsed && (
				<div className="text-[10px] font-semibold uppercase tracking-widest text-muted mb-3 px-2">
					Categories
				</div>
			)}

			<div className="space-y-0.5 flex-1 overflow-y-auto">
				{categories.map((cat) => {
					const Icon = CATEGORY_ICONS[cat] ?? Package;
					return (
						<button
							key={cat}
							type="button"
							title={collapsed ? cat : undefined}
							className={`w-full text-left rounded-lg text-sm transition-colors flex items-center gap-2 ${
								collapsed ? "px-2 py-2 justify-center" : "px-3 py-2"
							} ${
								activeCategories.has(cat)
									? "bg-card text-ink font-medium shadow-sm"
									: "text-muted hover:text-secondary hover:bg-card"
							}`}
							onClick={() => onToggleCategory(cat)}
						>
							<Icon className="w-4 h-4 shrink-0" />
							{!collapsed && <span className="truncate">{cat}</span>}
						</button>
					);
				})}
				{categories.length === 0 && !collapsed && (
					<p className="text-xs text-placeholder px-3">
						Run a scan to see categories
					</p>
				)}
			</div>

			<div className="border-t border-border pt-3 mt-3">
				<button
					type="button"
					title={collapsed ? "Settings" : undefined}
					className={`w-full text-left rounded-lg text-sm text-muted hover:text-secondary hover:bg-card transition-colors flex items-center gap-2 ${
						collapsed ? "px-2 py-2 justify-center" : "px-3 py-2"
					}`}
					onClick={onOpenSettings}
				>
					<Gear className="w-4 h-4 shrink-0" />
					{!collapsed && "Settings"}
				</button>
			</div>
		</motion.aside>
	);
}
