// use super::*;
use bevy::prelude::Vec2;

#[derive(Debug, Default)]
pub struct AABB {
    pub center: Vec2,
    pub half_dimension: Vec2,
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

// #[derive(Debug, Default)]
// pub struct QuadTree {
//     points: Vec<(Vec2, usize)>,
//     boundary: AABB,
//     children: [Option<Box<QuadTree>>; 4],
// }

// impl QuadTree {
//     pub fn new(boundary: AABB) -> Self {
//         Self {
//             points: Vec::new(),
//             boundary,
//             children: [None, None, None, None],
//         }
//     }

//     pub fn insert(&mut self, position: Vec2, data: usize) -> bool {
//         if !self.boundary.contains(position) {
//             return false;
//         }

//         if self.children[0].is_none() && self.points.len() < 4 {
//             self.points.push((position, data));
//             return true;
//         }

//         if self.children[0].is_none() {
//             self.subdivide();
//         }

//         if self.children[0].as_mut().unwrap().insert(position, data) {
//             return true;
//         }

//         if self.children[1].as_mut().unwrap().insert(position, data) {
//             return true;
//         }

//         if self.children[2].as_mut().unwrap().insert(position, data) {
//             return true;
//         }

//         if self.children[3].as_mut().unwrap().insert(position, data) {
//             return true;
//         }

//         false
//     }

//     pub fn remove(&mut self, position: Vec2) {
//         if !self.boundary.contains(position) {
//             return;
//         }
//         if let Some(id) = self.points.iter().position(|&(pos, _)| pos == position) {
//             self.points.remove(id);
//         } else {
//             if self.children[0].is_none() {
//                 return;
//             }
//             self.children[0].as_mut().unwrap().remove(position);
//             self.children[1].as_mut().unwrap().remove(position);
//             self.children[2].as_mut().unwrap().remove(position);
//             self.children[3].as_mut().unwrap().remove(position);

//             if self.children[0].as_ref().unwrap().points.is_empty()
//                 && self.children[1].as_ref().unwrap().points.is_empty()
//                 && self.children[2].as_ref().unwrap().points.is_empty()
//                 && self.children[3].as_ref().unwrap().points.is_empty()
//             {
//                 self.children[0] = None;
//                 self.children[1] = None;
//                 self.children[2] = None;
//                 self.children[3] = None;
//             }
//         }
//     }

//     fn subdivide(&mut self) {
//         let half_half = self.boundary.half_dimension / 2.0;
//         let nw = AABB::new(
//             self.boundary.center - Vec2::new(-half_half, half_half),
//             half_half,
//         );
//         let ne = AABB::new(
//             self.boundary.center - Vec2::new(half_half, half_half),
//             half_half,
//         );
//         let sw = AABB::new(
//             self.boundary.center - Vec2::new(-half_half, -half_half),
//             half_half,
//         );
//         let se = AABB::new(
//             self.boundary.center - Vec2::new(half_half, -half_half),
//             half_half,
//         );

//         self.children = [
//             Some(Box::new(Self::new(nw))),
//             Some(Box::new(Self::new(ne))),
//             Some(Box::new(Self::new(sw))),
//             Some(Box::new(Self::new(se))),
//         ]
//     }

//     pub fn query_range(&self, range: &AABB) -> Vec<usize> {
//         if self.boundary.intersects(&range) {
//             let mut points_in_range = Vec::new();

//             for point in &self.points {
//                 if range.contains(point.0) {
//                     points_in_range.push(point.1);
//                 }
//             }

//             if self.children[0].is_some() {
//                 for i in 0..4 {
//                     points_in_range
//                         .extend(self.children[i].as_ref().unwrap().query_range(&range));
//                 }
//             }

//             points_in_range
//         } else {
//             Vec::new()
//         }
//     }
// }

// #[cfg(test)]
// mod quadtree_tests {

//     use super::*;

//     #[test]
//     fn test() {
//         let boundary = AABB::new(Vec2::zero(), 200.0);
//         let mut quadtree = QuadTree::new(boundary);

//         quadtree.insert(Vec2::new(10.0, 10.0), 42);

//         assert_eq!(
//             quadtree
//                 .query_range(&AABB::new(Vec2::new(0.0, 0.0), 20.0))
//                 .is_empty(),
//             false
//         );
//         assert_eq!(
//             quadtree
//                 .query_range(&AABB::new(Vec2::new(0.0, 0.0), 5.0))
//                 .is_empty(),
//             true
//         );
//     }

//     #[test]
//     fn test_remove() {
//         let boundary = AABB::new(Vec2::zero(), 200.0);
//         let mut quadtree = QuadTree::new(boundary);

//         quadtree.insert(Vec2::new(42.0, 10.0), 42);
//         quadtree.insert(Vec2::new(15.0, 10.0), 42);
//         quadtree.insert(Vec2::new(30.0, 10.0), 42);
//         quadtree.insert(Vec2::new(20.0, 10.0), 42);
//         quadtree.insert(Vec2::new(10.0, 10.0), 42);

//         println!("{:#?}", quadtree);

//         assert_eq!(
//             quadtree
//                 .query_range(&AABB::new(Vec2::new(0.0, 0.0), 20.0))
//                 .is_empty(),
//             false
//         );

//         quadtree.remove(Vec2::new(10.0, 10.0));
//         println!("Removed!");
//         println!("{:#?}", quadtree);

//         assert_eq!(
//             quadtree
//                 .query_range(&AABB::new(Vec2::new(10.0, 10.0), 1.0))
//                 .is_empty(),
//             true
//         );
//     }
// }

// Refs : https://en.wikipedia.org/wiki/Quadtree#Polygonal_map_quadtree


// // V2
// #[derive(Debug)]
// pub struct QuadTree {
//     pub(crate) boundary: AABB,
//     pub(crate) children: Option<Vec<QuadTree>>,
//     pub(crate) points: Vec<(Vec2, usize)>,
// }

// #[derive(Debug)]
// pub enum QuadTreeError {
//     OutOfBoundary,
// }

// impl QuadTree {
//     pub fn new(boundary: AABB) -> Self {
//         Self {
//             boundary,
//             children: None,
//             points: Vec::new(),
//         }
//     }

//     fn subdivide(&mut self) {
//         // let half_dimension = self.boundary.half_dimension;
//         let delta: Vec2 = self.boundary.half_dimension / 2.0;

//         let position = self.boundary.center;

//         let a = AABB::new(position + Vec2::new(-delta.x, delta.y), delta);
//         let b = AABB::new(position + Vec2::new(delta.x, delta.y), delta);
//         let c = AABB::new(position + Vec2::new(-delta.x, -delta.y), delta);
//         let d = AABB::new(position + Vec2::new(delta.x, -delta.y), delta);

//         self.children = Some(vec![
//             QuadTree::new(a),
//             QuadTree::new(b),
//             QuadTree::new(c),
//             QuadTree::new(d),
//         ]);
//     }

//     pub fn insert(&mut self, point: Vec2, data: usize) -> Result<(), QuadTreeError> {
//         if !self.boundary.contains(point) {
//             return Err(QuadTreeError::OutOfBoundary);
//         }

//         if self.points.len() < 4 {
//             self.points.push((point, data));

//             return Ok(());
//         }

//         if self.children.is_none() {
//             self.subdivide();
//         }

//         if let Ok(()) = self.children.as_mut().unwrap()[0].insert(point, data) {
//             return Ok(());
//         }

//         if let Ok(()) = self.children.as_mut().unwrap()[1].insert(point, data) {
//             return Ok(());
//         }

//         if let Ok(()) = self.children.as_mut().unwrap()[2].insert(point, data) {
//             return Ok(());
//         }

//         if let Ok(()) = self.children.as_mut().unwrap()[3].insert(point, data) {
//             return Ok(());
//         }

//         unreachable!()
//     }

//     pub fn remove(&mut self, position: Vec2) {
//         if !self.boundary.contains(position) {
//             return;
//         }
//         if let Some(id) = self.points.iter().position(|&(pos, _)| pos == position) {
//             self.points.remove(id);
//         } else {
//             if self.children.is_none() {
//                 return;
//             }

//             for child in self.children.as_mut().unwrap().iter_mut() {
//                 child.remove(position);
//             }

//             if self
//                 .children
//                 .as_mut()
//                 .unwrap()
//                 .iter()
//                 .all(|child| child.points.is_empty())
//             {
//                 self.children = None;
//             }
//         }
//     }

//     pub fn query_range(&self, range: &AABB) -> Vec<(Vec2, usize)> {
//         if self.boundary.intersects(&range) {
//             let mut points_in_range = Vec::new();

//             for point in &self.points {
//                 if range.contains(point.0) {
//                     points_in_range.push(*point);
//                 }
//             }

//             if self.children.is_some() {
//                 for child in self.children.as_ref().unwrap().iter() {
//                     points_in_range.extend(child.query_range(&range));
//                 }
//             }

//             points_in_range
//         } else {
//             Vec::new()
//         }
//     }
// }

#[derive(Debug)]
pub struct QuadTree {
    pub(crate) boundary: AABB,
    pub(crate) children: Option<Vec<QuadTree>>,
    pub(crate) points: Vec<(Vec2, usize)>,
}

#[derive(Debug)]
pub enum QuadTreeError {
    OutOfBoundary,
}

impl QuadTree {
    pub fn new(boundary: AABB) -> Self {
        Self {
            boundary,
            children: None,
            points: Vec::new(),
        }
    }

    fn subdivide(&mut self) {
        // let half_dimension = self.boundary.half_dimension;
        let delta: Vec2 = self.boundary.half_dimension / 2.0;

        let position = self.boundary.center;

        let a = AABB::new(position + Vec2::new(-delta.x, delta.y), delta);
        let b = AABB::new(position + Vec2::new(delta.x, delta.y), delta);
        let c = AABB::new(position + Vec2::new(-delta.x, -delta.y), delta);
        let d = AABB::new(position + Vec2::new(delta.x, -delta.y), delta);

        self.children = Some(vec![
            QuadTree::new(a),
            QuadTree::new(b),
            QuadTree::new(c),
            QuadTree::new(d),
        ]);
    }

    pub fn insert(&mut self, point: Vec2, data: usize) -> Result<(), QuadTreeError> {
        if !self.boundary.contains(point) {
            return Err(QuadTreeError::OutOfBoundary);
        }

        if self.points.len() < 4 {
            self.points.push((point, data));

            return Ok(());
        }

        if self.children.is_none() {
            self.subdivide();
        }

        if let Ok(()) = self.children.as_mut().unwrap()[0].insert(point, data) {
            return Ok(());
        }

        if let Ok(()) = self.children.as_mut().unwrap()[1].insert(point, data) {
            return Ok(());
        }

        if let Ok(()) = self.children.as_mut().unwrap()[2].insert(point, data) {
            return Ok(());
        }

        if let Ok(()) = self.children.as_mut().unwrap()[3].insert(point, data) {
            return Ok(());
        }

        unreachable!()
    }

    pub fn remove(&mut self, position: Vec2) {
        if !self.boundary.contains(position) {
            return;
        }
        if let Some(id) = self.points.iter().position(|&(pos, _)| pos == position) {
            self.points.remove(id);
        } else {
            if self.children.is_none() {
                return;
            }

            for child in self.children.as_mut().unwrap().iter_mut() {
                child.remove(position);
            }

            if self
                .children
                .as_mut()
                .unwrap()
                .iter()
                .all(|child| child.points.is_empty())
            {
                self.children = None;
            }
        }
    }

    pub fn query_range(&self, range: &AABB) -> Vec<(Vec2, usize)> {
        if self.boundary.intersects(&range) {
            let mut points_in_range = Vec::new();

            for point in &self.points {
                if range.contains(point.0) {
                    points_in_range.push(*point);
                }
            }

            if self.children.is_some() {
                for child in self.children.as_ref().unwrap().iter() {
                    points_in_range.extend(child.query_range(&range));
                }
            }

            points_in_range
        } else {
            Vec::new()
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn empty_quadtree_from_new() {
        let quadtree = QuadTree::new(AABB::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0)));
        assert_eq!(quadtree.children.is_none(), true);
        assert_eq!(quadtree.points.len(), 0);
    }

    #[test]
    fn insert_element() {
        let mut quadtree = QuadTree::new(AABB::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0)));
        let has_inserted = quadtree.insert(Vec2::new(0.0, 0.0), 42);
        assert_eq!(quadtree.children.is_none(), true);
        assert_eq!(quadtree.points.len(), 1);
        assert_eq!(has_inserted.is_ok(), true);
    }

    #[test]
    fn insert_several_element() {
        let mut quadtree = QuadTree::new(AABB::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0)));
        quadtree.insert(Vec2::new(0.0, 0.0), 42);
        quadtree.insert(Vec2::new(10.0, 0.0), 42);
        quadtree.insert(Vec2::new(20.0, 0.0), 42);
        quadtree.insert(Vec2::new(30.0, 0.0), 42);

        assert_eq!(quadtree.children.is_none(), true);
        assert_eq!(quadtree.points.len(), 4);

        quadtree.insert(Vec2::new(40.0, 0.0), 42);

        assert_eq!(quadtree.children.is_some(), true);
        assert_eq!(quadtree.points.len(), 4);

        assert_eq!(
            quadtree.children.as_ref().unwrap()[0].children.is_none(),
            true
        );
        assert_eq!(
            quadtree.children.as_ref().unwrap()[1].children.is_none(),
            true
        );
        assert_eq!(
            quadtree.children.as_ref().unwrap()[2].children.is_none(),
            true
        );
        assert_eq!(
            quadtree.children.as_ref().unwrap()[3].children.is_none(),
            true
        );

        assert_eq!(quadtree.children.as_ref().unwrap()[0].points.len(), 0);
        assert_eq!(quadtree.children.as_ref().unwrap()[1].points.len(), 1);
        assert_eq!(quadtree.children.as_ref().unwrap()[2].points.len(), 0);
        assert_eq!(quadtree.children.as_ref().unwrap()[3].points.len(), 0);
    }

    #[test]
    fn remove_item() {
        let mut quadtree = QuadTree::new(AABB::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0)));
        quadtree.insert(Vec2::new(0.0, 0.0), 42);

        assert_eq!(quadtree.children.is_none(), true);
        assert_eq!(quadtree.points.len(), 1);

        quadtree.remove(Vec2::new(0.0, 0.0));

        assert_eq!(quadtree.children.is_none(), true);
        assert_eq!(quadtree.points.len(), 0);
    }

    #[test]
    fn query_range_tests() {
        let mut quadtree = QuadTree::new(AABB::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0)));
        quadtree.insert(Vec2::new(0.0, 0.0), 42);

        assert_eq!(quadtree.children.is_none(), true);
        assert_eq!(quadtree.points.len(), 1);
        assert_eq!(
            quadtree.query_range(&AABB::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0))),
            vec![(Vec2::new(0.0, 0.0), 42)]
        );

        assert_eq!(
            quadtree.query_range(&AABB::new(Vec2::new(-10.0, 0.0), Vec2::new(1.0, 1.0))),
            vec![]
        );

        quadtree.remove(Vec2::new(0.0, 0.0));

        assert_eq!(quadtree.children.is_none(), true);
        assert_eq!(quadtree.points.len(), 0);

        assert_eq!(
            quadtree.query_range(&AABB::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0))),
            vec![]
        );
    }
}
