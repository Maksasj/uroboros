#[macro_export]
macro_rules! sigma {
    () => {
        GrammaSymbols::Sigma
    };
}

#[macro_export]
macro_rules! sym {
    ($label:literal) => {
        GrammaSymbols::NonTerminal($label.to_string())
    };
    ($terminal:expr) => {
        GrammaSymbols::Terminal($terminal)
    };
}

#[macro_export]
macro_rules! prod {
    ($left:literal => $( ($($right:expr),*) )|* ) => {
        Production { 
            left: sym!($left),
            right: vec![
                $(
                    vec![$( sym!($right), )*],
                )*
            ] 
        }
    };
}