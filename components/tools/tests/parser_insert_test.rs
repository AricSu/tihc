#[cfg(test)]
mod tests {
    use tools::replace_all_column_names;

    #[test]
    fn test_insert_basic() {
        let sql = "INSERT INTO users (id, name) VALUES (1, 'Alice')";
        let ast = replace_all_column_names(sql);
        assert_eq!(
            ast.unwrap(),
            "INSERT INTO users (tihc, tihc) VALUES (1, 'Alice')"
        );
    }

    #[test]
    fn test_insert_multiple_values() {
        let sql = "INSERT INTO users (id, name) VALUES (1, 'Alice'), (2, 'Bob')";
        let ast = replace_all_column_names(sql);
        assert_eq!(
            ast.unwrap(),
            "INSERT INTO users (tihc, tihc) VALUES (1, 'Alice'), (2, 'Bob')"
        );
    }

    #[test]
    fn test_insert_with_columns() {
        let sql = "INSERT INTO users (id, name) VALUES (1, 'Alice')";
        let ast = replace_all_column_names(sql);
        assert_eq!(
            ast.unwrap(),
            "INSERT INTO users (tihc, tihc) VALUES (1, 'Alice')"
        );
    }

    #[test]
    fn test_insert_without_columns() {
        let sql = "INSERT INTO users VALUES (1, 'Alice')";
        let ast = replace_all_column_names(sql);
        assert_eq!(ast.unwrap(), "INSERT INTO users VALUES (1, 'Alice')");
    }

    #[test]
    fn test_insert_with_select() {
        let sql = "INSERT INTO users (id, name) SELECT id, name FROM old_users";
        let ast = replace_all_column_names(sql);
        assert_eq!(
            ast.unwrap(),
            "INSERT INTO users (tihc, tihc) SELECT tihc, tihc FROM old_users"
        );
    }

    #[test]
    fn test_insert_on_duplicate_key_update() {
        let sql =
            "INSERT INTO users (id, name) VALUES (1, 'Alice') ON DUPLICATE KEY UPDATE name='Alice'";
        let ast = replace_all_column_names(sql);
        assert_eq!(ast.unwrap(), "INSERT INTO users (tihc, tihc) VALUES (1, 'Alice') ON DUPLICATE KEY UPDATE tihc = 'Alice'");
    }

    #[test]
    fn test_replace_all_column_names_insert_with_subquery() {
        let sql = "INSERT INTO users (id, name) SELECT id, name FROM old_users WHERE id = 1";
        let result = replace_all_column_names(sql);

        match result {
            Ok(res) => {
                assert_eq!(res, "INSERT INTO users (tihc, tihc) SELECT tihc, tihc FROM old_users WHERE tihc = 1");
            }
            Err(e) => {
                panic!("Test failed due to error: {}", e);
            }
        }
    }

    #[test]
    fn test_insert_with_default_values() {
        let sql = "INSERT INTO users (id, name) VALUES (DEFAULT, 'Alice')";
        let ast = replace_all_column_names(sql);
        assert_eq!(
            ast.unwrap(),
            "INSERT INTO users (tihc, tihc) VALUES (DEFAULT, 'Alice')"
        );
    }

    #[test]
    fn test_insert_with_returning() {
        let sql = "INSERT INTO users (id, name) VALUES (1, 'Alice') RETURNING id, name";
        let ast = replace_all_column_names(sql);
        assert_eq!(
            ast.unwrap(),
            "INSERT INTO users (tihc, tihc) VALUES (1, 'Alice') RETURNING tihc, tihc"
        );
    }
}
