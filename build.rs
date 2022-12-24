use std::process::Command;

fn main() {
    Command::new("tailwindcss")
        .args("-c ./tailwind.config.js -o ./static/styles.css".split_whitespace())
        .status()
        .unwrap();
}
