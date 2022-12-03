pub const INPUTS: &[&str] = &[INPUT];
pub const INPUT: &str = include_str!("../../input/day1.txt");

#[inline]
pub fn run(input: &str) -> (String, String) {
    let mut elf_calories: Vec<i32> = input
        .split("\n\n")
        .map(|elf_backpack_items| {
            elf_backpack_items
                .split('\n')
                .map(|item| item.parse::<i32>().unwrap_or(0))
                .sum()
        })
        .collect();

    let most_calories: i32 = elf_calories.iter().max().unwrap().to_owned();

    elf_calories.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let highest_three: i32 = elf_calories.iter().take(3).sum();
    (format!("{}", most_calories), format!("{}", highest_three))
}
