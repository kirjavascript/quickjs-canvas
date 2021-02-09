#[macro_export]
macro_rules! clone {
    ( $( $x:ident ),* => $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}
