#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    let _guard = shwip_lib::logger::init_tracing();

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && !args[1].starts_with("--tauri") {
        use clap::Parser;
        let cli = shwip_lib::cli::Cli::parse();
        shwip_lib::cli::run_cli(cli);
    } else {
        shwip_lib::run();
    }
}
