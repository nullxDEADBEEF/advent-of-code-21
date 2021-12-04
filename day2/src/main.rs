use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let mut horizontal = 0;
    let mut depth = 0;

    input.lines().for_each(|x| {
        let mut value = x.split(' ');
        let direction = value.next().unwrap();
        let direction_value = value.next().unwrap().parse::<u32>().unwrap();
        match direction {
            "forward" => horizontal += direction_value,
            "down" => depth += direction_value,
            "up" => depth -= direction_value,
            _ => {}
        }
    });

    Ok(horizontal * depth)
}

fn part2(input: &str) -> Result<u32> {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    input.lines().for_each(|x| {
        let mut value = x.split(' ');
        let direction = value.next().unwrap();
        let direction_value = value.next().unwrap().parse::<u32>().unwrap();
        match direction {
            "forward" => {
                horizontal += direction_value;
                depth += aim * direction_value;
            }
            "down" => {
                aim += direction_value;
            }
            "up" => {
                aim -= direction_value;
            }
            _ => {}
        }
    });

    Ok(horizontal * depth)
}
