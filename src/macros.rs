#[macro_export]
macro_rules! lmatch {
    ($($p:pat $(if $guard:expr)? => $e:expr),+ $(,)?) => {
        |x| match x {
            $($p $(if $guard)? => $e),+
        }
    };
}
