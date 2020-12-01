fn find_entries_and_multiply(numbers: &[isize]) -> Option<isize> {
    let size = numbers.len();
    for i in 0..size {
        let x_i = numbers[i];
        for j in i..size {
            let x_j = numbers[j];
            if x_i + x_j == 2020 { return Some(x_i * x_j) }
        }
    }
    None
}

pub fn part_a(input: &str) -> isize {
    let numbers: Vec<isize> = input.lines()
        .filter_map(|x| x.parse::<isize>().ok())
        .collect();
    find_entries_and_multiply(&numbers).unwrap()
}

#[test]
fn example_one() {
    let numbers = [
        1721,
        979,
        366,
        299,
        675,
        1456,
    ];
    assert_eq!(find_entries_and_multiply(&numbers), Some(514579));
}