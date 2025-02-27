use clap::Args;
use utils::process::check_log_pattern;

// Options for collect debug
#[derive(Args)]
pub struct DebugOptions {
    #[clap(
        long,
        short = 'c',
        help = "Component to collect, e.g., tidb, tikv, pd, br, tidb-lightning"
    )]
    pub component: String,

    #[clap(
        long,
        short = 'a',
        required_unless_present = "processid",
        conflicts_with = "processid",
        value_name = "IP:PORT",
        help = "Instance address (required for tidb/tikv/pd), format: ip:port, e.g., 127.0.0.1:2379"
    )]
    pub address: Option<String>,

    #[clap(
        long,
        short = 'P',
        required_if_eq("component", "br"),
        required_if_eq("component", "tidb-lightning"),
        conflicts_with = "address",
        help = "Process ID (required for br/tidb-lightning)"
    )]
    pub processid: Option<i32>,

    #[clap(
        long,
        short = 't',
        default_value = "all",
        help = "Collection type, e.g., heap, goroutine, profile"
    )]
    pub collection_type: String,

    #[clap(
        long,
        short = 's',
        default_value = "60",
        help = "Duration in seconds for profiling"
    )]
    pub seconds: usize,

    #[clap(
        long,
        short = 'o',
        default_value = "output",
        help = "Output directory (default './output')"
    )]
    pub output: String,
}

impl DebugOptions {
    pub async fn collect(&self) -> Result<(), anyhow::Error> {
        let url = if let Some(pid) = self.processid {
            utils::profile::send_usr1(pid.try_into().unwrap())?;
            let log_path = utils::process::get_log_path_by_pid(pid)?;
            let port = check_log_pattern(&log_path, "bound pprof to addr")?;
            format!("127.0.0.1:{}", port)
        } else {
            self.address
                .clone()
                .ok_or_else(|| anyhow::anyhow!("地址未提供"))?
        };

        utils::profile::collect_profile(
            &url,
            &self.component,
            &self.collection_type,
            self.seconds.try_into().unwrap(),
        )
        .await?;

        Ok(())
    }
}
