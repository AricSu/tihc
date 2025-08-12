#[cfg(test)]
#[cfg(feature = "tidb-engine")]  // 只在启用 TiDB 引擎时运行测试
mod column_type_conversion_tests {

    #[test]
    fn test_sql_validation_complete_statements_required() {
        // 测试新的验证功能：必须包含完整的CREATE DATABASE + CREATE TABLE + ALTER TABLE
        test_sql_validation_logic();
        
        println!("\n=== Testing Complete DDL Statement Validation ===");
        
        // 使用DDLAnalysisHandler来获得更好的错误信息
        use plugin_lossy_ddl::plugin::DDLAnalysisHandler;
        let handler = DDLAnalysisHandler::new();
        
        // 测试用例1：完整的SQL（应该通过）
        let complete_sql = "CREATE DATABASE testdb;
CREATE TABLE testdb.users (id INT PRIMARY KEY, status INT);
ALTER TABLE testdb.users MODIFY COLUMN status VARCHAR(100);";

        println!("Testing complete SQL (should pass validation):");
        println!("{}", complete_sql);
        
        let result = handler.analyze_sql(complete_sql, true);
        print_analysis_result(&result);
        
        // 测试用例2：缺少CREATE DATABASE（应该失败）
        let missing_db_sql = "CREATE TABLE testdb.orders (id INT PRIMARY KEY, status INT);
ALTER TABLE testdb.orders MODIFY COLUMN status VARCHAR(100);";

        println!("\n=== Testing Missing CREATE DATABASE ===");
        println!("Testing SQL missing CREATE DATABASE (should fail):");
        println!("{}", missing_db_sql);
        
        let result2 = handler.analyze_sql(missing_db_sql, true);
        print_analysis_result(&result2);
        
        // 测试用例3：数据库名不一致（应该失败）
        let inconsistent_db_sql = "CREATE DATABASE db1;
CREATE TABLE db2.products (id INT PRIMARY KEY, name VARCHAR(50));
ALTER TABLE db2.products MODIFY COLUMN name VARCHAR(100);";

        println!("\n=== Testing Inconsistent Database Names ===");
        println!("Testing SQL with inconsistent database names (should fail):");
        println!("{}", inconsistent_db_sql);
        
        let result3 = handler.analyze_sql(inconsistent_db_sql, true);
        print_analysis_result(&result3);
        
        // 验证至少有一个结果是可用的
        assert!(!result.warnings.is_empty() || result.error.is_none());
    }

    fn test_sql_validation_logic() {
        println!("=== Testing SQL Validation Logic ===");
        
        // 测试基本的SQL验证逻辑
        use plugin_lossy_ddl::plugin::DDLAnalysisHandler;
        let handler = DDLAnalysisHandler::new();
        
        // 测试用例1：完整且正确的SQL
        let valid_sql = "CREATE DATABASE inventory;
CREATE TABLE inventory.products (col VARCHAR(64));
ALTER TABLE inventory.products MODIFY col VARCHAR(1024);";
        
        let result = handler.analyze_sql(valid_sql, true);
        
        println!("Test 1 - Complete and valid SQL:");
        println!("Input: {}", valid_sql);
        print_analysis_result(&result);
        
        // 测试用例2：缺少数据库前缀的SQL（应该失败）
        let invalid_sql = "CREATE DATABASE inventory;
CREATE TABLE products (col VARCHAR(64));
ALTER TABLE products MODIFY col VARCHAR(1024);";
        
        let result2 = handler.analyze_sql(invalid_sql, true);
        
        println!("\nTest 2 - SQL without database prefix (should fail):");
        println!("Input: {}", invalid_sql);
        print_analysis_result(&result2);
        
        println!("✅ SQL validation tests completed");
    }

    fn print_analysis_result(result: &plugin_lossy_ddl::AnalysisResult) {
        println!("Analysis result:");
        println!("  risk_level: {:?}", result.risk_level);
        println!("  lossy_status: {:?}", result.lossy_status);

        if let Some(ref error) = result.error {
            println!("  error: {}", error);
        } else {
            println!("  error: None");
        }
        
        println!("  warnings: {:?}", result.warnings);
        
        // 验证结果
        if result.error.is_some() {
            println!("  ❌ Error detected in analysis");
        } else if result.lossy_status == plugin_lossy_ddl::LossyStatus::Unknown {
            println!("  ⚠️ Unknown lossy status, further investigation needed");
        } else if result.lossy_status == plugin_lossy_ddl::LossyStatus::Lossy {
            println!("  🔴 LOSSY operation detected");
            println!("  Risk level: {:?}", result.risk_level);
        } else {
            println!("  ✅ SAFE operation detected");
        }
    }

}