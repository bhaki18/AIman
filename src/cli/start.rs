use owo_colors::OwoColorize;

pub fn start() {
    let banner = r#"
     █████╗ ██╗███╗   ███╗ █████╗ ███╗   ██╗
    ██╔══██╗██║████╗ ████║██╔══██╗████╗  ██║
    ███████║██║██╔████╔██║███████║██╔██╗ ██║
    ██╔══██║██║██║╚██╔╝██║██╔══██║██║╚██╗██║
    ██║  ██║██║██║ ╚═╝ ██║██║  ██║██║ ╚████║
    ╚═╝  ╚═╝╚═╝╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝"#;

    println!("{}", banner.bright_cyan().bold());
    println!(
        "    {} {}",
        "Local AI Model & Server Manager".bright_white().bold(),
        "v0.1.0".dimmed()
    );
    println!(
        "    {}\n",
        "────────────────────────────────────────".bright_blue()
    );
}
