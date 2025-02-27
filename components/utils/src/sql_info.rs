use chrono::{DateTime, Local, Utc};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::BufReader;
use tools::replace_all_column_names;
use tracing::{info, warn};

use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Serialize)]
pub struct SqlInfo {
    pub sql_digest: String,
    pub original_sql_text: String,
    pub replaced_sql_text: Option<String>,
    pub replaced_sql_digest: Option<String>,
    pub error: Option<String>,
}

use anyhow::{Context, Result};

pub fn extract_and_replace_sql_info(file_path: &str) -> Result<Vec<SqlInfo>> {
    let file = File::open(file_path).with_context(|| format!("无法打开文件: {}", file_path))?;
    let reader = BufReader::new(file);
    let data: Value = serde_json::from_reader(reader).with_context(|| "无法解析 JSON 数据")?;

    let mut sql_infos = Vec::new();

    if let Some(data_array) = data["data"].as_array() {
        for item in data_array {
            if let Some(plans) = item.get("plans").and_then(|p| p.as_array()) {
                for _plan in plans {
                    let sql_digest = item
                        .get("sql_digest")
                        .and_then(|s| s.as_str())
                        .unwrap_or("No sql_digest")
                        .to_string();
                    let sql_text = item
                        .get("sql_text")
                        .and_then(|s| s.as_str())
                        .unwrap_or("No sql_text")
                        .to_string();

                    if sql_text.trim().is_empty() {
                        warn!("Skipping invalid SQL text for sql_digest: {}", sql_digest);
                        continue;
                    }

                    let (replaced_sql_text, replaced_sql_digest, error) =
                        match replace_all_column_names(&sql_text) {
                            Ok(sql) => {
                                let mut hasher = Sha256::new();
                                hasher.update(sql.as_bytes());
                                let result = hasher.finalize();
                                let digest = format!("{:x}", result);
                                (Some(sql), Some(digest), None)
                            }
                            Err(e) => {
                                warn!(
                                    "Failed to replace column names for sql_digest: {}: {}",
                                    sql_digest, e
                                );
                                (None, None, Some(e.to_string()))
                            }
                        };

                    sql_infos.push(SqlInfo {
                        sql_digest,
                        original_sql_text: sql_text,
                        replaced_sql_text,
                        replaced_sql_digest,
                        error,
                    });
                }
            }
        }
    }

    info!("Data successfully extracted and processed");
    Ok(sql_infos)
}

pub fn calculate_top_replaced_sql_digest(sql_infos: &[SqlInfo]) -> Vec<(String, usize, f64)> {
    let mut digest_count = HashMap::new();
    let total_count = sql_infos.len();

    for info in sql_infos {
        if let Some(ref digest) = info.replaced_sql_digest {
            *digest_count.entry(digest.clone()).or_insert(0) += 1;
        }
    }

    let mut digest_count_vec: Vec<(String, usize, f64)> = digest_count
        .into_iter()
        .map(|(digest, count)| {
            let frequency = (count as f64 / total_count as f64) * 100.0;
            (digest, count, frequency)
        })
        .collect();

    digest_count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    digest_count_vec.truncate(10);

    digest_count_vec
}

pub fn generate_html_from_sql_info(
    sql_infos: &[SqlInfo],
    html_file_path: &str,
    start_time: u64,
    end_time: u64,
) -> Result<()> {
    let mut output_file = File::create(html_file_path)
        .with_context(|| format!("无法创建 HTML 文件: {}", html_file_path))?;

    let now: DateTime<Utc> = Utc::now();

    writeln!(output_file, "<html><head><style>").with_context(|| "写入 HTML 头部失败")?;
    writeln!(output_file, "body {{ font-family: Arial, sans-serif; }}")?;
    writeln!(output_file, "h1 {{ text-align: center; }}")?;
    writeln!(
        output_file,
        "table {{ border-collapse: collapse; width: 100%; margin: 20px 0; }}"
    )?;
    writeln!(
        output_file,
        "th, td {{ border: 1px solid #ddd; padding: 8px; }}"
    )?;
    writeln!(
        output_file,
        "th {{ background-color: #f2f2f2; text-align: left; }}"
    )?;
    writeln!(output_file, "tr:hover {{ background-color: #f5f5f5; }}")?;
    writeln!(output_file, "a {{ text-decoration: none; color: blue; }}")?;
    writeln!(output_file, "</style></head><body>")?;
    writeln!(output_file, "<h1>SQL Diagnostic Report</h1>")?;
    writeln!(
        output_file,
        "<p>Generated on: {}</p>",
        now.format("%Y-%m-%d %H:%M:%S UTC")
    )?;

    writeln!(output_file, "<h2>Summary</h2>")?;
    writeln!(output_file, "<p>Total SQL Samples: {}</p>", sql_infos.len())?;
    let start_time_local = DateTime::from_timestamp(start_time as i64, 0)
        .map(|dt| dt.with_timezone(&Local))
        .ok_or_else(|| anyhow::anyhow!("无效的开始时间戳"))?;

    let end_time_local = DateTime::from_timestamp(end_time as i64, 0)
        .map(|dt| dt.with_timezone(&Local))
        .ok_or_else(|| anyhow::anyhow!("无效的结束时间戳"))?;

    writeln!(
        output_file,
        "<p>Statistics Time Range: {} to {} (Timezone: {})</p>",
        start_time_local.format("%Y-%m-%d %H:%M:%S"),
        end_time_local.format("%Y-%m-%d %H:%M:%S"),
        Local::now().format("%Z")
    )?;

    writeln!(
        output_file,
        "<h2>Top 10 Replaced SQL Digest by Frequency</h2>"
    )?;
    writeln!(output_file, "<table>")?;
    writeln!(output_file, "<tr><th>Replaced SQL Digest</th><th>Count</th><th>Percentage</th><th>SQL Digests</th></tr>")?;
    let top_replaced_sql_digests = calculate_top_replaced_sql_digest(sql_infos);
    for (digest, count, percent) in top_replaced_sql_digests {
        let sql_digests: Vec<String> = sql_infos
            .iter()
            .filter(|info| info.replaced_sql_digest.as_deref() == Some(&digest))
            .map(|info| {
                format!(
                    "<a href=\"#sql{}\">{}</a>",
                    info.sql_digest, info.sql_digest
                )
            })
            .collect();
        let sql_digests_str = sql_digests.join(", ");
        writeln!(
            output_file,
            "<tr><td><a href=\"#sql{}\">{}</a></td><td>{}</td><td>{:.2}%</td><td>{}</td></tr>",
            digest, digest, count, percent, sql_digests_str
        )?;
    }
    writeln!(output_file, "</table>")?;

    writeln!(output_file, "<h2>SQL Details</h2>")?;
    writeln!(output_file, "<table>")?;
    writeln!(output_file, "<tr><th>SQL Digest</th><th>Original SQL Text</th><th>Replaced SQL Text</th><th>Replaced SQL Digest</th><th>Error</th></tr>")?;

    for info in sql_infos {
        writeln!(
            output_file,
            "<tr id=\"sql{}\"><td><a href=\"#sql{}\">{}</a></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            info.sql_digest, info.sql_digest, info.sql_digest,
            info.original_sql_text,
            info.replaced_sql_text.as_deref().unwrap_or(""),
            info.replaced_sql_digest.as_deref().unwrap_or(""),
            info.error.as_deref().unwrap_or("")
        )?;
    }

    writeln!(output_file, "</table>")?;

    writeln!(output_file, "</body></html>")?;

    info!("HTML file successfully generated");
    Ok(())
}
