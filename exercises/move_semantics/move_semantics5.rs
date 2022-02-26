// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)


fn main() {
    let mut x = 100; // x -> 100;
    let y = &mut x; // y -pointer-> 100 <-pointer- x
    *y += 100;  // y -pointer-> 200 <-pointer- x
    let z = &mut x;// z -pointer-> 200 <-pointer- x
    *z += 1000;// z -pointer-> 1200 <-pointer- x
    assert_eq!(x, 1200);
}
