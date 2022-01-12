use anyhow::Result;
use clap::App;

/// Match commands
pub fn cli_build() -> Result<()> {
    // Get matches
    let yaml = load_yaml!("tihc_cmd.yml");
    let cli_matches = App::from_yaml(yaml).get_matches();
    let name = cli_matches.value_of("grafana_user")
        .expect("This can't be None, we said it was required");
    let pwd = cli_matches.value_of("grafana_pwd")
        .expect("This can't be None, we said it was required");

    Ok(())
}
