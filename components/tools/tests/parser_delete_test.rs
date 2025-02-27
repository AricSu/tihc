use tools::replace_all_column_names;

#[test]
fn test_replace_all_column_names_delete_with_nested_query() {
    let sql = "DELETE FROM users WHERE id IN (SELECT user_id FROM orders WHERE order_date < '2021-01-01')";
    let result = replace_all_column_names(sql);

    match result {
        Ok(res) => {
            assert_eq!(
                res,
                "DELETE FROM users WHERE tihc IN (SELECT tihc FROM orders WHERE tihc < '2021-01-01')"
            );
        }
        Err(e) => {
            panic!("Test failed due to error: {}", e);
        }
    }
}
