use std::io::Read;

fn part_1(input: &str) -> u64 {
    let mut groups = input.split("\n\n");
    let mut items: Vec<u64> =
        groups.next().unwrap().split_ascii_whitespace().skip(1).map(|num| num.parse().unwrap()).collect();

    for group in groups {
        let mappings: Vec<(u64, u64, u64)> = group.lines().skip(1).map(|line| {
            let mut nums = line.split_ascii_whitespace().map(|num| num.parse().unwrap());
            let dst = nums.next().unwrap();
            let src = nums.next().unwrap();
            let len = nums.next().unwrap();
            (dst, src, src + len)
        }).collect();

        for i in &mut items {
            for &(dst, src, src_end) in &mappings {
                if *i >= src && *i < src_end {
                    *i = *i - src + dst;
                    break;
                }
            }
        }
    }

    items.into_iter().min().unwrap()
}

fn part_2(input: &str) -> u64 {
    let mut groups = input.split("\n\n");
    let mut nums = groups.next().unwrap().split_ascii_whitespace().skip(1).map(|num| num.parse::<u64>().unwrap());
    let mut items = Vec::new();
    while let Some(num) = nums.next() {
        items.push((num, num + nums.next().unwrap()))
    }

    let mut new_items = Vec::new();
    let mut unmapped_items = Vec::new();

    for group in groups {
        new_items.clear();

        for line in group.lines().skip(1) {
            unmapped_items.clear();

            let mut nums = line.split_ascii_whitespace().map(|num| num.parse::<u64>().unwrap());
            let dst = nums.next().unwrap();
            let src = nums.next().unwrap();
            let len = nums.next().unwrap();
            let src_end = src + len;

            for &(item, item_end) in &items {
                unmapped_items.extend([
                    (item, std::cmp::min(src, item_end)),
                    (std::cmp::max(src_end, item), item_end),
                ].into_iter().filter(|&(start, end)| end > start));

                let start = dst + (std::cmp::min(std::cmp::max(src, item), src_end) - src);
                let end = dst + (std::cmp::max(std::cmp::min(src_end, item_end), src) - src);
                if end > start {
                    new_items.push((start, end));
                }
            }

            std::mem::swap(&mut items, &mut unmapped_items);
        }

        if items.len() > new_items.len() {
            items.extend(new_items.iter().copied());
        } else {
            new_items.extend(items.iter().copied());
            std::mem::swap(&mut items, &mut new_items);
        }

        // Merge adjacent ranges, to save time
        items.sort();
        new_items.clear();
        for &i in &items {
            let (start, end) = i;
            let mut added = false;
            if let Some(last) = new_items.last_mut() {
                if last.1 == start {
                    last.1 = end;
                    added = true;
                }
            }
            if !added {
                new_items.push(i);
            }
        }
        std::mem::swap(&mut items, &mut new_items);
    }

    items.into_iter().min().unwrap().0
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}
