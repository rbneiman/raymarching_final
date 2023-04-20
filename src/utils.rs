

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[macro_export]
macro_rules! log_warn {
    ( $( $t:tt )* ) => {
        web_sys::console::warn_1(&format!( $( $t )* ).into());
    }
}

#[macro_export]
macro_rules! log_error {
    ( $( $t:tt )* ) => {
        web_sys::console::error_1(&format!( $( $t )* ).into());
    }
}
