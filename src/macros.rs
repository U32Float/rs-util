#[macro_export]
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}

#[macro_export]
macro_rules! implies {
    ($p:expr => $q:expr) => {
        ($p && $q) || !$p
    };
}

// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! with_clone {
    ($i:ident, move ||$l:expr) => {{
        let $i = $i.clone();
        move || {
            $l
        }
    }};
    ($i:ident, move |$($k:pat_param),*|$l:expr) => {{
        let $i = $i.clone();
        move |$( $k ),*| {
            $l
        }
    }};

    (($($i:ident),+), move ||$l:expr) => {{
        let ($($i),+) = ($($i.clone()),+);
        move || {
            $l
        }
    }};
    (($($i:ident),+), move |$($k:pat_param),*|$l:expr) => {{
        let ($($i),+) = ($($i.clone()),+);
        move |$( $k ),*| {
            $l
        }
    }};
}

mod test_with_clone {

    // If this test compiles, it works
    #[test]
    fn test() {
        let x = "String".to_string();
        let y = std::sync::Arc::new(5);

        fn no_arg(f: impl FnOnce()) {
            f()
        }

        no_arg(with_clone!(x, move || {
            drop(x);
        }));

        no_arg(with_clone!((x, y), move || {
            drop(x);
            drop(y);
        }));

        fn one_arg(f: impl FnOnce(usize)) {
            f(1)
        }

        one_arg(with_clone!(x, move |_| {
            drop(x);
        }));
        one_arg(with_clone!((x, y), move |b| {
            drop(x);
            drop(y);
            println!("{}", b);
        }));

        fn two_arg(f: impl FnOnce(usize, bool)) {
            f(5, true)
        }

        two_arg(with_clone!((x, y), move |a, b| {
            drop(x);
            drop(y);
            println!("{}{}", a, b)
        }));
        two_arg(with_clone!((x, y), move |a, _| {
            drop(x);
            drop(y);
            println!("{}", a)
        }));
        two_arg(with_clone!((x, y), move |_, b| {
            drop(x);
            drop(y);
            println!("{}", b)
        }));

        struct Example {
            z: usize,
        }

        fn destructuring_example(f: impl FnOnce(Example)) {
            f(Example { z: 10 })
        }

        destructuring_example(with_clone!(x, move |Example { z }| {
            drop(x);
            println!("{}", z);
        }));

        let a_long_variable_1 = "".to_string();
        let a_long_variable_2 = "".to_string();
        let a_long_variable_3 = "".to_string();
        let a_long_variable_4 = "".to_string();
        two_arg(with_clone!(
            (
                x,
                y,
                a_long_variable_1,
                a_long_variable_2,
                a_long_variable_3,
                a_long_variable_4
            ),
            move |a, b| {
                drop(x);
                drop(y);
                drop(a_long_variable_1);
                drop(a_long_variable_2);
                drop(a_long_variable_3);
                drop(a_long_variable_4);
                println!("{}{}", a, b)
            }
        ));

        fn single_expression_body(f: impl FnOnce(usize) -> usize) -> usize {
            f(20)
        }

        let _result = single_expression_body(with_clone!(y, move |z| *y + z));

        // Explicitly move all variables
        drop(x);
        drop(y);
        drop(a_long_variable_1);
        drop(a_long_variable_2);
        drop(a_long_variable_3);
        drop(a_long_variable_4);
    }
}
