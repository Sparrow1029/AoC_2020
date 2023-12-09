use aoc_2020::{get_puzzle_input_string, grid::Grid2D};

fn get_trees_for_slope(grid: &Grid2D<char>, x_slope: usize, y_slope: usize) -> usize {
    let mut trees = 0;
    for (x, y) in (0..grid.height).zip(0..grid.height) {
        let pt = ((x * x_slope) % grid.width, y * y_slope).into();
        // println!("\t{pt:?}");
        if let Some(c) = grid.get(pt) {
            if *c == '#' {
                trees += 1
            }
        }
    }
    trees
}

fn part1(grid: &Grid2D<char>) -> usize {
    get_trees_for_slope(grid, 3, 1)
}

fn part2(grid: &Grid2D<char>) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(x, y)| get_trees_for_slope(grid, x, y))
        .product()
}

fn main() {
    let grid: Grid2D<char> = get_puzzle_input_string(3)
        .expect("I/O error")
        .as_str()
        .into();

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod test {
    use crate::*;

    const SAMPLE: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_part1() {
        let grid: Grid2D<char> = SAMPLE.into();
        assert_eq!(part1(&grid), 7);
    }

    #[test]
    fn test_part2() {
        let grid: Grid2D<char> = SAMPLE.into();
        let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        for (x, y) in slopes {
            println!("Checking slope: {:?}", (x, y));
            let slope = get_trees_for_slope(&grid, x, y);
            println!("\ttrees: {slope}\n\n");
            // println!("");
            // println!("");
        }
        // let (x, y) = slopes[4];
        // println!(
        //     "slope: {:?} -> {}",
        //     (x, y),
        //     get_trees_for_slope(&grid, x, y)
        // );
        assert_eq!(part2(&grid), 336);
    }
}
