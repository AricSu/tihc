use anyhow::Result;
use clap::App;

/// Match commands
pub fn cli_build() -> Result<()> {
    // Get matches
    let yaml = load_yaml!("tihc_cmd.yml");
    let cli_matches = App::from_yaml(yaml).get_matches();

    // Matches Commands
    match cli_matches.subcommand_name() {
        Some("tool-ogg") => {
            if let Some(sub_m) = cli_matches.subcommand_matches("tool-ogg") {
                let out_file_path = sub_m.value_of("output-file-name").unwrap();

                // deal with pattern of "table XXX;" ,using Regex experssion function
                let input_file_path = sub_m.value_of("input-file-name").unwrap();
            }
        }

        _ => {
            // Arguments are required by default (in Clap)
            // This section should never execute and thus
            // should probably be logged in case it executed.
        }
    }

    Ok(())
}
