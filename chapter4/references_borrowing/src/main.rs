

//----------------------------------------------------------------------------
// Reference and Borrowing
// The action of creating a reference is called `borrowing`.
// Rust prevent data race and dangling references.
// The Rules of References
// - At any given time, you can have either one mutable references
//   or any number of immutable references.
// - References must always be valid.
//----------------------------------------------------------------------------
//fn main() {
//    let s1 = String::from("hello");
//    // Pass by reference
//    // Unlike a pointer, Reference type is guaranteed to point 
//    // to a valid value of a particular type for the life of that reference.
//    let len = calc_len(&s1);                    
//    println!("The length of {s1} is {len}.");
//}
//
//fn calc_len(s: &String) -> usize { // s is a reference to a String
//    s.len()
//} // Here, s goes out of scope. But because it does not have ownership of what
//  // it referes to, it is not dropped.

//----------------------------------------------------------------------------
// Mutable References
// References are immutable by default as variables are.
//----------------------------------------------------------------------------
//fn main() {
//    // let s = String::from("hello");  // NG: immutable variable can't be passed through mutable reference.
//    let mut s = String::from("hello");
//    // change(&s);  // NG: even if s is declared as mutable, mut must be mentions when it is being passed.
//    change(&mut s);
//    println!("{s}");
//}
//
//fn change(s: &mut String) {
////fn change(s: &String) { // NG: s is immutable
//    s.push_str(", world"); 
//}


//----------------------------------------------------------------------------
// Variable can have only one mutable reference
// This restriction prevents data races at compile time.
//----------------------------------------------------------------------------
//fn main() {
//    let mut s = String::from("hello");
//    let r1 = &mut s;
//    let r2 = &mut s;
//
//    println!("{r1} {r2}");
//}


//----------------------------------------------------------------------------
// Curly brackets to create a new scope, allowing for multiple mutable refernces.
//----------------------------------------------------------------------------
//fn main() {
//    let mut s = String::from("hello");
//
//    {
//        let r1 = &mut s;
//    } // r1 goes out of scope here, so we can make a new reference with no problems.
//}

//----------------------------------------------------------------------------
// Big problem with a mixed mutable and immutable references.
// We can't have a mutable reference while we have an immutable one to the same value.
//----------------------------------------------------------------------------
//fn main() {
//    let mut s = String::from("hello");
//
//    let r1 = &s;    // Ok
//    let r2 = &s;    // OK
//
//    println!("{r1} {r2}"); // OK
//
//    let r3 = &mut s;// BIG PROBLEM
//
//    // Immutable variables (r1 and r2) expect for value not to be changed.
//    println!("{r1} {r2} {r3}");
//}

//----------------------------------------------------------------------------
// Dangling References
//----------------------------------------------------------------------------
fn main() {
    let ref_to_nothing = dangling();
}

fn dangling() -> &String {
    let s = String::from("hello");
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!!
