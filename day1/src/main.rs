use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    println!("{}", part1(&input)?);
    println!("{}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .flat_map(<i32 as std::str::FromStr>::from_str)
        .collect::<Vec<_>>()
        .windows(2)
        .flat_map(|num| {
            num.get(0)
                .and_then(|current| num.get(1).map(|next| (current < next).then(|| 1)))
        })
        .flatten()
        .count())
}

fn part2(input: &str) -> Result<usize> {
    // get sums
    let sums: Vec<i32> = input
        .lines()
        .flat_map(<i32 as std::str::FromStr>::from_str)
        .collect::<Vec<_>>()
        .windows(3)
        .map(|x| x.get(0).unwrap() + x.get(1).unwrap() + x.get(2).unwrap())
        .collect::<Vec<_>>();

    Ok(sums
        .windows(2)
        .flat_map(|num| {
            num.get(0)
                .and_then(|current| num.get(1).map(|next| (current < next).then(|| 1)))
        })
        .flatten()
        .count())
}
