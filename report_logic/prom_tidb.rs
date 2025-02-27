use crate::ResultData;
use super::query_prometheus_range;
use std::error::Error;
use chrono::Duration;
use chrono::Utc;

pub async fn get_tidb_duration() -> Result<Vec<ResultData>, Box<dyn Error>> {
    let query = r#"sum(rate(tidb_server_handle_query_duration_seconds_bucket[1m])) by (le)"#;
    // Define the time range (e.g., the last 60 minutes)
    let end_time = Utc::now();
    let start_time = end_time - Duration::minutes(60);

    // Define the step interval for Prometheus data points
    let step = "15s"; // 15 seconds interval

    // Fetch the results from Prometheus with the time range
    let result = query_prometheus_range(query, start_time, end_time, step).await;

    // Use proper error handling instead of assert
    match result {
        Ok(prometheus_response) => Ok(prometheus_response.data.result),
        Err(e) => Err(e), // No need to box the error again
    }
}

pub fn agg_prom_point(data: Vec<ResultData>) -> Vec<(f64, f64)>{

    let mut aggregated_values: Vec<(f64, f64)> = Vec::new();

    for result in data {
        // Aggregate all values from the result into the aggregated_values vector
        for (timestamp, value) in result.values {
            aggregated_values.push((timestamp, value.parse().expect("Failed to parse f64")));
        }
    }
    
    aggregated_values
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_get_tidb_duration() {
        let result = get_tidb_duration().await;
        assert!(result.is_ok(), "Failed to fetch TiDB duration data");
        // if let Ok(data) = result {
        //     // Additional checks for data
        //     assert!(!data.is_empty(), "No data received");
        //     for result in data {
        //         println!("Metric: {:?}", result.metric);
        //         for (timestamp, value) in result.values {
        //             println!("Point: {}, Value: {}", crate::convert_unix_to_readable(timestamp), value);
        //         }
        //     }
        // }
        let x = agg_prom_point(result.unwrap());
        for i in x{
            println!("{:?}", i);
        }
    }
}
