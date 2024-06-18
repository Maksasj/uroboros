#[macro_export]
macro_rules! sigma {
    () => {
        GrammaSymbol::Sigma
    };
}

#[macro_export]
macro_rules! sym {
    ($label:literal) => {
        GrammaSymbol::NonTerminal($label.to_string())
    };
    ($terminal:expr) => {
        GrammaSymbol::Terminal($terminal)
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

#[macro_export]
macro_rules! gram {
    ($( ($left:literal => $( ($($right:expr),*) )|* )),* ) => {
        vec![
            $(
                Production { 
                    left: sym!($left),
                    right: vec![
                        $(
                            vec![$( sym!($right), )*],
                        )*
                    ] 
                }
            ),*
        ]
    };
}