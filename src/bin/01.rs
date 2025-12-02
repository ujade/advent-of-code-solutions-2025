advent_of_code::solution!(1);
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    // fn new(real: f64, imag: f64) -> Self {
    //     Complex { real, imag }
    // }

    fn from_polar(radius: f64, angle: f64) -> Self {
        Complex {
            real: radius * angle.cos(),
            imag: radius * angle.sin(),
        }
    }

    fn mul(&self, other: &Complex) -> Complex {
        // (a + bi)(c + di) = (ac - bd) + (ad + bc)i
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }

    // angle in radians [0, 2π)
    fn angle(&self) -> f64 {
        let mut angle = self.imag.atan2(self.real);
        if angle < 0.0 {
            angle += 2.0 * PI;
        }
        angle
    }
}

struct CircleIterator {
    position: Complex,
    step_angle: f64,
    total_steps: usize,
    radius: f64,
    zero_crossings: u64,
}

impl CircleIterator {
    fn new(total_steps: usize, radius: f64, initial_position: usize) -> Self {
        let step_angle = 2.0 * PI / (total_steps as f64);
        let initial_angle = step_angle * (initial_position as f64);

        CircleIterator {
            position: Complex::from_polar(radius, initial_angle),
            step_angle,
            total_steps,
            radius,
            zero_crossings: 0,
        }
    }

    // fn current_index(&self) -> usize {
    //     let angle = self.position.angle();
    //     let index = ((angle / self.step_angle) + 0.5).floor() as usize;
    //     index % self.total_steps
    // }

    fn rotate(&mut self, steps: usize, left: bool) -> u64 {
        let mut crossings = 0u64;

        let rotation_angle = if left {
            -self.step_angle
        } else {
            self.step_angle
        };

        let rotator = Complex::from_polar(1.0, rotation_angle);

        for _ in 0..steps {
            self.position = self.position.mul(&rotator);

            let angle = self.position.angle();
            let epsilon = self.step_angle * 0.5; //half steps are precise enough

            if angle < epsilon || angle > (2.0 * PI - epsilon) {
                crossings += 1;
            }
        }

        self.zero_crossings += crossings;
        crossings
    }

    fn move_left(&mut self, steps: usize) -> u64 {
        self.rotate(steps, true)
    }

    fn move_right(&mut self, steps: usize) -> u64 {
        self.rotate(steps, false)
    }

    fn total_zero_crossings(&self) -> u64 {
        self.zero_crossings
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut circle = CircleIterator::new(100, 1.0, 50);

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (direction, steps_str) = line.split_at(1);
        let steps: usize = steps_str.parse().ok()?;

        match direction {
            "L" => {
                circle.move_left(steps);
            }
            "R" => {
                circle.move_right(steps);
            }
            _ => continue,
        };
    }

    Some(circle.total_zero_crossings())
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut circle = CircleIterator::new(100, 1.0, 0);

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (direction, steps_str) = line.split_at(1);
        let steps: usize = steps_str.parse().ok()?;

        match direction {
            "L" => {
                circle.move_left(steps);
            }
            "R" => {
                circle.move_right(steps);
            }
            _ => continue,
        };
    }

    Some(circle.total_zero_crossings())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
