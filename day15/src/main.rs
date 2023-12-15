fn hash(string: &str) -> u8 {
    string.bytes().fold(0, |acc, x| {
        acc.wrapping_add(x).wrapping_mul(17)
    })
}

fn part_1(input: &str) -> u32 {
    input.trim_end().split(',').map(|step| hash(step) as u32).sum()
}

fn part_2(input: &str) -> u32 {
    let mut boxes = vec![Vec::new(); 0x100];

    for step in input.trim_end().split(',') {
        if step.ends_with('-') {
            let label = &step[..step.len() - 1];
            let r#box = &mut boxes[hash(label) as usize];
            if let Some(index) = r#box.iter().position(|&(slot_label, _)| slot_label == label) {
                r#box.remove(index);
            }
        } else {
            let mut parts = step.split('=');
            let label = parts.next().unwrap();
            let focal_length = parts.next().unwrap();
            let r#box = &mut boxes[hash(label) as usize];
            if let Some((_, slot_focal_length)) = r#box.iter_mut().find(|&&mut (slot_label, _)| slot_label == label) {
                *slot_focal_length = focal_length;
            } else {
                r#box.push((label, focal_length));
            }
        }
    }

    boxes.into_iter().zip(1..).flat_map(|(r#box, n)| {
        r#box.into_iter().zip(1..).map(move |((_, focal_length), m)| n * m * focal_length.parse::<u32>().unwrap())
    }).sum()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}
