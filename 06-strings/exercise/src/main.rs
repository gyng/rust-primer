struct CakeString {
    desc: String,
}

struct CakeStr<'a> {
    desc: &'a str,
}

// Q: Why is input str and output String?
fn reverse(input: &str) -> String {
    // Reverse isn't trivial in Rust, why? (Mentioned in the demo)
    input.chars().into_iter().rev().collect::<String>()
}

fn main() {
    println!("{}", reverse("abc"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_string_struct() {
        let mut flavour = "strawberry";

        let strawberry = CakeString {
            // You need to do a conversion here, why?
            desc: flavour,
        };

        flavour = "mango";
        let mango = CakeString {
            // You need to do a conversion here again, why?
            desc: flavour,
        };

        assert_ne!(strawberry.desc, mango.desc);

        // Why does this String <> &str comparison work?
        assert_eq!(strawberry.desc, "strawberry");
    }

    #[test]
    fn str_references_in_structs() {
        let mut flavour = "chonky chocolate";
        let chocolate = CakeStr { desc: flavour };

        flavour = "velvet vanilla";
        let vanilla = CakeStr { desc: flavour };

        // 🤔🤔🤔 &str?
        assert_ne!(chocolate.desc, vanilla.desc);
    }

    #[test]
    fn parse_a_number() {
        let my_random_number = "27";

        // Rust can infer the right type of parse,
        // so you only need to do a simple handling  of the Result
        let parsed = my_random_number.parse()

        assert_eq!(27i64, parsed);
    }

    #[test]
    fn concats_two_strings() {
        // a + b
        let a = "a".to_string();
        let b = "b".clone();
        let c: String = "c".into();

        // a + b is same as let ab = a.add(&b);
        let ab = a + b;
        assert_eq!(ab, "ab");
        
        // Two things to fix here, the input to add, and reference to a
        let ac = a + c;
        assert_eq!(ac, "ac");
    }
}
