use color_eyre::eyre::Context;

fn main() {
    color_eyre::install().unwrap();

    let mut max = 0;

    for group in include_str!("input.txt")
        // deal with new lines in windows
        .replace("\r\n", "\n")
        .split("\n\n")
    {
        let mut sum = 0;
        for line in group.lines() {
            let value: u64 = line.parse().unwrap();
            sum += value;
        }
        if sum > max {
            max = sum;
        };
    }

    println!("The burdenedst elf is carrying {max} calories");

    // Ok(());
}
