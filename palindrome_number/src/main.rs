fn is_palindrome(x: i32) -> bool {
    let s = x.to_string();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let x = 121;
    if is_palindrome(x) {
        println!("{} is a palindrome", x);
    } else {
        println!("{} is not a palindrome", x);
    }
}
