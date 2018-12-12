use std::ops::Add;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn from_string(input: &str) -> Self {
        let numbers: Vec<i64> = input
            .replace("<", "")
            .replace(">", "")
            .split(",")
            .map(|value| {
                value
                    .trim()
                    .parse::<i64>()
                    .expect(&format!("Invalid numer in `{}`", value))
            }).collect();

        assert!(
            numbers.len() == 2,
            "Expected exactly two numbers per vector got {} for {}",
            numbers.len(),
            input
        );

        Self::new(numbers[0], numbers[1])
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Debug)]
struct Particle {
    position: Vector,
    velocity: Vector,
}

impl Particle {
    fn new(position: Vector, velocity: Vector) -> Self {
        Self { position, velocity }
    }

    fn tick(&mut self) {
        self.position = self.position + self.velocity;
    }
}

fn parse(input: &str) -> Vec<Particle> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|line| {
            let idx = line
                .chars()
                .position(|char| char == '>')
                .expect(&format!("Expected to find at least one `>` in {}", line));
            let (position_definition, velocity_definition) = line.split_at(idx + 1);
            let cleaned_position = position_definition.trim().trim_start_matches("position=");
            let cleaned_veclocity = velocity_definition.trim().trim_start_matches("velocity=");

            Particle::new(
                Vector::from_string(cleaned_position),
                Vector::from_string(cleaned_veclocity),
            )
        }).collect()
}

fn extract_extremes(particles: &[Particle]) -> ((i64, i64), (i64, i64)) {
    let max_x = particles.iter().map(|p| p.position.x).max().unwrap();
    let max_y = particles.iter().map(|p| p.position.y).max().unwrap();

    let min_x = particles.iter().map(|p| p.position.x).min().unwrap();
    let min_y = particles.iter().map(|p| p.position.y).min().unwrap();

    ((max_x, min_x), (max_y, min_y))
}

fn format_particles(particles: &[Particle]) -> String {
    let ((max_x, min_x), (max_y, min_y)) = extract_extremes(particles);
    let (width, height) = (max_x - min_x + 1, max_y - min_y + 1);
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width as usize]; height as usize];

    for particle in particles {
        let x = width - max_x + particle.position.x - 1;
        let y = height - max_y + particle.position.y - 1;

        grid[y as usize][x as usize] = '#';
    }

    grid.into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn star_one(input: &str, ticks: usize) -> String {
    let mut particles = parse(input);

    for i in 0..ticks {
        for particle in &mut particles {
            particle.tick();
        }
    }

    let result = format_particles(&particles);

    result
}

#[cfg(test)]
mod tests {
    use super::star_one;
    static EXAMPLE: &str = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
    static EXEPCTED_OUTPUT: &str = "#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(EXAMPLE, 3), EXEPCTED_OUTPUT)
    }
}
