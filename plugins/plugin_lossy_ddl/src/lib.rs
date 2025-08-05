// use std::ffi::CString;
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// pub fn precheck_sql(sql: &str, collation_enabled: bool, verbose: bool) -> i32 {
//     let c_sql = CString::new(sql).unwrap();
//     unsafe {
//         PrecheckSQL(
//             c_sql.as_ptr(),
//             if collation_enabled { 1 } else { 0 },
//             if verbose { 1 } else { 0 },
//         )
//     }
// }