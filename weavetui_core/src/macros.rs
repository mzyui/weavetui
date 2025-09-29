//! Convenience macros for the `weavetui` framework.

#[macro_export]
macro_rules! kb {
    ($($key:expr => $action:expr),* $(,)?) => {
        [
            $(($key, $crate::event::ActionKind::from($action))),*
        ]
    };

    ($(($key:expr, $action:expr)),* $(,)?) => {
        [
            $(($key, $crate::event::ActionKind::from($action))),*
        ]
    };
}

#[macro_export]
macro_rules! components {
    ( $( $x:expr $( => $t:ty )* ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(
                    Box::new($x)
                        as Box<dyn $crate::Component $( $t + )* >
                );
            )*
            temp_vec
        }
    };
}
