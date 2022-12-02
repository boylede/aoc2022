const INPUT: &str = include_str!("../../input/day1.txt");

fn main() {
    println!("day 1");
    let mut elf_calories: Vec<i32> = INPUT
        .split("\n\n")
        .map(|elf_backpack_items| {
            elf_backpack_items
                .split('\n')
                .map(|item| item.parse::<i32>().unwrap_or(0))
                .sum()
        })
        .collect();

    let most_calories = elf_calories.iter().max().unwrap();
    println!("parta: {}", most_calories);

    elf_calories.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let highest_three: i32 = elf_calories.iter().take(3).sum();
    println!("partb: {}", highest_three);
}
