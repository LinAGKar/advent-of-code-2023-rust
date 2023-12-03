use std::io::Read;

fn part_1(input: &str) -> u32 {
    let width = input.find('\n').unwrap();
    let check = |pos: usize| !["\n", "."].contains(&&input[pos..pos + 1]);

    let mut pos = 0;
    let mut sum = 0;
    while let Some(start) = input[pos..].find(|c: char| c.is_ascii_digit()) {
        let start = start + pos;
        let end = input[start..].find(|c: char| !c.is_ascii_digit()).unwrap() + start;

        if (start > 0 && check(start - 1)) ||
           check(end) ||
           (start > width && (start - width - 1..end - width).any(|pos| check(pos))) ||
           (start > width + 1 && check(start - width - 2)) ||
           (start + width < input.len() && (start + width..end + width + 2).any(|pos| check(pos))) {
            sum += input[start..end].parse::<u32>().unwrap();
        }

        pos = end;
    }

    sum
}

fn part_2(input: &str) -> u32 {
    let width = input.find('\n').unwrap();
    let mut counts = vec![(0, 1); input.len()];
    let mut gears = Vec::new();
    let mut check = |pos, num: u32| {
        if &input[pos..pos + 1] == "*" {
            let (count, product) = &mut counts[pos];
            if *count == 0 {
                gears.push(pos);
            }
            *count += 1;
            *product *= num;
        }
    };

    let mut pos = 0;
    while let Some(start) = input[pos..].find(|c: char| c.is_ascii_digit()) {
        let start = start + pos;
        let end = input[start..].find(|c: char| !c.is_ascii_digit()).unwrap() + start;
        let num = input[start..end].parse().unwrap();

        if start > 0 { check(start - 1, num); }
        check(end, num);
        if start > width {
            for pos in start - width - 1..end - width {
                check(pos, num);
            }
        }
        if start > width + 1 { check(start - width - 2, num); }
        if start + width < input.len() {
            for pos in start + width..end + width + 2 {
                check(pos, num);
            }
        }

        pos = end;
    }

    gears.into_iter().filter_map(|pos| {
        let (count, product) = counts[pos];
        if count == 2 {
            Some(product)
        } else {
            None
        }
    }).sum()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
