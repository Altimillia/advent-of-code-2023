use std::fmt::Display;
use itertools::min;
use crate::domain::vector3::Vector3;
use crate::domain::vector3_large::Vector3Large;

pub fn part_one(input: String) -> impl Display {
    let mut hail:Vec<Hail> = input.lines().map(|line| Hail::parse(line)).collect();
    get_intersections(hail)
}

fn get_intersections(hail_stones:Vec<Hail>) -> i32 {
    let mut counter = 0;
    let min_zone = Vector3Large::new(200000000000000.0, 200000000000000.0, 0.0);
    let max_zone = Vector3Large::new(400000000000000.0, 400000000000000.0, 0.0);
    for i in 0..hail_stones.len() {
        for j in i + 1..hail_stones.len() {
            let (hail_1, hail_2) = (&hail_stones[i], &hail_stones[j]);
            if check_intersection_2(hail_1.clone(), hail_2.clone(), min_zone, max_zone) {
                counter += 1;
            }
        }
    }

    counter
}

pub fn part_two(input: String) -> impl Display {
    0
}

fn check_intersection(hail_1: Hail, hail_2: Hail, min_zone: Vector3Large, max_zone: Vector3Large) -> bool {

    // Lets get the hail at min position x,y
    let relative_velocity = hail_2.velocity - hail_1.velocity;
    let relative_position = hail_2.position - hail_1.position;

    let xy_dot = relative_velocity.x * relative_position.x + relative_velocity.y * relative_position.y;

    if xy_dot < 0.0 {
        let collision_time = -(relative_position.x * relative_velocity.x + relative_position.y * relative_position.y) / xy_dot;
        let collision_point = Vector3Large::new(hail_1.position.x + hail_1.velocity.x * collision_time, hail_1.position.y + hail_1.velocity.y * collision_time, 0.0);

        println!("{}", collision_point);
        if collision_point.x > min_zone.x && collision_point.x < max_zone.x && collision_point.y > min_zone.y && collision_point.y < max_zone.y
        {
            return true
        }
    }

    false
}

fn check_intersection_2(hail_1: Hail, hail_2: Hail, min_zone: Vector3Large, max_zone: Vector3Large) -> bool {
    if hail_1.a * hail_2.b == hail_1.b * hail_2.a {
        return false;
    }

    let x = (hail_1.c * hail_2.b - hail_2.c * hail_1.b) / (hail_1.a * hail_2.b - hail_2.a * hail_1.b);
    let y = (hail_2.c * hail_1.a - hail_1.c * hail_2.a) / (hail_1.a * hail_2.b - hail_2.a * hail_1.b);

    if x >= min_zone.x && x <= max_zone.x && y >= min_zone.y && y <= max_zone.y {
        return [hail_1, hail_2].iter().all(|h| (x - h.position.x) * h.velocity.x >= 0.0 && (y - h.position.y) * h.velocity.y >= 0.0);
    }

    false
}

#[derive(Debug, PartialEq, Clone, Copy, PartialOrd)]
struct Hail {
    position: Vector3Large,
    velocity: Vector3Large,
    a: f64,
    b: f64,
    c: f64
}
impl Hail {
    fn parse(input_line: &str) -> Self {
        let [start, velocity] = input_line.split("@").map(|s| Vector3Large::parse(s)).next_chunk().unwrap();

        Hail { position: start, velocity, a: velocity.y, b: -velocity.x, c: velocity.y * start.x - velocity.x * start.y}
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day_24::{check_intersection, check_intersection_2, Hail};
    use crate::domain::vector3_large::Vector3Large;

    #[test]
    fn can_get_hail_intersection_in_zone() {
        let hail_1 = Hail::parse(r#"19, 13, 30 @ -2,  1, -2"#);
        let hail_2 = Hail::parse(r#"18, 19, 22 @ -1, -1, -2"#);

        let result = check_intersection_2(hail_1, hail_2, Vector3Large::new(7.0,7.0,7.0), Vector3Large::new(27.0,27.0,27.0));

        assert_eq!(result, true);
    }

    #[test]
    fn can_get_hail_intersection_in_past() {
        let hail_1 = Hail::parse(r#"19, 13, 30 @ -2, 1, -2"#);
        let hail_2 = Hail::parse(r#"20, 19, 15 @ 1, -5, -3"#);

        let result = check_intersection_2(hail_1, hail_2, Vector3Large::new(7.0,7.0,7.0), Vector3Large::new(27.0,27.0,27.0));

        assert_eq!(result, false);
    }

    #[test]
    fn can_get_intersection_out_of_area() {
        let hail_1 = Hail::parse(r#"19, 13, 30 @ -2, 1, -2"#);
        let hail_2 = Hail::parse(r#"12, 31, 28 @ -1, -2, -1"#);

        let result = check_intersection_2(hail_1, hail_2, Vector3Large::new(7.0,7.0,7.0), Vector3Large::new(27.0,27.0,27.0));

        assert_eq!(result, false);
    }

    #[test]
    fn parallel_returns_false() {
        let hail_1 = Hail::parse(r#"18, 19, 22 @ -1, -1, -2"#);
        let hail_2 = Hail::parse(r#"20, 25, 34 @ -2, -2, -4"#);

        let result = check_intersection_2(hail_1, hail_2, Vector3Large::new(7.0,7.0,7.0), Vector3Large::new(27.0,27.0,27.0));

        assert_eq!(result, false);
    }
}