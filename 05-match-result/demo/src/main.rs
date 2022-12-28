fn main() {
    struct Point {
        x: f64,
        y: f64,
    };

    // Tagged unions: enums can hold values
    // Generics are supported too
    enum DrawCommand {
        Begin,
        // Tuple in enum
        Point((f64, f64)),
        // Struct in enum
        LineTo { x: f64, y: f64 },
        LineBetween((Point, Point)),
        Clear,
        End,
    }

    impl DrawCommand {
        // Option type: A wrapper over a value
        // We don't have nulls
        // This abstraction forces us to check every time we want to use a value that might not exist
        fn from_u8(n: u8, data: Option<(f64, f64)>) -> Option<DrawCommand> {
            // Simple match: case statement equiv
            match n {
                0x1 => Some(DrawCommand::Begin),
                0x2 => {
                    if let Some((x, y)) = data {
                        return Some(DrawCommand::Point((x, y)));
                    }
                    None
                }
                0x3 => Some(DrawCommand::End),
                _ => None,
            }
        }
    }

    fn process_command(command: DrawCommand) -> () {
        match command {
            DrawCommand::End => println!("End!"),
            DrawCommand::Begin | DrawCommand::Clear => println!("Begin!"),
            // Conditonal match: match guard
            DrawCommand::Point((x, _)) if x < 0.0 => {
                println!("Invalid command: cannot draw left of 0!")
            }
            // Destructure in match arm
            DrawCommand::Point((x, y)) => println!("Drawing at {} {}!", x, y),
            // Exhaustive: everything else
            _ => println!("Unhandled command!"),
            // There's more, but these are the basics
        }
    }

    let invalid_commands: Vec<Option<DrawCommand>> =
        vec![(0x1, None), (0x2, Some((1.0, 2.0))), (0xf, None)]
            .into_iter()
            .map(|(id, data)| DrawCommand::from_u8(id, data))
            .collect();

    fn process_commands(commands: Vec<Option<DrawCommand>>) -> () {
        for command in commands {
            match command {
                Some(c) => process_command(c),
                None => println!("Invalid command"),
            }
        }
    }

    println!("Invalid commands");
    process_commands(invalid_commands);

    // Result (Ok, Err) is similar to Option, but used semantically to report operations
    // Eg, opening a file or parsing a value that should fail

    // Using values in Option/Result
    let wrapper: Option<i32> = Some(1);
    dbg!(wrapper);

    let value: i32 = match wrapper {
        Some(val) => val,
        None => panic!(),
    };
    dbg!(value);

    // But match is inconvenient

    // Panics! avoid unwrap in your application code
    let value = wrapper.unwrap();
    dbg!(value);

    // Panics! avoid in your application code
    let value = wrapper.expect("wrapper was none");
    dbg!(value);

    // Sugar to deal with Option types
    if wrapper.is_some() {
        dbg!(wrapper.unwrap());
    }

    if let x = Some(wrapper) {
        dbg!(x);
    }

    // Question mark sugar return an Error if it fails to unwrap
    // let value = wrapper?;
}
