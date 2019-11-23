#[macro_export]
macro_rules! hashmap {
    ($($k:expr => $v:expr,)+) => { hashmap!($($k => $v),+) };
    ($($k:expr => $v:expr), *) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(map.insert($k, $v);)*
            map
        }
    };
}
