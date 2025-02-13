use tools::replace_all_column_names;

#[test]
fn test_replace_all_column_names_select_with_inner_join() {
    let sql =
        "SELECT a.id, b.name FROM users a INNER JOIN orders b ON a.id = b.user_id WHERE a.id = 1";
    let result = replace_all_column_names(sql);

    match result {
        Ok(res) => {
            assert_eq!(res, "SELECT tihc.tihc, tihc.tihc FROM users AS a JOIN orders AS b ON tihc.tihc = tihc.tihc WHERE tihc.tihc = 1");
        }
        Err(e) => {
            panic!("Test failed due to error: {}", e);
        }
    }
}

#[test]
fn test_replace_all_column_names_select_with_left_join() {
    let sql =
        "SELECT a.id, b.name FROM users a LEFT JOIN orders b ON a.id = b.user_id WHERE a.id = 1";
    let result = replace_all_column_names(sql);

    match result {
        Ok(res) => {
            assert_eq!(
                res,
                "SELECT tihc.tihc, tihc.tihc FROM users AS a LEFT JOIN orders AS b ON tihc.tihc = tihc.tihc WHERE tihc.tihc = 1"
            );
        }
        Err(e) => {
            panic!("Test failed due to error: {}", e);
        }
    }
}
#[test]
fn test_replace_all_column_names_select_with_right_join() {
    let sql =
        "SELECT a.id, b.name FROM users a RIGHT JOIN orders b ON a.id = b.user_id WHERE a.id = 1";
    let result = replace_all_column_names(sql);

    match result {
        Ok(res) => {
            assert_eq!(
                res,
                "SELECT tihc.tihc, tihc.tihc FROM users AS a RIGHT JOIN orders AS b ON tihc.tihc = tihc.tihc WHERE tihc.tihc = 1"
            );
        }
        Err(e) => {
            panic!("Test failed due to error: {}", e);
        }
    }
}

#[test]
fn test_replace_all_column_names_select_with_full_outer_join() {
    let sql =
        "SELECT a.id, b.name FROM users a FULL OUTER JOIN orders b ON a.id = b.user_id WHERE a.id = 1";
    let result = replace_all_column_names(sql);

    match result {
        Ok(res) => {
            assert_eq!(
                res,
                "SELECT tihc.tihc, tihc.tihc FROM users AS a FULL JOIN orders AS b ON tihc.tihc = tihc.tihc WHERE tihc.tihc = 1"
            );
        }
        Err(e) => {
            panic!("Test failed due to error: {}", e);
        }
    }
}

#[test]
fn test_replace_all_column_names_select_with_cross_join() {
    let sql = "SELECT a.id, b.name FROM users a CROSS JOIN orders b WHERE a.id = 1";
    let result = replace_all_column_names(sql);

    match result {
        Ok(res) => {
            assert_eq!(
                res,
                "SELECT tihc.tihc, tihc.tihc FROM users AS a CROSS JOIN orders AS b WHERE tihc.tihc = 1"
            );
        }
        Err(e) => {
            panic!("Test failed due to error: {}", e);
        }
    }
}

#[test]
fn test_replace_all_column_names_select_with_self_join() {
    let sql = "SELECT a.id, b.name FROM users a, users b WHERE a.id = b.id AND a.id = 1";
    let result = replace_all_column_names(sql);

    match result {
        Ok(res) => {
            assert_eq!(
                res,
                "SELECT tihc.tihc, tihc.tihc FROM users AS a, users AS b WHERE tihc.tihc = tihc.tihc AND tihc.tihc = 1"
            );
        }
        Err(e) => {
            panic!("Test failed due to error: {}", e);
        }
    }
}

#[test]
fn test_replace_all_column_names_select_with_subquery() {
    let sql = "SELECT a.id, (SELECT name FROM orders WHERE user_id = a.id) FROM users a WHERE a.id = 1";
    let result = replace_all_column_names(sql);

    match result {
        Ok(res) => {
            assert_eq!(
                res,
                "SELECT tihc.tihc, (SELECT tihc FROM orders WHERE tihc = tihc.tihc) FROM users AS a WHERE tihc.tihc = 1"
            );
        }
        Err(e) => {
            panic!("Test failed due to error: {}", e);
        }
    }
}

#[test]
fn test_replace_all_column_names_select_with_nested_query() {
    let sql = "SELECT a.id, b.name FROM (SELECT * FROM users) a JOIN (SELECT * FROM orders) b ON a.id = b.user_id WHERE a.id = 1";
    let result = replace_all_column_names(sql);

    match result {
        Ok(res) => {
            assert_eq!(
                res,
                "SELECT tihc.tihc, tihc.tihc FROM (SELECT * FROM users) AS a JOIN (SELECT * FROM orders) AS b ON tihc.tihc = tihc.tihc WHERE tihc.tihc = 1"
            );
        }
        Err(e) => {
            panic!("Test failed due to error: {}", e);
        }
    }
}

use tracing_subscriber;

#[test]
fn test_replace_all_column_names_select_with_group_by() {
    // 初始化日志记录器
    tracing_subscriber::fmt::init();

    let sql = "select `job_meta` , `processing` from `mysql` . `tidb_ddl_job` where `job_id` in ( select min ( `job_id` ) from `mysql` . `tidb_ddl_job` group by `schema_ids` , `table_ids` , `processing` ) and not `reorg` order by `processing` desc , `job_id`";
    let result = replace_all_column_names(sql);

    match result {
        Ok(res) => {
            assert_eq!(
                res,
                "SELECT `tihc`, `tihc` FROM `mysql`.`tidb_ddl_job` WHERE `tihc` IN (SELECT min(`tihc`) FROM `mysql`.`tidb_ddl_job` GROUP BY `tihc`, `tihc`, `tihc`) AND NOT `tihc` ORDER BY `tihc` DESC, `tihc`"
            );
        }
        Err(e) => {
            panic!("Test failed due to error: {}", e);
        }
    }
}