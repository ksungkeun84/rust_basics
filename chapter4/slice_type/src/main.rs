
//---------------------------------------------------------------
// Slice Type 
//---------------------------------------------------------------
//fn main() {
//    let mut s = String::from("hi there");
//    let idx = first_word(&s);
//    s.clear();
//    let word = &s[..idx];    // index out of bounds
//    println!("{word}");
//
//}
//
//fn first_word(s: &String) -> usize {
//    let bytes = s.as_bytes();
//
//    for (i, &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//            return i;
//        }
//    }
//    s.len()
//}

fn main() {
    //let mut s = String::from("hi there");
    let mut s = "hi there";
    let word = first_word(&s);
    println!("{word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..]
}
