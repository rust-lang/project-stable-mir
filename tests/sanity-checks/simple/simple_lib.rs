//! Just a simple library

pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> u64 {
        let (x_dist, x_over) = (self.x - other.x).overflowing_pow(2);
        let (y_dist, y_over) = (self.y - other.y).overflowing_pow(2);
        if y_over | x_over {
            panic!("overflow");
        }

        let dist = (x_dist as u64 + y_dist as u64) >> 1;
        dist
    }

    pub fn distance_root(&self) -> u64 {
        self.distance(&Point { x: 0, y: 0 })
    }
}
