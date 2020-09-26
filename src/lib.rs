// Declarative Macros are way to do code replacement, by using syntax pattern matching
// Macros are used to do code substition i.e declarative, while with proc its more procedural i.e programs.
// With Proc Macros, you have more control on the input syntax, and hence why macro_rules! is a proc macro.
// Identifiers in Macro world are different from the normal code, and compilation fails, instead pass those values as arguements to macros
#[macro_export]
macro_rules! avec {
    () => {
        // Here directly expands to a exp that can be assigned, so need of blocks usage.
        Vec::new()
    };

    // [3,3,4....]
    // expr covers most stuff in Rust, anything you can terminate with ;
    // Macros can't be expanded to statements directly, and hence you need to expand to a block
    // Here we are writing a regular expression to match multiple args.
    // Here , is a token used to separate args but it could be anything as its a macro
    // Allow, Trailing comma by adding extra $(,)? as the follow up
    ($($args: expr),+ $(,)?) => {{
        let mut vs = Vec::new();
        // Repeat the inside code, for each element in args
        $(
            vs.push($args);
        )*
        // Return this as it will be directly returned when the macro is called, and assigned.
        vs
    }};

    // [x ; count]
    ($arg1: expr; $arg2: expr) => {{
        let mut vs = Vec::new();
        // Repeat the inside code, for each element in args
        // we can't directly loop and call $arg1 as it can be an exp which evalualte only once
        // Hence, evaluate it once and use clone on each push.
        let ans = $arg1;
        for _ in 0..$arg2 {
            vs.push(ans.clone());
        }
        vs
    }};
}

#[test]
fn empty_vec() {
    // delimiters do not matter, and cannot be imposed on the caller.
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec!(43);
    assert!(!x.is_empty());
    assert_eq!(x[0], 43);
}

#[test]
fn double() {
    let x: Vec<u32> = avec!(43, 32);
    assert!(!x.is_empty());
    assert_eq!(x[0], 43);
}

#[test]
fn trailing() {
    let _ = avec!("hisdhisdisds", "asdasdasdas",);
}

#[test]
fn clonee() {
    let x = avec!(42; 5);
    assert_eq!(x.len(), 5);
}

#[test]
fn clone_nonliteral() {
    let mut y = Some(42);
    let x = avec!(y.take().unwrap(); 3);
    assert_eq!(x.len(), 3);
}
