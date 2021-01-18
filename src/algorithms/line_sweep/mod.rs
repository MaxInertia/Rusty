use std::collections::BinaryHeap;

pub mod event;
pub mod line_segment;

use line_segment::LineSegment;
use line_segment::Point;

// Intersection: two lines and the point they overlap
struct Intersection {
    lines: [LineSegment; 2],
    overlap: Point,
}

fn find_all_intersections(mut lines: BinaryHeap<LineSegment>) -> Vec<Intersection> {
    if lines.is_empty() {
        return vec![];
    }

    //use events::Event::*;
    //let mut es = BinaryHeap::new();
    // TODO: Cannot check for neighboring line segments with the Binary Heap. Create custom heap?
    //es.append(Upper(lines.pop()));
    //es.append(Upper(lines.pop()));

    unimplemented!() // TODO: Implement
}

fn find_intersection(a: LineSegment, b: LineSegment) -> Option<Intersection> {
    unimplemented!() // TODO: Implement
}

#[test]
fn no_lines() {
    //let mut lines = BTreeSet::new();
    let lines = BinaryHeap::new();
    let intersections = find_all_intersections(lines);
    assert!(intersections.is_empty())
}

#[test]
fn one_line() {
    //let mut lines = BTreeSet::new();
    let mut lines = BinaryHeap::new();
    lines.push(LineSegment::new((0, 1), (0, 2)));
    let intersections = find_all_intersections(lines);
    assert!(intersections.is_empty())
}

#[test]
fn two_lines() {
    //let mut lines = BTreeSet::new();
    let mut lines = BinaryHeap::new();
    lines.push(LineSegment::new((0, 1), (0, 2)));
    lines.push(LineSegment::new((0, -1), (0, -2)));
    let intersections = find_all_intersections(lines);
    assert!(intersections.is_empty())
}

#[test]
fn two_lines_one_intersections() {
    //let mut lines = BTreeSet::new();
    let mut lines = BinaryHeap::new();
    lines.push(LineSegment::new((2, 2), (-2, -2)));
    lines.push(LineSegment::new((-2, 2), (2, -2)));
    let intersections = find_all_intersections(lines);
    assert!(!intersections.is_empty());
    let zero: [i32; 2] = [0, 0];
    assert_eq!(intersections[0].overlap, zero);
}
