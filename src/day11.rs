fn nth_digit(number: usize, idx: usize) -> Option<usize> {
    let mut n = number;
    let mut i = 1;

    while n > 0 {
        n = n / 10;
        if i == idx {
            return Some(n % 10);
        }

        i += 1;
    }

    None
}

pub fn power(grid: &Vec<Vec<i64>>, location: &(usize, usize), window_size: usize) -> i64 {
    (location.0..(location.0 + window_size))
        .map(|x| (location.1..(location.1 + window_size)).fold(0, |acc, y| acc + grid[x][y]))
        .sum()
}

pub fn build_grid(serial: usize, size: usize) -> Vec<Vec<i64>> {
    (0..size)
        .map(|x| {
            (0..size)
                .map(|y| {
                    let rack_id = x + 1 + 10;

                    let interim = (rack_id * (y + 1) + serial) * rack_id;
                    (nth_digit(interim, 2).unwrap_or(0) as i64) - 5
                }).collect()
        }).collect()
}

pub fn star_one(serial: usize, size: usize, window: usize) -> (usize, usize) {
    let grid = build_grid(serial, size);

    let result = (0..size - window)
        .flat_map(|x| {
            (0..size - window)
                .clone()
                .map(|y| {
                    let coordinate = (x, y);
                    return (power(&grid, &coordinate, window), coordinate);
                }).collect::<Vec<(i64, (usize, usize))>>()
        }).max_by(|(a, _), (b, _)| a.cmp(b))
        .and_then(|(power, (x, y))| Some((power, (x + 1, y + 1))))
        .unwrap();

    result.1
}

pub fn star_two(serial: usize, size: usize) -> (usize, usize, usize) {
    let grid = build_grid(serial, size);

    // Who needs smart realisations when you have a fast language and some patience?
    let (power, (x, y), final_size) = (0..size)
        .map(|window| {
            (0..size - window)
                .flat_map(|x| {
                    (0..size - window)
                        .clone()
                        .map(|y| {
                            let coordinate = (x, y);
                            return (power(&grid, &coordinate, window), coordinate);
                        }).collect::<Vec<(i64, (usize, usize))>>()
                }).max_by(|(a, _), (b, _)| a.cmp(b))
                .and_then(|(power, (x, y))| Some((power, (x + 1, y + 1), window)))
                .unwrap()
        }).max_by(|(a, _, _), (b, _, _)| a.cmp(b))
        .unwrap();

    (x, y, final_size)
}

#[cfg(test)]
mod tests {
    use super::{build_grid, nth_digit, power, star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(18, 300, 3), (33, 45));
        assert_eq!(star_one(42, 300, 3), (21, 61));
    }

    #[test]
    fn test_star_two() {
        // These two are slow
        // assert_eq!(star_two(18, 300), (90, 269, 16));
        // assert_eq!(star_two(42, 300), (232, 251, 12));
    }

    #[test]
    fn test_grid() {
        {
            let grid = build_grid(8, 300);

            assert_eq!(grid[2][4], 4);
        }

        {
            let grid = build_grid(57, 300);

            assert_eq!(grid[121][78], -5);
        }
    }

    #[test]
    fn test_power() {
        let grid = build_grid(18, 300);
        let coord = (32, 44);

        assert_eq!(power(&grid, &coord, 3), 29);
    }

    #[test]
    fn test_nth_digit() {
        assert_eq!(nth_digit(0, 2), None);
        assert_eq!(nth_digit(949, 2), Some(9));
        assert_eq!(nth_digit(12345, 2), Some(3));
        assert_eq!(nth_digit(1384020, 2), Some(0));
        assert_eq!(nth_digit(17567680, 2), Some(6));
    }
}
