use std::{io::Read, str::FromStr, fmt::Debug, ops::{Mul, Sub, Add, Div}};

const MIN_COORD: f64 = 200000000000000.0;
const MAX_COORD: f64 = 400000000000000.0;

fn parse_hailstone<T: Default + Copy + FromStr>(line: &str) -> [[T; 3]; 2] where <T as FromStr>::Err: Debug {
    let mut coord = [[T::default(); 3]; 2];
    for (src, dst) in line.split(" @ ").zip(&mut coord) {
        for (src, dst) in src.split(", ").zip(dst) {
            *dst = src.trim().parse().unwrap();
        }
    }
    coord
}

fn parse_hailstones<T: Default + Copy + FromStr<Err = impl Debug>>(input: &str) -> std::iter::Map<std::str::Lines<'_>, for<'a> fn(&'a str) -> [[T; 3]; 2]> {
    input.lines().map(parse_hailstone)
}

// Math for calculating intersection coordinates in two dimensions, in StarMath syntax (e.g. Libreoffice Math)
//
// In the below, x and y are the intersection coordinates, p_[ab][xy] are the initial positions, and v_[ab][xy] are the
// velocities.
//
// We can set up a an equation system for the y value of each stone depends on the x coordinate, and the initial values,
// under the assumption no stone has velocity 0 on the x axis:
// left lbrace stack { y = p_ay + ( x - p_ax ) v_ay over v_ax # y = p_by + ( x - p_bx ) v_by over v_bx } right none
//
// Merging those equations over y gives:
// p_ay + ( x - p_ax ) v_ay over v_ax = p_by + ( x - p_bx ) v_by over v_bx dlrarrow
// p_ay + x v_ay over v_ax - p_ax v_ay over v_ax = p_by + x v_by over v_bx - p_bx v_by over v_bx dlrarrow
// x v_ay over v_ax - x v_by over v_bx = p_by - p_ay - p_bx v_by over v_bx + p_ax v_ay over v_ax dlrarrow
// x left ( v_ay over v_ax - v_by over v_bx right ) = p_by - p_ay - p_bx v_by over v_bx + p_ax v_ay over v_ax dlrarrow
// x = { p_by - p_ay - { p_bx v_by } over v_bx + { p_ax v_ay } over v_ax } over { v_ay over v_ax - v_by over v_bx } dlrarrow
// x = { { p_by v_bx v_ax - p_ay v_bx v_ax - p_bx v_by v_ax + p_ax v_ay v_bx } over { v_bx v_ax } } over { { v_ay v_bx - v_by v_ax } over { v_ax v_bx } } dlrarrow
// x = { (p_by v_bx v_ax - p_ay v_bx v_ax - p_bx v_by v_ax + p_ax v_ay v_bx) (v_ax v_bx) } over { (v_bx v_ax) (v_ay v_bx - v_by v_ax) } dlrarrow
// x = { (p_by - p_ay) v_bx v_ax - p_bx v_by v_ax + p_ax v_ay v_bx } over { v_ay v_bx - v_by v_ax }
//
// If the denominator here is 0, that means the hailstones are moving in the same or opposite direction, i.e. the lines
// are parallel. In that case, there are either no solutions, or an infinite number of solutions (if the hailstones are
// moving on the same line, but I'm making the assumption that that doesn't occur in the input).
//
// The initial equations wouldn't work if either hailstone has velocity 0 on the x-axis, but the resulting formula looks
// like it does (it just returns that hailstone's initial position on the x-axis).

fn intersect_2d<T: Mul<Output = T> + Div<Output = T> + Sub<Output = T> + Add<Output = T> + Default + PartialEq + Copy>(a: [[T; 2]; 2], b: [[T; 2]; 2]) -> Option<[T; 2]> {
    let [[p_ax, p_ay], [v_ax, v_ay]] = a;
    let [[p_bx, p_by], [v_bx, v_by]] = b;

    let denominator = v_ay * v_bx - v_by * v_ax;
    if denominator == T::default() {
        return None;
    }

    let numerator = (p_by - p_ay) * v_bx * v_ax - p_bx * v_by * v_ax + p_ax * v_ay * v_bx;
    let intersection_x = numerator / denominator;
    let intersection_y = p_ay + (intersection_x - p_ax) * (v_ay / v_ax);
    Some([intersection_x, intersection_y])
}

fn part_1(input: &str) -> usize {
    let hailstones: Vec<_> = parse_hailstones::<f64>(input).collect();

    hailstones.iter().enumerate().flat_map(|(n, a)| {
        hailstones[n + 1..].iter().map(move |b| {
            (a, b)
        })
    }).filter(|(&a, &b)| {
        let [[p_ax, p_ay, _], [v_ax, v_ay, _]] = a;
        let [[p_bx, p_by, _], [v_bx, v_by, _]] = b;

        if let Some([intersection_x, intersection_y]) = intersect_2d([[p_ax, p_ay], [v_ax, v_ay]], [[p_bx, p_by], [v_bx, v_by]]) {
            let a_in_past = (intersection_x < p_ax) != (v_ax < 0.0);
            let b_in_past = (intersection_x < p_bx) != (v_bx < 0.0);
            let inside = intersection_x >= MIN_COORD && intersection_x <= MAX_COORD && intersection_y >= MIN_COORD && intersection_y <= MAX_COORD;
            !a_in_past && !b_in_past && inside
        } else {
            false
        }
    }).count()
}

// Math for calculating intersection coordinates in two dimensions, in StarMath syntax (e.g. Libreoffice Math)
//
// In order to guarantee a unique solution, we need to find the hailstones moving in non-intersecting non-parallel
// trajectories.
//
// In the below, P_[abc] is the initial positions of the hailstones as 3-vectors, V_[abc] are the velocities of the
// hailstones as 3-vectors, P_x is is the initial position of the thrown rock as a 3-vector, V_x is the velocity of the
// thrown rock as a 3-vector, t_[abc] are the times after which each hailstone will intersect with the rock, and
// p_[abc][xyz], v_[abc][xyz], p_x[xyz], v_x[xyz] are the respective positions and velocities as scalar values.
// In the below, x and y are the intersection coordinates, p_[ab][xy] are the initial positions, and v_[ab][xy] are the
// velocities.
//
// We can calculate the position of an object after a certain amount of time by adding its initial position the its
// velocity multiplied by the passed time. Since ech hailstone would be in the same position as the rock when it impacts
// the rock, we can set up equations for the initial values and time-to-impact for each hailstone as follows:
// left lbrace stack {
// 	P_a + V_a t_a = P_x + V_x t_a #
// 	P_b + V_b t_b = P_x + V_x t_b #
// 	P_c + V_c t_c = P_x + V_x t_c
// } right none dlrarrow
// left lbrace stack {
// 	P_a - P_x = (V_x - V_a) t_a #
// 	P_b - P_x = (V_x - V_a) t_b #
// 	P_c - P_x = (V_x - V_a) t_c
// } right none
//
// In the above (P_a - P_x) differs from (V_x - V_a) by a scalar factor. Consequently, they are parallel, which means
// their cross products are (0, 0, 0). Ditto for the other hailstones. By expanding the cross products we get three
// equations for each hailstone:
//
// left lbrace stack {
// 	(p_ay - p_xy) (v_xz - v_az) - (p_az - p_xz) (v_xy - v_ay) = 0 #
// 	(p_ax - p_xx) (v_xz - v_az) - (p_az - p_xz) (v_xx - v_ax) = 0 #
// 	(p_ay - p_xy) (v_xx - v_ax) - (p_ax - p_xx) (v_xy - v_ay) = 0 #
// 	(p_by - p_xy) (v_xz - v_bz) - (p_bz - p_xz) (v_xy - v_by) = 0 #
// 	(p_bx - p_xx) (v_xz - v_bz) - (p_bz - p_xz) (v_xx - v_bx) = 0 #
// 	(p_by - p_xy) (v_xx - v_bx) - (p_bx - p_xx) (v_xy - v_by) = 0 #
// 	(p_cy - p_xy) (v_xz - v_cz) - (p_cz - p_xz) (v_xy - v_cy) = 0 #
// 	(p_cx - p_xx) (v_xz - v_cz) - (p_cz - p_xz) (v_xx - v_cx) = 0 #
// 	(p_cy - p_xy) (v_xx - v_cx) - (p_cx - p_xx) (v_xy - v_cy) = 0
// } right none dlrarrow
// left lbrace stack {
// 	p_ay v_xz - p_ay v_az - p_xy v_xz + p_xy v_az - p_az v_xy + p_az v_ay + p_xz v_xy - p_xz v_ay = 0 #
// 	p_ax v_xz - p_ax v_az - p_xx v_xz + p_xx v_az - p_az v_xx + p_az v_ax + p_xz v_xx - p_xz v_ax = 0 #
// 	p_ay v_xx - p_ay v_ax - p_xy v_xx + p_xy v_ax - p_ax v_xy + p_ax v_ay + p_xx v_xy - p_xx v_ay = 0 #
// 	p_by v_xz - p_by v_bz - p_xy v_xz + p_xy v_bz - p_bz v_xy + p_bz v_by + p_xz v_xy - p_xz v_by = 0 #
// 	p_bx v_xz - p_bx v_bz - p_xx v_xz + p_xx v_bz - p_bz v_xx + p_bz v_bx + p_xz v_xx - p_xz v_bx = 0 #
// 	p_by v_xx - p_by v_bx - p_xy v_xx + p_xy v_bx - p_bx v_xy + p_bx v_by + p_xx v_xy - p_xx v_by = 0 #
// 	p_cy v_xz - p_cy v_cz - p_xy v_xz + p_xy v_cz - p_cz v_xy + p_cz v_cy + p_xz v_xy - p_xz v_cy = 0 #
// 	p_cx v_xz - p_cx v_cz - p_xx v_xz + p_xx v_cz - p_cz v_xx + p_cz v_cx + p_xz v_xx - p_xz v_cx = 0 #
// 	p_cy v_xx - p_cy v_cx - p_xy v_xx + p_xy v_cx - p_cx v_xy + p_cx v_cy + p_xx v_xy - p_xx v_cy = 0
// } right none
//
// Now we have 9 equations with 6 unknowns. Unfortunately, some of the terms are non-linear, but fortunately, these
// terms are all purely based on the unknown, and thus identical for each each hailstone. So by subtracting the last
// hailstone from the other two, and rearranging it a bit, we get:
//
// left lbrace stack {
// 	(p_ay - p_cy) v_xz + (v_az - v_cz) p_xy + (p_cz - p_az) v_xy + (v_cy - v_ay) p_xz = p_ay v_az - p_az v_ay - p_cy v_cz + p_cz v_cy #
// 	(p_ax - p_cx) v_xz + (v_az - v_cz) p_xx + (p_cz - p_az) v_xx + (v_cx - v_ax) p_xz = p_ax v_az - p_az v_ax - p_cx v_cz + p_cz v_cx #
// 	(p_ay - p_cy) v_xx + (v_ax - v_cx) p_xy + (p_cx - p_ax) v_xy + (v_cy - v_ay) p_xx = p_ay v_ax - p_ax v_ay - p_cy v_cx + p_cx v_cy #
// 	(p_by - p_cy) v_xz + (v_bz - v_cz) p_xy + (p_cz - p_bz) v_xy + (v_cy - v_by) p_xz = p_by v_bz - p_bz v_by - p_cy v_cz + p_cz v_cy #
// 	(p_bx - p_cx) v_xz + (v_bz - v_cz) p_xx + (p_cz - p_bz) v_xx + (v_cx - v_bx) p_xz = p_bx v_bz - p_bz v_bx - p_cx v_cz + p_cz v_cx #
// 	(p_by - p_cy) v_xx + (v_bx - v_cx) p_xy + (p_cx - p_bx) v_xy + (v_cy - v_by) p_xx = p_by v_bx - p_bx v_by - p_cy v_cz + p_cz v_cy
// } right none
//
// From these equations we can then construct a matrix, with the columns p_xx, p_xy, p_xz, v_xx, v_xy, v_xz, on which
// we can then perform gaussian elimination:
//
// left [ matrix {
//  0 # v_az - v_cz # v_cy - v_ay # 0 # p_cz - p_az # p_ay - p_cy ##
//  v_az - v_cz # 0 # v_cx - v_ax # p_cz - p_az # 0 # p_ax - p_cx ##
//  v_cy - v_ay # v_ax - v_cx # 0 # p_ay - p_cy # p_cx - p_ax # 0 ##
//  0 # v_bz - v_cz # v_cy - v_by # 0 # p_cz - p_bz # p_by - p_cy ##
//  v_bz - v_cz # 0 # v_cx - v_bx # p_cz - p_bz # 0 # p_bx - p_cx ##
//  v_cy - v_by # v_bx - v_cx # 0 # p_by - p_cy # p_cx - p_bx # 0
// } " " mline " " stack {
//  p_ay v_az - p_az v_ay - p_cy v_cz + p_cz v_cy #
//  p_ax v_az - p_az v_ax - p_cx v_cz + p_cz v_cx #
//  p_ay v_ax - p_ax v_ay - p_cy v_cx + p_cx v_cy #
//  p_by v_bz - p_bz v_by - p_cy v_cz + p_cz v_cy #
//  p_bx v_bz - p_bz v_bx - p_cx v_cz + p_cz v_cx #
//  p_by v_bx - p_bx v_by - p_cy v_cx + p_cx v_cy
// } right ]

// f64 isn't precise enough, as well as leading to trouble when comparing to 0. Rational64 isn't big enough.
type CoordType = num::rational::Ratio<i128>;

fn part_2(input: &str) -> CoordType {
    let mut hailstones = Vec::new();

    // Find three non-parallel non-intersecting hailstones
    for hailstone in parse_hailstones::<CoordType>(input) {
        let [[p_ax, p_ay, z_0a], [v_ax, v_ay, v_az]] = hailstone;

        if hailstones.iter().all(|other| {
            let &[[p_bx, p_by, z_0b], [v_bx, v_by, v_bz]] = other;
            match (
                intersect_2d([[p_ax, p_ay], [v_ax, v_ay]], [[p_bx, p_by], [v_bx, v_by]]),
                intersect_2d([[p_ax, z_0a], [v_ax, v_az]], [[p_bx, z_0b], [v_bx, v_bz]]),
            ) {
                // Parallel
                (None, None) => false,
                // Parallel from one perspective, intersecting from another
                (None, Some(_)) | (Some(_), None) => true,
                // Intersecting from both perspectives. Check if intersection is in the same place
                (Some([intersection_x_y, _]), Some([intersection_x_z, _])) => intersection_x_y != intersection_x_z,
            }
        }) {
            hailstones.push(hailstone);
        }

        if hailstones.len() == 3 {
            break;
        }
    }

    // Set up equation matrix
    let [[p_ax, p_ay, p_az], [v_ax, v_ay, v_az]] = hailstones[0];
    let [[p_bx, p_by, p_bz], [v_bx, v_by, v_bz]] = hailstones[1];
    let [[p_cx, p_cy, p_cz], [v_cx, v_cy, v_cz]] = hailstones[2];
    let mut equations = [
        [CoordType::default(), v_az - v_cz, v_cy - v_ay, CoordType::default(), p_cz - p_az, p_ay - p_cy, p_ay * v_az - p_az * v_ay - p_cy * v_cz + p_cz * v_cy],
        [v_az - v_cz, CoordType::default(), v_cx - v_ax, p_cz - p_az, CoordType::default(), p_ax - p_cx, p_ax * v_az - p_az * v_ax - p_cx * v_cz + p_cz * v_cx],
        [v_cy - v_ay, v_ax - v_cx, CoordType::default(), p_ay - p_cy, p_cx - p_ax, CoordType::default(), p_ay * v_ax - p_ax * v_ay - p_cy * v_cx + p_cx * v_cy],
        [CoordType::default(), v_bz - v_cz, v_cy - v_by, CoordType::default(), p_cz - p_bz, p_by - p_cy, p_by * v_bz - p_bz * v_by - p_cy * v_cz + p_cz * v_cy],
        [v_bz - v_cz, CoordType::default(), v_cx - v_bx, p_cz - p_bz, CoordType::default(), p_bx - p_cx, p_bx * v_bz - p_bz * v_bx - p_cx * v_cz + p_cz * v_cx],
        [v_cy - v_by, v_bx - v_cx, CoordType::default(), p_by - p_cy, p_cx - p_bx, CoordType::default(), p_by * v_bx - p_bx * v_by - p_cy * v_cx + p_cx * v_cy],
    ];

    // Perform gaussian elimination
    // Iterate diagonally from top left, to turn matrix into reduced row echelon form
    for i in 0..6 {
        // Find non-zero item in current column, from current row or after
        let non_zero_row = (i..6).find(|&row| {
            equations[row][i] != CoordType::default()
        }).unwrap();

        // Swap current row with first non-zero row
        if non_zero_row != i {
            (equations[i], equations[non_zero_row]) = (equations[non_zero_row], equations[i]);
        }

        // Divide row by value at current pos, to turn value into 1
        let curr_val = equations[i][i];
        equations[i][i] = CoordType::from_integer(1);
        for item in &mut equations[i][i + 1..] {
            *item /= curr_val;
        }

        // Subtract multiple of current row from lower rows, to turn column below current item to 0
        for row in i + 1..6 {
            let multiple = equations[row][i];
            equations[row][i] = CoordType::default();
            if multiple != CoordType::default() {
                for col in i + 1..7 {
                    equations[row][col] -= equations[i][col] * multiple;
                }
            }
        }
    }

    // Iterate diagonally from bottom right, to turn matrix (except last column) into unit matrix.
    for i in (0..6).rev() {
        for row in 0..i {
            equations[row][6] -= equations[i][6] * equations[row][i];
            equations[row][i] = CoordType::default();
        }
    }

    equations.iter().take(3).map(|x| x[6]).sum::<CoordType>()
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
