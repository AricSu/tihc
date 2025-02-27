use tools::replace_all_column_names;

#[test]
fn test_replace_all_column_names_update_simple() {
    let sql = "UPDATE users SET name = 'John' WHERE id = 1";
    let result = replace_all_column_names(sql);

    match result {
        Ok(res) => {
            assert_eq!(res, "UPDATE users SET tihc = 'John' WHERE tihc = 1");
        }
        Err(e) => {
            panic!("Test failed due to error: {}", e);
        }
    }
}
