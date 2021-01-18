use super::line_segment::*;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
pub enum Event {
    UpperPoint(LineSegment),                     // Top point on a LineSegment
    CrossPoint(Point, LineSegment, LineSegment), // Intersection between two LineSegments
    LowerPoint(LineSegment),                     // Bottom point on a LineSegment
}

impl Event {
    // Value used that orders events in the order the sweep-line reaches them.
    fn cmp_value(&self) -> i32 {
        match self {
            Event::UpperPoint(line) => line.top_endpoint()[Y],
            Event::LowerPoint(line) => line.bot_endpoint()[Y],
            Event::CrossPoint(p, _, _) => p[Y],
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        let self_top = self.cmp_value();
        let other_top = other.cmp_value();

        if self_top < other_top {
            Ordering::Less
        } else if self_top > other_top {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
