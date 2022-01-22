use std::error::Error;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("{:?}", part1(&input));
    Ok(())
}

fn part1(s: &str) -> Result<u32> {
    // storing how many 0's per column
    let mut columns = [0; 12];
    let rows = s.lines().count();
    s.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, character)| {
            if character == '0' {
                columns[i] += 1
            }
        })
    });

    let gamma_columns = columns
        .iter()
        .map(|n| if n > &(rows / 2) { '0' } else { '1' })
        .collect::<String>();
    let epsilon = gamma_columns
        .chars()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect::<String>();
    let gamma_rate = u32::from_str_radix(&gamma_columns, 2)?;
    let epsilon_rate = u32::from_str_radix(&epsilon, 2)?;

    Ok(gamma_rate * epsilon_rate)
}
