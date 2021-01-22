use std::cmp::Ordering;

// 2D Point
pub type Point = [i32; 2];

pub const X: usize = 0;
pub const Y: usize = 1;

// 2D Line
#[derive(Debug, Eq)]
pub struct LineSegment {
    endpoints: [Point; 2],
}

impl LineSegment {
    pub fn new(a: (i32, i32), b: (i32, i32)) -> LineSegment {
        LineSegment {
            endpoints: [[a.0, a.1], [b.0, b.1]],
        }
    }

    // Returns the endpoint with the largest y-value
    // if equal, the first endpoint is returned
    pub fn top_endpoint(&self) -> Point {
        if self.endpoints[0][Y] >= self.endpoints[1][Y] {
            self.endpoints[0]
        } else {
            self.endpoints[1]
        }
    }

    // Returns the endpoint with the smallest y-value
    // if equal, the second endpoint is returned
    pub fn bot_endpoint(&self) -> Point {
        if self.endpoints[0][Y] >= self.endpoints[1][Y] {
            self.endpoints[1]
        } else {
            self.endpoints[0]
        }
    }
}

impl Ord for LineSegment {
    fn cmp(&self, other: &LineSegment) -> Ordering {
        let self_top = self.top_endpoint()[Y];
        let other_top = other.top_endpoint()[Y];
        if self_top < other_top {
            Ordering::Less
        } else if self_top > other_top {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for LineSegment {
    fn eq(&self, other: &LineSegment) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for LineSegment {
    fn partial_cmp(&self, other: &LineSegment) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn getters() {
    let line = LineSegment::new((0, 1), (2, 3));
    assert_eq!(line.bot_endpoint()[X], 0);
    assert_eq!(line.bot_endpoint()[Y], 1);
    assert_eq!(line.top_endpoint()[X], 2);
    assert_eq!(line.top_endpoint()[Y], 3);
}

#[test]
fn ordering() {
    let new_ls = LineSegment::new;
    let zero = new_ls((0, 0), (0, 0));
    {
        let p2 = new_ls((0, 0), (0, 0));
        assert_eq!(zero, p2);
    }

    // Only the highest point on a line segment is considered for equality

    // less than zero
    assert!(zero > new_ls((0, -1), (0, -2)));
    assert!(zero > new_ls((0, -2), (0, -1)));
    // more than zero
    assert!(zero < new_ls((0, 0), (0, 2)));
    assert!(zero < new_ls((0, 2), (0, 0)));

    // Changes in X are not relevant to equality here
    assert_eq!(zero, new_ls((-2, 0), (0, 0)));
    assert_eq!(zero, new_ls((0, 0), (-2, 0)));
    assert_eq!(zero, new_ls((0, 0), (2, 0)));
    assert_eq!(zero, new_ls((2, 0), (0, 0)));

    // Ignoring lowest point
    assert_eq!(zero, new_ls((0, 0), (0, -2)));
    assert_eq!(zero, new_ls((0, -2), (0, 0)));
}
