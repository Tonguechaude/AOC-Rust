fn main() {
    println!("Hello, world!");
}

// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Your code here
    let s1_taille = s1.trim().chars().count();
    let s2_taille = s2.trim().chars().count();

    if s1_taille > s2_taille {
        return Some(&s1);
    } else if s1_taille < s2_taille {
        Some(&s2)
    } else {
        None
    }
}
