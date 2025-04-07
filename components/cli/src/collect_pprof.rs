use clap::{builder::TypedValueParser, Args};
use collect::pprof::{HttpProfileCollector, ProfileCollector};
use utils::process::check_log_pattern;

// Options for collect debug
#[derive(Args)]
pub struct PprofOptions {
    #[clap(
        long,
        short = 'c',
        help = "Component to collect, e.g., tidb, tikv, pd, br",
        value_parser = clap::builder::PossibleValuesParser::new(["tidb", "tikv", "pd", "br"])
            .map(|s| s.to_string())
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
        conflicts_with = "address",
        help = "Process ID (required for br)"
    )]
    pub processid: Option<i32>,

    #[clap(
        long,
        short = 't',
        required = true,
        value_parser = clap::builder::PossibleValuesParser::new(["config", "profile", "mutex", "heap", "goroutine"]),
        help = "Collection type (config, profile, mutex, heap, goroutine)"
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
        required = false,
        help = "storage path for the collected data (optional for goroutine collection)"
    )]
    pub output: Option<String>,
}

impl PprofOptions {
    fn validate(&self) -> Result<(), anyhow::Error> {
        // For TiKV, only heap and profile collection types are supported
        if self.component == "tikv"
            && !["config", "heap", "profile"].contains(&self.collection_type.as_str())
        {
            return Err(anyhow::anyhow!(
                "TiKV only supports 'heap' and 'profile' collection types"
            ));
        }

        // For BR, only "profile", "mutex", "heap", "goroutine" collection types are supported
        if self.component == "br"
            && !["profile", "mutex", "heap", "goroutine"].contains(&self.collection_type.as_str())
        {
            return Err(anyhow::anyhow!(
                "BR only supports 'profile', 'mutex', 'heap', and 'goroutine' collection types"
            ));
        }

        Ok(())
    }

    pub async fn collect(&self) -> Result<(), anyhow::Error> {
        self.validate()?;

        let url = if let Some(pid) = self.processid {
            utils::process::send_usr1_signal(pid)?;
            let log_path = utils::process::get_log_path_by_pid(pid)?;
            let port = check_log_pattern(&log_path, "bound pprof to addr")?;
            format!("127.0.0.1:{}", port)
        } else {
            self.address
                .clone()
                .ok_or_else(|| anyhow::anyhow!("you have to provide the IP:port"))?
        };

        HttpProfileCollector::new(
            &url,
            &self.component,
            &self.collection_type,
            self.seconds.try_into().unwrap(),
            self.output.clone().unwrap_or_default(),
        )
        .collect()
        .await?;

        Ok(())
    }
}
