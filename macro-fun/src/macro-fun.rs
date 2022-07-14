macro_rules! simple_hi {

    /*
    The below takes no arguments. Allows for simplest match.
    Indicates that no other match versions are possible.
     */

    () => {
        println!("Hello WORD");
    };
}


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
    println!("Hello, world!");
}
