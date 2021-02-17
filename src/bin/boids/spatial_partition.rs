// use super::*;
use bevy::prelude::Vec2;

#[derive(Debug, Default, Clone, Copy)]
pub struct AABB {
    pub center: Vec2,
    pub half_dimension: Vec2,
}

impl Eq for AABB {}

impl PartialEq for AABB {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.half_dimension == other.half_dimension
    }
}

impl AABB {
    pub fn new(center: Vec2, half_dimension: Vec2) -> Self {
        Self {
            center,
            half_dimension,
        }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x <= self.center.x + self.half_dimension.x
            && point.x >= self.center.x - self.half_dimension.x
            && point.y <= self.center.y + self.half_dimension.y
            && point.y >= self.center.y - self.half_dimension.y
    }

    pub fn intersects(&self, range: &AABB) -> bool {
        let one_position = range.center;
        let two_position = self.center;

        let one_size = range.half_dimension * 2.0;
        let two_size = self.half_dimension * 2.0;

        let one_half_size = range.half_dimension;
        let two_half_size = self.half_dimension;

        let one_position = Vec2::new(
            one_position.x - one_half_size.x,
            one_position.y - one_half_size.y,
        );
        let two_position = Vec2::new(
            two_position.x - two_half_size.x,
            two_position.y - two_half_size.y,
        );

        // collision x-axis?
        let collision_x = one_position.x + one_size.x >= two_position.x
            && two_position.x + two_size.x >= one_position.x;
        // collision y-axis?
        let collision_y = one_position.y + one_size.y >= two_position.y
            && two_position.y + two_size.y >= one_position.y;

        collision_x && collision_y
    }
}

#[derive(Debug, PartialEq)]
enum QuadTreeData {
    Leaf(Vec<(Vec2, usize)>),
    Node(Vec<QuadTree>),
    Empty,
}

#[derive(Debug, PartialEq)]
pub struct QuadTree {
    root: QuadTreeData,
    pub boundary: AABB,
}

impl QuadTree {
    pub fn new(boundary: AABB) -> Self {
        Self {
            root: QuadTreeData::Empty,
            boundary,
        }
    }

    pub fn insert(&mut self, point: Vec2, data: usize) {
        if self.boundary.contains(point) {
            match &mut self.root {
                QuadTreeData::Empty => {
                    self.root = QuadTreeData::Leaf(vec![(point, data)]);
                }
                QuadTreeData::Node(ref mut nodes) => {
                    nodes[0].insert(point, data);
                    nodes[1].insert(point, data);
                    nodes[2].insert(point, data);
                    nodes[3].insert(point, data);
                }
                QuadTreeData::Leaf(ref mut points) => {
                    if points.len() < 4 {
                        points.push((point, data));
                    } else {
                        self.subdivide();
                        if let QuadTreeData::Node(nodes) = &mut self.root {
                            nodes[0].insert(point, data);
                            nodes[1].insert(point, data);
                            nodes[2].insert(point, data);
                            nodes[3].insert(point, data);
                        }
                    }
                }
            }
        }
    }

    pub fn subdivide(&mut self) {
        let pos = self.boundary.center;
        let half_size = self.boundary.half_dimension;

        let ne = AABB::new(pos + half_size / 2.0, half_size / 2.0);
        let nw = AABB::new(pos - half_size / 2.0, half_size / 2.0);
        let neg_half_size = half_size * Vec2::new(1.0, -1.0);
        let se = AABB::new(pos + neg_half_size / 2.0, half_size / 2.0);
        let sw = AABB::new(pos - neg_half_size / 2.0, half_size / 2.0);

        if let QuadTreeData::Leaf(data) = &self.root {
            let mut trees = vec![
                QuadTree::new(ne),
                QuadTree::new(nw),
                QuadTree::new(se),
                QuadTree::new(sw),
            ];

            for tree in &mut trees {
                for point in data {
                    tree.insert(point.0, point.1);
                }
            }

            self.root = QuadTreeData::Node(trees)
        }
    }

    pub fn remove(&mut self, point: Vec2) {
        if self.boundary.contains(point) {
            match &mut self.root {
                QuadTreeData::Leaf(ref mut data) => {
                    let mut to_remove = Vec::new();
                    for i in 0..data.len() {
                        if data[i].0 == point {
                            to_remove.push(i);
                        }
                    }
                    for i in to_remove {
                        data.remove(i);
                    }
                    if data.is_empty() {
                        self.root = QuadTreeData::Empty
                    }
                }
                QuadTreeData::Node(ref mut nodes) => {
                    nodes[0].remove(point);
                    nodes[1].remove(point);
                    nodes[2].remove(point);
                    nodes[3].remove(point);
                }
                QuadTreeData::Empty => {}
            }
        }
    }

    pub fn query_range(&self, range: &AABB) -> Vec<(Vec2, usize)> {
        let mut found = Vec::new();

        if self.boundary.intersects(&range) {
            match &self.root {
                QuadTreeData::Leaf(data) => {
                    found.extend(data.iter().filter(|(p, _)| range.contains(*p)));
                }
                QuadTreeData::Node(nodes) => {
                    found.extend(nodes[0].query_range(&range));
                    found.extend(nodes[1].query_range(&range));
                    found.extend(nodes[2].query_range(&range));
                    found.extend(nodes[3].query_range(&range));
                }
                QuadTreeData::Empty => {}
            }
        }

        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_quadtree() {
        let tree = QuadTree::new(AABB::new(Vec2::new(0.0, 0.0), Vec2::new(0.5, 0.5)));
        assert_eq!(tree.root, QuadTreeData::Empty);
    }
    #[test]
    fn add_element() {
        let mut tree = QuadTree::new(AABB::new(Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0)));
        // println!("{:?}", tree);
        tree.insert(Vec2::new(42.0, 0.0), 0);
        // println!("{:?}", tree);
        if let QuadTreeData::Leaf(data) = tree.root {
            assert_eq!(data.len(), 1);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn four_element() {
        let mut tree = QuadTree::new(AABB::new(Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0)));
        tree.insert(Vec2::new(42.0, 0.0), 0);
        tree.insert(Vec2::new(43.0, 0.0), 1);
        tree.insert(Vec2::new(44.0, 0.0), 2);
        tree.insert(Vec2::new(45.0, 0.0), 3);
        if let QuadTreeData::Leaf(data) = tree.root {
            assert_eq!(data.len(), 4);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn five_element() {
        let mut tree = QuadTree::new(AABB::new(Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0)));
        tree.insert(Vec2::new(-20.0, 10.0), 0);
        tree.insert(Vec2::new(20.0, 10.0), 1);
        tree.insert(Vec2::new(-20.0, -10.0), 2);
        tree.insert(Vec2::new(20.0, -10.0), 3);
        if let QuadTreeData::Leaf(data) = &tree.root {
            assert_eq!(data.len(), 4);
        } else {
            unreachable!()
        }

        tree.insert(Vec2::new(22.0, 10.0), 4);
        println!("{:#?}", tree);
        if let QuadTreeData::Node(data) = &tree.root {
            if let QuadTreeData::Leaf(data) = &data[0].root {
                assert_eq!(data.len(), 2);
            } else {
                unreachable!();
            }
            if let QuadTreeData::Leaf(data) = &data[1].root {
                assert_eq!(data.len(), 1);
            } else {
                unreachable!();
            }
            if let QuadTreeData::Leaf(data) = &data[2].root {
                assert_eq!(data.len(), 1);
            } else {
                unreachable!();
            }
            if let QuadTreeData::Leaf(data) = &data[3].root {
                assert_eq!(data.len(), 1);
            } else {
                unreachable!();
            }
        } else {
            unreachable!()
        }
    }

    #[test]
    fn remove_element() {
        let mut tree = QuadTree::new(AABB::new(Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0)));
        tree.insert(Vec2::new(-20.0, 10.0), 0);
        tree.insert(Vec2::new(20.0, 10.0), 1);
        tree.insert(Vec2::new(-20.0, -10.0), 2);
        tree.insert(Vec2::new(20.0, -10.0), 3);
        if let QuadTreeData::Leaf(data) = &tree.root {
            assert_eq!(data.len(), 4);
        } else {
            unreachable!()
        }

        tree.insert(Vec2::new(22.0, 10.0), 4);

        tree.remove(Vec2::new(20.0, 10.0));

        // println!("{:#?}", tree);
        if let QuadTreeData::Node(data) = &tree.root {
            if let QuadTreeData::Leaf(data) = &data[0].root {
                assert_eq!(data.len(), 1);
            } else {
                unreachable!();
            }
            if let QuadTreeData::Leaf(data) = &data[1].root {
                assert_eq!(data.len(), 1);
            } else {
                unreachable!();
            }
            if let QuadTreeData::Leaf(data) = &data[2].root {
                assert_eq!(data.len(), 1);
            } else {
                unreachable!();
            }
            if let QuadTreeData::Leaf(data) = &data[3].root {
                assert_eq!(data.len(), 1);
            } else {
                unreachable!();
            }
        } else {
            unreachable!()
        }

        tree.remove(Vec2::new(-20.0, -10.0));

        if let QuadTreeData::Node(data) = &tree.root {
            if let QuadTreeData::Leaf(data) = &data[0].root {
                assert_eq!(data.len(), 1);
            } else {
                unreachable!();
            }
            if let QuadTreeData::Empty = &data[1].root {
                
            } else {
                unreachable!();
            }
            if let QuadTreeData::Leaf(data) = &data[2].root {
                assert_eq!(data.len(), 1);
            } else {
                unreachable!();
            }
            if let QuadTreeData::Leaf(data) = &data[3].root {
                assert_eq!(data.len(), 1);
            } else {
                unreachable!();
            }
        } else {
            unreachable!()
        }
    }

    #[test]
    fn query_elements() {
        let mut tree = QuadTree::new(AABB::new(Vec2::new(0.0, 0.0), Vec2::new(50.0, 50.0)));
        tree.insert(Vec2::new(-20.0, 10.0), 0);
        tree.insert(Vec2::new(20.0, 10.0), 1);
        tree.insert(Vec2::new(-20.0, -10.0), 2);
        tree.insert(Vec2::new(20.0, -10.0), 3);
        tree.insert(Vec2::new(22.0, 10.0), 4);

        let res = tree.query_range(&AABB::new(Vec2::new(-20.0, 10.0), Vec2::new(1.0, 1.0)));
        assert_eq!(res.len(), 1);
        assert_eq!(res[0], (Vec2::new(-20.0, 10.0), 0));

        // println!("{:#?}", tree);

        let res = tree.query_range(&AABB::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0)));
        assert_eq!(res.len(), 5);
    }
}
