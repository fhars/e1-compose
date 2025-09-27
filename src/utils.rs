pub fn print_rule(replacement: &str) {
    let mut chars = replacement.chars();
    let mut ch = chars.next().unwrap();
    if ch == '◌'
        && let Some(c) = chars.next()
    {
        // We copied the ◌ with the combining characters from the tables
        ch = c;
        println!(
            " \"\\x{:x}\" U{:04X} # Combining {}",
            ch as u32, ch as u32, replacement
        );
    } else {
        println!(" \"{}\" U{:04X}", ch, ch as u32,);
    }
}
