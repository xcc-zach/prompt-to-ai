use std::io::{self, Write};

pub fn prompt_to_input(target: &mut String, prompt: &str) -> Result<(), io::Error> {
    println!("{prompt}");
    io::stdout().flush()?;
    io::stdin().read_line(target)?;

    let old = std::mem::take(target);
    target.push_str(old.trim());

    Ok(())
}
