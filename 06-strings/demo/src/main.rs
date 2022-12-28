use std::str::FromStr;

fn main() {
    let s: &str = "Hello, world!";

    // Convert into String
    let s2: String = String::from_str(s).unwrap(); // FromStr trait is -> Result, and can do numbers too!
    let i = i32::from_str(s).expect_err("this should error"); // FromStr trait is -> Result, and can do numbers too!

    let s2: String = s.to_owned();
    // String implements the FromStr trait
    // Rust converts the &str slice into String using `into`
    // `into` is automatically implemented if FromStr is implemented
    // https://doc.rust-lang.org/std/convert/trait.Into.html
    let s2: String = s.into();

    // Convert back to str
    // s3 lives as long as the String s2 exists
    let s3: &str = s2.as_str();

    // Strings, and &str are valid Unicode!
    let bytedance = vec![240, 159, 146, 131];
    let dance = std::str::from_utf8(&bytedance).unwrap();
    println!("{}", dance);

    let bad_bytedance = vec![240, 159, 146];

    let safe_bad_dance = String::from_utf8_lossy(&bad_bytedance);
    println!("{}", safe_bad_dance);

    let bad_dance = std::str::from_utf8(&bad_bytedance);
    assert!(bad_dance.is_err());

    // Strings and utf-8
    // Because Strings are utf-8, you can't easily get index into it
    let s4 = "a🦃bcd";
    dbg!(s4.as_bytes());

    let second_byte_of_turkey = s4.as_bytes()[1];
    println!("Turkey char? {}", second_byte_of_turkey);

    let actual_second_byte_of_turkey = s4.chars().nth(1).unwrap();
    println!("Turkey char! {}", actual_second_byte_of_turkey);

    // What are other programming languages even doing?
    // What does "a🦃bcd"[1] in JavaScript do?

    // The design of Rust APIs gives you wing^H^H^H^H guardrails!
}
