use clap::{Arg, Command};

pub fn clap() {
    let matches = Command::new("toutui")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("update")
                .long("update")
                .help("Run update script via curl")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("uninstall")
                .long("uninstall")
                .help("Run uninstall script via curl")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if matches.get_flag("uninstall") {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(
                r#"bash -c 'tmpfile=$(mktemp) && curl -LsSf https://github.com/AlbanDAVID/Toutui/raw/install_with_cargo/hello_toutui.sh -o "$tmpfile" && bash "$tmpfile" uninstall && rm -f "$tmpfile"'"#,
            )
            .status()
            .expect("failed to run uninstall script");
        return;
    }
    if matches.get_flag("update") {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(
                r#"bash -c 'tmpfile=$(mktemp) && curl -LsSf https://github.com/AlbanDAVID/Toutui/raw/install_with_cargo/hello_toutui.sh -o "$tmpfile" && bash "$tmpfile" update && rm -f "$tmpfile"'"#,
            )
            .status()
            .expect("failed to run update script");
        return;
    }

}

