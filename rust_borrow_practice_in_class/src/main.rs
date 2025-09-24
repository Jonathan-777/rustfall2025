fn concat_strings(s1: &String, s2: &String) -> String {
    let mut answer = String::new();
    answer.push_str(s1);
    answer.push_str(s2);
    answer
}

fn clone_and_modify(s: &String) -> String {
    let mut cloned = s.clone();   // deep copy of s
    cloned.push_str("World!");    // safe modification
    cloned


}
fn sum(total: &mut i32, low: i32, high: i32) {
    *total = 0;                       // reset 
    for i in low..=high {
        *total += i;                  // deref and add
    }
}


fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"

// second part
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"

//third part

    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Total: {}", total); // should print 5050

}