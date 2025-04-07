use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::error::Error;

#[tokio::test]
async fn test_mysql_connection() -> Result<(), Box<dyn Error>> {
    // 1. 配置数据库连接
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:@127.0.0.1:4000/tihc")
        .await?;

    // 2. 执行简单查询验证连接
    let result: (i64,) = sqlx::query_as("SELECT 1").fetch_one(&pool).await?;

    assert_eq!(result.0, 1, "数据库连接测试失败");
    println!("数据库连接测试成功");

    // 3. 查询数据库版本
    let version: (String,) = sqlx::query_as("SELECT VERSION()").fetch_one(&pool).await?;

    println!("MySQL 版本: {}", version.0);

    Ok(())
}
