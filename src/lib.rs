#![no_std]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_debug_implementations
)]

extern crate alloc;
use alloc::vec::Vec;

pub type Point = glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
/// A linked-list node, with usize references to a backing array.
pub struct Node {
    /// Position of the node's vertex.
    pub pos: Point,
    /// A reference to the previous vertex at the end of a connected edge.
    pub prev: usize,
    /// A reference to the next vertex at the end of a connected edge.
    pub next: usize,
}

/// The backing storage for the linked list.
#[derive(Debug, Clone, PartialEq)]
pub struct Earcut {
    pub nodes: Vec<Node>,
}
impl Earcut {
    #[must_use]
    /// Create a new earcut instance.
    ///
    /// Ensure that the point are valid (in order, deduplicated, etc).
    pub fn new(polygon: &[Point]) -> Self {
        let len = polygon.len();
        let nodes = polygon
            .iter()
            .enumerate()
            .map(|(idx, pos)| Node {
                pos: *pos,
                prev: idx.checked_sub(1).unwrap_or(len - 1),
                next: (idx + 1) % len,
            })
            .collect::<Vec<_>>();

        Self { nodes }
    }

    #[must_use]
    /// Perform the earcut algorithm.
    pub fn earcut(mut self) -> Vec<Triangle> {
        let mut tris = Vec::with_capacity(self.nodes.len() - 2);

        let mut current_node_idx = 0;

        loop {
            let current_node = self.nodes[current_node_idx];
            if current_node.next == current_node.prev {
                break;
            }

            let previous_node = self.nodes[current_node.prev];
            let next_node = self.nodes[current_node.next];

            let current_triangle = Triangle(previous_node.pos, current_node.pos, next_node.pos);

            if !current_triangle.is_reflex() {
                let mut poly_is_ear = true;
                let mut checking_node_idx = next_node.next;
                while checking_node_idx != current_node.prev {
                    let checking_node = self.nodes[checking_node_idx];

                    if current_triangle.contains(checking_node.pos) {
                        poly_is_ear = false;
                        break;
                    }

                    checking_node_idx = checking_node.next;
                }

                if poly_is_ear {
                    self.nodes[current_node.prev].next = current_node.next;
                    self.nodes[current_node.next].prev = current_node.prev;

                    tris.push(current_triangle);
                }
            }

            current_node_idx = current_node.next;
        }

        tris
    }
}

#[derive(Debug, Clone, Copy)]
/// A triangle, made up of 3 points.
pub struct Triangle(pub Point, pub Point, pub Point);
impl Triangle {
    #[must_use]
    /// Check if a given point lies within a triangle.
    pub fn contains(self, point: Point) -> bool {
        #[inline]
        fn cross_product(a: Point, b: Point) -> f32 {
            a.x * b.y - b.x * a.y
        }

        // Center abc around the origin.
        let a = self.0 - point;
        let b = self.1 - point;
        let c = self.2 - point;

        // Get the signed direction of the perpendicular and ensure they all face the origin.
        let ca = cross_product(c, a) >= 0.0;
        let ab = cross_product(a, b) >= 0.0;
        let bc = cross_product(b, c) >= 0.0;

        ca && ab && bc
    }

    #[must_use]
    /// Check if the triangle is reflex.
    pub fn is_reflex(self) -> bool {
        let ba = self.1 - self.0;
        let cb = self.2 - self.1;

        let a = ba.y * cb.x;
        let b = cb.y * ba.x;

        (a - b) > 0.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn earcut_new_is_valid_indices() {
        let polygon = [Point::new(0.0, 0.0); 3];

        let earcut = Earcut::new(&polygon);

        assert_eq!(
            earcut.nodes.as_slice(),
            &[
                Node {
                    pos: Point::new(0.0, 0.0),
                    prev: 2,
                    next: 1
                },
                Node {
                    pos: Point::new(0.0, 0.0),
                    prev: 0,
                    next: 2
                },
                Node {
                    pos: Point::new(0.0, 0.0),
                    prev: 1,
                    next: 0
                },
            ]
        )
    }
}
