use std::error::Error;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const ROWS: u32 = 1000;
const COLUMNS: usize = 12;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
    Ok(())
}

fn part1(s: &str) -> Result<u32> {
    // storing how many 0's per column
    let mut columns = [0; 12];
    s.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, character)| {
            if character == '0' {
                columns[i] += 1
            }
        })
    });

    let gamma_columns = columns
        .iter()
        .map(|n| if n > &(ROWS / 2) { '0' } else { '1' })
        .collect::<String>();
    let epsilon = gamma_columns
        .chars()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect::<String>();
    let gamma_rate = u32::from_str_radix(&gamma_columns, 2)?;
    let epsilon_rate = u32::from_str_radix(&epsilon, 2)?;

    Ok(gamma_rate * epsilon_rate)
}

fn get_most_bit_occurence(data: &[&str], col: usize) -> char {
    let mut zeroes = 0;
    let mut ones = 0;
    data.iter().for_each(|line| {
        if line.chars().nth(col).unwrap() == '0' {
            zeroes += 1
        } else {
            ones += 1
        }
    });

    if zeroes > ones {
        '0'
    } else {
        '1'
    }
}

fn find_rating_by_bit_criteria(data: &mut Vec<&str>, find_most_common: bool) {
    (0..COLUMNS).for_each(|column| {
        if data.len() > 1 {
            let common_bit = get_most_bit_occurence(data, column);
            if find_most_common {
                data.retain(|x| x.chars().nth(column).unwrap() == common_bit);
            } else {
                data.retain(|x| x.chars().nth(column).unwrap() != common_bit);
            }
        }
    });
}

fn part2(s: &str) -> Result<u32> {
    let mut oxygen_rating_data = s.lines().collect::<Vec<&str>>();
    let mut scrubber_data = s.lines().collect::<Vec<&str>>();

    find_rating_by_bit_criteria(&mut oxygen_rating_data, true);
    find_rating_by_bit_criteria(&mut scrubber_data, false);

    let oxygen_rating = u32::from_str_radix(oxygen_rating_data[0], 2)?;
    let co2_scrubber = u32::from_str_radix(scrubber_data[0], 2)?;

    Ok(oxygen_rating * co2_scrubber)
}
