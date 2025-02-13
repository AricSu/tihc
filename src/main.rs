use clap::Parser;
use tracing_subscriber;
use cli::{Commands, ToolsCommands};
use utils::{fetch_and_save_json, sql_info::{extract_and_replace_sql_info, generate_html_from_sql_info}};

#[derive(Parser)]
#[clap(name = "tihc", version = "1.0", author = "Author: Aric", about = "TiHC CLI Tool\nEmail: askaric@gmail.com\nDoc: https://www.askaric.com/zh/")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    // 初始化日志记录器
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Tools(options) => match options {
            ToolsCommands::Docdb(options) => {
                println!("Running tools docdb...");
                println!("Instance: {}", options.instance);
                println!("NGURL: {}", options.ngurl);
                println!("Start: {}", options.start);
                println!("End: {}", options.end);
                println!("Top: {}", options.top);
                println!("Window: {}", options.window);

                // 在这里添加 docdb 的逻辑
                if let Err(e) = fetch_and_save_json(options.ngurl, options.instance, options.start, options.end, options.top, options.window, "topsql_data.json").await {
                    eprintln!("Error: {}", e);
                }
                match extract_and_replace_sql_info("topsql_data.json") {
                    Ok(sql_infos) => {
                        if let Err(e) = generate_html_from_sql_info(&sql_infos, "topsql_data_result.html", options.start, options.end) {
                            eprintln!("Error: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            ToolsCommands::Collect(options) => {
                println!("Running tools collect...");
                println!("Instance: {}", options.instance);
                println!("NGURL: {}", options.ngurl);

                // 在这里添加 collect 的逻辑
            }
        },
        Commands::Chk(options) => {
            println!("Running chk...");
            println!("Instance: {}", options.instance);
            println!("NGURL: {}", options.ngurl);
            // 在这里添加 chk 的逻辑
        }
    }
}