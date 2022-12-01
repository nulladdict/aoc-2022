fn main() {
    let mut elfs = include_str!("in")
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|food| food.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .collect::<Vec<_>>();

    elfs.sort_by(|a, b| b.cmp(a));

    dbg!(part1(&elfs));
    dbg!(part2(&elfs));
}

fn part1(elfs: &[u64]) -> u64 {
    elfs[0]
}

fn part2(elfs: &[u64]) -> u64 {
    elfs.iter().take(3).sum::<u64>()
}
