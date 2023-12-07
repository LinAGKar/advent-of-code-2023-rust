use std::io::Read;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn winnings(input: &str, jokers: bool) -> u32 {
    // Parse input
    let mut hands: Vec<_> = input.lines().map(|line| {
        let mut words = line.split_ascii_whitespace();
        (words.next().unwrap(), words.next().unwrap().parse::<u16>().unwrap())
    }).collect();

    // Sort by hand type and hand card values
    hands.sort_by_cached_key(|&(hand_src, _)| {
        // Translate card names to values
        let mut hand = [0u8; 5];
        for (src, dst) in hand_src.chars().zip(&mut hand) {
            *dst = match src {
                '2'..='9' => (src.to_digit(10).unwrap() - 1) as u8,
                'T' => 9,
                'J' => if jokers { 0 } else { 10 },
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                _ => panic!(),
            };
        }

        // Count how many occurrences there are of each card
        let mut counts = [0u8; 14];
        for &i in &hand {
            counts[i as usize] += 1;
        }

        // Change type of jokers to match whatever we have most of
        if jokers {
            let max_index = counts.iter().enumerate().skip(1).max_by_key(|(_, &x)| x).unwrap().0;
            counts[max_index] += counts[0];
            counts[0] = 0;
        }

        // Calculate type of hand
        let hand_type = if counts.contains(&5) {
            HandType::FiveOfAKind
        } else if counts.contains(&4) {
            HandType::FourOfAKind
        } else if counts.contains(&3) {
            if counts.contains(&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else if counts.contains(&2) {
            if counts.iter().filter(|&&x| x == 2).count() == 2 {
                HandType::TwoPairs
            } else {
                HandType::OnePair
            }
        } else {
            HandType::HighCard
        };

        (hand_type, hand)
    });

    // Sum up winnings
    hands.iter().enumerate().map(|(n, &(_, bid))| {
        (n as u32 + 1) * bid as u32
    }).sum()
}

fn part_1(input: &str) -> u32 {
    winnings(input, false)
}

fn part_2(input: &str) -> u32 {
    winnings(input, true)
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
