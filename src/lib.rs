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

    // expr covers most stuff in Rust, anything you can terminate with ;
    // Macros can't be expanded to statements directly, and hence you need to expand to a block
    ($arg1: expr) => {{
        let mut vs = Vec::new();
        vs.push($arg1);
        // Return this as it will be directly returned when the macro is called, and assigned.
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
