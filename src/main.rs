fn main() {
    // this is an inline comment.. *snore*
    /*
     * this is a block comment
     *
     * this is a code commented out
     * println!("Hello, world!");
     *
     */
    
    /*
     With RUST, there's no need for the extra *s
     they're purely for show.
     */

    // comments can also be smack down in the middle of code
    /* OLD LESSON
     *
     * let x = 5 + /* 90 + */ 5;
     * println!("Is `x` 10 or 100? x = {}", x);
     *
     */
     println!("{} days", 31);

     // variables generally know what to do
     println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

     // works with named vars also
     println!("{subject} {verb} {object}",
              object="the lazy dog",
              subject="the quick brown fox",
              verb="jumps over");

     // formatting can be specified after a :
     println!("{} out of {:b} people know binary, the other half doesn't", 1, 2);

     // can specify width and alignment of text
     println!("{number:>width$}", number=1, width=6);
    
     // can pad numbers with characters 
     println!("{number:0>width$}", number=1, width=6);

     // create a structure and i32
     #[allow(dead_code)]
     struct Structure(i32);

     // however they require more than minimal handling
     // println!("This struct `{}` won't print...", Structure(3));
     // FIXME ^ Comment out this line
     
     // You can capture arguments from surrounding variables too
     let number: f64 = 1.0;
     let width: usize = 6;
     println!("{number:>width$}");
}
