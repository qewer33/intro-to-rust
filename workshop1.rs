use std::io::{self, Write};

enum Food {
    Cheese,
    Bread,
    Cat,
}

fn parse_food(s: &str) -> Option<Food> {
    match s {
        "cheese" => Some(Food::Cheese),
        "bread"  => Some(Food::Bread),
        "cat"    => Some(Food::Cat),
        _        => None,
    }
}

fn main() -> io::Result<()> {
    let mut hunger: u8 = 3;
    println!("🐭 the rat is hungry. (hunger: {hunger})");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        println!("{}", line);

        let reaction = match parse_food(line.trim()) {
            Some(Food::Cheese) => { hunger -= 1; "🐭 *nibble nibble*" }
            Some(Food::Bread)  => { hunger -= 1; "🐭 *crunch*" }
            Some(Food::Cat)    => "🐭 NO. NEVER.",
            None               => "🐭 ...what is that.",
        };

        if hunger == 0 {
            println!("🐭 *burp* I'm full!");
            break;
        }

        println!("{reaction} (hunger: {hunger})");
    }
    Ok(())
}
