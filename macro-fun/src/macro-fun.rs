macro_rules! simple_hi {

    /*
    The below takes no arguments. Allows for simplest match.
    Indicates that no other match versions are possible.
     */

    () => {
        println!("Hello WORD");
    };
}

macro_rules! expanded_hi {

    () => {
        println!("Hello WORD");
    };

    ( $x:expr ) => {
        println!("Multiple arguments? {}", $x);
    };

    ( $( $x:expr ), *) => {
        {
            $(
                println!("And another: {}", $x);
            )*
        }
    };

}



/*

errors:

error: expected one of: `*`, `+`, or `?` -->
    pattern matching means parentheses expression repeats? Try without

error: variable 'x' is still repeating at this depth ->
    If $x is a repeating expression, that needs handling via range

*/


//https://doc.rust-lang.org/book/ch19-06-macros.html
//Simplified version of the vec! macro definition

//#[macro_export] indicates the macro is available when this crate is in scope
#[macro_export]
macro_rules! vec {

    ( $( $x:expr ), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
                temp_vec
        }
    };

}


fn main() {
    simple_hi!();
    expanded_hi!();
    expanded_hi!("meaningless");
    expanded_hi!("meaningless", "further", "again");
}
