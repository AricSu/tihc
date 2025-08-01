#[macro_export]
macro_rules! parse_args {
    // 用法: parse_args!(args, T1, T2, ...)
    ($args:expr, $($ty:ty),+) => {{
        let mut _i = 0;
        (
            $(
                {
                    let val = serde_json::from_str::<$ty>(&$args[_i]).expect("parse_args! failed");
                    _i += 1;
                    val
                }
            ),*
        )
    }};
}
