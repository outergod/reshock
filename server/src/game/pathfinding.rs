use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use glam::{ivec2, IVec2};

use super::resource::Path;

const SEARCH_LIMIT: u16 = u16::MAX;

#[derive(Debug, Clone)]
struct Node {
    index: IVec2,
    g: f32,
    f: f32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.index.x == other.index.x && self.index.y == other.index.y
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.partial_cmp(&self.f).unwrap()
    }
}

fn euclidian_distance(a: &IVec2, b: &IVec2) -> f32 {
    let x = (b.x - a.x) as f32;
    let y = (b.y - a.y) as f32;

    (x.powf(2.0) + y.powf(2.0)).sqrt() as f32
}

pub struct AStar {
    obstacles: HashSet<IVec2>,
    moves: HashMap<IVec2, f32>,
}

impl AStar {
    pub fn new(obstacles: HashSet<IVec2>) -> Self {
        Self {
            obstacles,
            moves: Self::moves(),
        }
    }

    fn moves() -> HashMap<IVec2, f32> {
        (-1..=1)
            .flat_map(|x: i32| {
                (-1..=1).filter_map(move |y: i32| {
                    if x == 0 && y == 0 {
                        None
                    } else {
                        let cost = if x.abs() + y.abs() > 1 { 1.5 } else { 1.0 };

                        Some((ivec2(x, y), cost))
                    }
                })
            })
            .collect()
    }

    fn h(a: &IVec2, b: &IVec2) -> f32 {
        euclidian_distance(a, b)
    }

    fn recreate_path(parents: HashMap<IVec2, Node>, goal: IVec2) -> Path {
        let mut path = Path::new();

        let mut index = goal;

        path.push(index);

        while let Some(parent) = parents.get(&index) {
            path.push(parent.index);
            index = parent.index;
        }
        path.reverse();

        path
    }

    fn neighbors(&self, closed: &HashMap<IVec2, f32>, goal: &IVec2, node: &Node) -> Vec<Node> {
        self.moves
            .iter()
            .filter_map(|(delta, weight)| {
                let index = node.index + *delta;
                let g = node.g + *weight;
                let closed_better = closed.get(&index).is_some_and(|closed_g| **closed_g <= g);

                if (&index != goal && self.obstacles.contains(&index)) || closed_better {
                    None
                } else {
                    Some(Node {
                        index,
                        g,
                        f: g + Self::h(&index, &goal),
                    })
                }
            })
            .collect()
    }

    pub fn search(&self, start: IVec2, goal: IVec2) -> Option<Path> {
        let mut fringe: BinaryHeap<Node> = BinaryHeap::new();
        let mut closed: HashMap<IVec2, f32> = HashMap::new();
        let mut parents: HashMap<IVec2, Node> = HashMap::new();

        let mut obstacles = self.obstacles.clone();
        obstacles.remove(&goal);

        fringe.push(Node {
            index: start,
            g: 0.0,
            f: Self::h(&start, &goal),
        });
        closed.insert(start, 0.0);

        let mut steps = 0;

        while steps < SEARCH_LIMIT && let Some(node) = fringe.pop() {
            steps += 1;

            if node.index == goal {
                return Some(Self::recreate_path(parents, goal));
            }

            for neighbor in self.neighbors(&closed, &goal, &node) {
                fringe.push(neighbor.clone());
                closed.insert(neighbor.index, neighbor.g);
                parents.insert(neighbor.index, node.clone());
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use glam::{ivec2, IVec2};

    use crate::game::resource::Room;

    use super::{AStar, SEARCH_LIMIT};

    fn extract_room(room: String) -> (HashSet<IVec2>, IVec2, IVec2) {
        let room: Room = room.into();

        let obstacles = room
            .0
            .iter()
            .filter_map(|(pos, c)| (*c == '#').then_some(*pos))
            .collect();

        let start = room
            .0
            .iter()
            .find_map(|(pos, c)| (*c == '@').then_some(*pos))
            .unwrap();

        let goal = room
            .0
            .iter()
            .find_map(|(pos, c)| (*c == 'X').then_some(*pos))
            .unwrap();

        (obstacles, start, goal)
    }

    #[test]
    fn test_id() {
        let astar = AStar::new(Default::default());
        assert_eq!(
            astar.search((0, 0).into(), (0, 0).into()),
            Some(vec![(0, 0).into()])
        );
    }

    #[test]
    fn test_backup() {
        let room = "
####
  @#X
 ###"
        .to_string();

        let (obstacles, start, goal) = extract_room(room);

        let astar = AStar::new(obstacles);
        assert_eq!(
            astar.search(start, goal),
            Some(vec![
                ivec2(2, 1),
                ivec2(1, 1),
                ivec2(0, 0),
                ivec2(1, -1),
                ivec2(2, -1),
                ivec2(3, -1),
                ivec2(4, 0),
                ivec2(4, 1),
            ])
        );
    }

    #[test]
    fn test_simple() {
        let room = "
###
#@ X
###"
        .to_string();

        let (obstacles, start, goal) = extract_room(room);

        let astar = AStar::new(obstacles);
        assert_eq!(
            astar.search(start, goal),
            Some(vec![ivec2(1, 1), ivec2(2, 1), ivec2(3, 1)])
        );
    }

    #[test]
    fn test_blocked() {
        let room = "
###
#@#X
###"
        .to_string();

        let (obstacles, start, goal) = extract_room(room);

        let astar = AStar::new(obstacles);
        assert_eq!(astar.search(start, goal), None);
    }

    #[test]
    fn test_too_far() {
        let astar = AStar::new(Default::default());
        assert_eq!(
            astar.search(ivec2(0, 0), ivec2(SEARCH_LIMIT as i32, 0)),
            None
        );
        assert_ne!(
            astar.search(ivec2(0, 0), ivec2((SEARCH_LIMIT - 1) as i32, 0)),
            None
        );
    }
}
