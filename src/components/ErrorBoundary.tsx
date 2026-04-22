import React from "react";

interface Props {
	children: React.ReactNode;
}

interface State {
	hasError: boolean;
	error: Error | null;
}

function ErrorFallback({
	error,
	onReset,
}: {
	error: Error | null;
	onReset: () => void;
}) {
	return (
		<div className="flex h-full items-center justify-center bg-bg">
			<div className="max-w-md w-full mx-4 p-6 bg-card border border-border rounded-2xl">
				<div className="flex items-center gap-3 mb-4">
					<svg
						width="20"
						height="20"
						viewBox="0 0 20 20"
						fill="none"
						className="shrink-0 text-orange"
					>
						<title>Warning</title>
						<path
							d="M10 2L18 16H2L10 2Z"
							stroke="currentColor"
							strokeWidth="1.5"
							strokeLinejoin="round"
						/>
						<path
							d="M10 8V11"
							stroke="currentColor"
							strokeWidth="1.5"
							strokeLinecap="round"
						/>
						<circle cx="10" cy="13.5" r="0.75" fill="currentColor" />
					</svg>
					<h2 className="font-serif text-base font-semibold text-ink">
						Something went wrong
					</h2>
				</div>
				{error && (
					<p className="text-sm text-muted mb-6 break-words">
						{error.message.length > 200
							? `${error.message.slice(0, 200)}...`
							: error.message}
					</p>
				)}
				<div className="flex gap-3">
					<button
						type="button"
						onClick={onReset}
						className="flex-1 px-4 py-2 bg-teal-btn text-white rounded-lg text-sm font-semibold hover:-translate-y-px hover:shadow-md transition-all"
					>
						Try Again
					</button>
					<button
						type="button"
						onClick={() => window.location.reload()}
						className="flex-1 px-4 py-2 bg-card border border-border text-ink rounded-lg text-sm font-semibold hover:bg-bg2 transition-colors"
					>
						Reload App
					</button>
				</div>
			</div>
		</div>
	);
}

export class ErrorBoundary extends React.Component<Props, State> {
	state: State = { hasError: false, error: null };

	static getDerivedStateFromError(error: Error): State {
		return { hasError: true, error };
	}

	componentDidCatch(error: Error, info: React.ErrorInfo) {
		console.error("ErrorBoundary:", error, info.componentStack);
	}

	handleReset = () => {
		this.setState({ hasError: false, error: null });
	};

	render() {
		if (this.state.hasError) {
			return (
				<ErrorFallback error={this.state.error} onReset={this.handleReset} />
			);
		}
		return this.props.children;
	}
}
