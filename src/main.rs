use std::fmt;
use std::cmp;

const BOARD_SIZE: usize = 4;

const ALL_POINTS: usize = BOARD_SIZE.pow(3);

const POINT_ORDER_SIZE: usize = BOARD_SIZE.pow(3);

const BLOCK_SIZE: usize = 4;

const NUM_OF_BLOCK_TYPES: usize = 12;

type BoardType = [[[bool; BOARD_SIZE]; BOARD_SIZE]; BOARD_SIZE];

type BlockTypes = [BlockType; NUM_OF_BLOCK_TYPES];

fn main() {
    println!("Hello, world!");

    let mut board: Board = Board::create_empty_board();

    let block_types: BlockTypes = [
        BlockType {
            p2: build_point(0, 0, 0),   // |0
            p1: build_point(0, 0, -1),  // |0_
            p3: build_point(0, -1, 0),  //  0
            p4: build_point(0, 1, 0),
        },
        BlockType {
            p1: build_point(0, 0, 0),   //  |
            p2: build_point(0, 0, -1),  // 0|00_
            p3: build_point(1, 0, 0),   //
            p4: build_point(-1, 0, 0),
        },
        BlockType {
            p3: build_point(0, 0, 0),   //  |0
            p1: build_point(-1, 0, 0),  // <|0_
            p2: build_point(0, 1, 0),   //   0
            p4: build_point(0, -1, 0),
        },
        BlockType {
            p2: build_point(0, 0, 0),  //  |
            p1: build_point(-1, 0, 0), // <|0_
            p3: build_point(0, 0, 1),  //
            p4: build_point(0, 0, -1),
        },
        BlockType {
            p3: build_point(0, 0, 0),  //  |
            p2: build_point(1, -1, 0), // 0|00_
            p1: build_point(-1, 0, 0), //   v
            p4: build_point(1, 0, 0),
        },
        BlockType {
            p3: build_point(0, 0, 0),  // |
            p2: build_point(0, -1, 0), // |0_
            p1: build_point(0, 0, 1),  //  v
            p4: build_point(0, 0, -1),
        },
        BlockType {
            p2: build_point(0, 0, 0),  // |0
            p4: build_point(0, 0, 1),  // |x_
            p1: build_point(0, -1, 0), //  0
            p3: build_point(0, 1, 0),
        },
        BlockType {
            p2: build_point(0, 0, 0),  //  |
            p3: build_point(0, 0, 1),  // 0|x0_
            p1: build_point(-1, 0, 0), //
            p4: build_point(1, 0, 0),
        },
        BlockType {
            p2: build_point(0, 0, 0),  // |
            p4: build_point(1, 0, 0),  // |0>_
            p1: build_point(0, 0, -1), //
            p3: build_point(0, 0, 1),
        },
        BlockType {
            p2: build_point(0, 0, 0),  // |0
            p4: build_point(1, 0, 0),  // |0>_
            p1: build_point(0, -1, 0), //  0
            p3: build_point(0, 1, 0),
        },
        BlockType {
            p2: build_point(0, 0, 0),  //  |^
            p3: build_point(0, 1, 0),  // 0|00_
            p1: build_point(-1, 0, 0), //
            p4: build_point(1, 0, 0),
        },
        BlockType {
            p2: build_point(0, 0, 0),  //  |^
            p3: build_point(0, 1, 0),  //  |0_
            p1: build_point(0, 0, -1), //
            p4: build_point(0, 0, 1),
        }
    ];

    let point_order: [Point; POINT_ORDER_SIZE] = generate_point_order();

    board.fill_with(block_types, &point_order);

    //let blockTypes: [dyn BlockPosition; 3] = [BlockX{}, BlockY{}, BlockZ{}];
//    let blockTypes = [impl Block { }, impl Block { } ];
}

struct Board {
    points: BoardType,
    layers_capacity: [i32; BOARD_SIZE],
    full_layers: usize
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut heights: [[i32; BOARD_SIZE]; BOARD_SIZE] = [[0; BOARD_SIZE]; BOARD_SIZE];
        let mut free = 0;
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                for z in 0..BOARD_SIZE {
                    if self.points[x][y][z] {
//                        println!("{} {} {}", x, y, z);
                        heights[y][x] = z as i32 + 1;
                    } else {
                        free += 1;
                    }
                }
            }
        }
        write!(f, "\n---");
        for x in 0..BOARD_SIZE {
            write!(f, "\n{:?}", heights[x]);
        }
        write!(f, "\n--- free: {:?} full layers: {}", free, self.full_layers())
    }
}

impl Board {
    fn create_empty_board() -> Board {
        Board {
            points: [[[false; BOARD_SIZE]; BOARD_SIZE]; BOARD_SIZE],
            layers_capacity: [BOARD_SIZE.pow(2) as i32; BOARD_SIZE],
            full_layers: 0
        }
    }

    fn full_layers(&self) -> usize {
        self.full_layers
    }

    fn top_free_only(&self) -> bool {
        let mut heights: [[i32; BOARD_SIZE]; BOARD_SIZE] = [[0; BOARD_SIZE]; BOARD_SIZE];
        let mut free = 0;
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                for z in 0..BOARD_SIZE {
                    if self.points[x][y][z] {
//                        println!("{} {} {}", x, y, z);
                        heights[y][x] = z as i32 + 1;
                    } else {
                        free += 1;
                    }
                }
            }
        }
        let mut free_from_the_top: i32 = 0;
        for x in 0..BOARD_SIZE {
            free_from_the_top += heights[x].iter().map(|v| { BOARD_SIZE as i32 - v }).sum::<i32>();
        }

        free == free_from_the_top
    }

    fn fill_with(&mut self, block_types: BlockTypes, point_order: &[Point; POINT_ORDER_SIZE]) {
        self.fill_next(&block_types, 7, 1, &point_order);
    }

    fn occupied(&self, point: &Point) -> bool {
        self.points[point.x as usize][point.y as usize][point.z as usize]
    }

    fn occupy(&mut self, point: &Point) {
        self.update(point, true);
    }

    fn release(&mut self, point: &Point) {
        self.update(point, false);
    }

    fn update(&mut self, point: &Point, occupied: bool) {
        self.points[point.x as usize][point.y as usize][point.z as usize] = occupied;
        self.layers_capacity[point.z as usize] += if occupied { -1 } else { 1 };
        if self.layers_capacity[point.z as usize] == 0 {
            self.full_layers += 1;
        } else if self.layers_capacity[point.z as usize] == 1 && !occupied {
            self.full_layers -= 1;
        }
    }

    fn put(&mut self, block: &BlockPosition) -> bool {
        let points = &block.points;
        for point in points {
            if cmp::max(cmp::max(0, point.x), cmp::max(point.y, point.z)) >= BOARD_SIZE as i32 ||
                cmp::min(cmp::min(0, point.x), cmp::min(point.y, point.z)) < 0 as i32 ||
                self.occupied(point) {
                return false;
            }
        }

        for point in points {
            self.occupy(point);
        }

        return true;
    }

    fn take(&mut self, block: &BlockPosition) {
        let offsets = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];
        for point in &block.points {
            self.release(point);
        }
    }

    fn fill_next(&mut self, block_types: &BlockTypes, next_block_type_index: usize, start_point_index: usize, point_order: &[Point; POINT_ORDER_SIZE]) -> bool {
        let starting_point: &Point = &point_order[start_point_index];
        let block = &block_types[next_block_type_index].create_block_position(starting_point);
        // println!("check points (type: {}): {:?}", next_block_type_index, block);
        if self.full_layers() > 0 {
            // println!("current: {:?}", self);
        }

        if !self.put(block) { return false; };

        if self.full() {
            println!("solution: {:?}", block);
            return true;
        };

        //println!("{}", start_point_index);
        for k in (start_point_index + 1)..POINT_ORDER_SIZE {
            let next_point: &Point = &point_order[k];
            let full_layers = self.full_layers();
            // println!("next point: {:?}", next_point);
            if (next_point.z > 1 + full_layers as i32) {
                break;
            }
            if !self.occupied(next_point) {
                for block_type in 0..NUM_OF_BLOCK_TYPES {
                    if self.fill_next(&block_types, block_type, k, &point_order) {
                        println!("solution: {:?}", block);
                        return true;
                    }
                }
            }
        }

        self.take(block);
//        println!("removed points: {:?}", points);

        //let fullAfterNext = self.fill_next(blockTypes, 
        return false;
    }

    fn full(&self) -> bool {
        self.full_layers == BOARD_SIZE
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn build_point(x: i32, y: i32, z: i32) -> Point {
    Point {
        x,
        y,
        z,
    }
}

#[derive(Debug)]
struct BlockPosition {
    points: [Point; BLOCK_SIZE],
}

struct BlockType {
    p1: Point,
    p2: Point,
    p3: Point,
    p4: Point,
}

impl BlockType {
    fn create_block_position(&self, start: &Point) -> BlockPosition {
        BlockPosition {
            points: [
                build_point(self.p1.x + start.x, self.p1.y + start.y, self.p1.z + start.z),
                build_point(self.p2.x + start.x, self.p2.y + start.y, self.p2.z + start.z),
                build_point(self.p3.x + start.x, self.p3.y + start.y, self.p3.z + start.z),
                build_point(self.p4.x + start.x, self.p4.y + start.y, self.p4.z + start.z)
            ]
        }
    }
}

fn generate_point_order<const LEN: usize>() -> [Point; LEN] {
    let init_point = build_point(0, 0, 0);

    let mut order: [Point; LEN] = [init_point; LEN];
    let mut k = 0;
    for z in 0..BOARD_SIZE {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                // let k = cantor_pairing(x, cantor_pairing(y, z));
//                    println!("{:?}: {:?} {:?} {:?}", k, x, y, z);
                order[k as usize] = build_point(x as i32, y as i32, z as i32);
                k += 1;
            }
        }
    }
    order
}

fn cantor_pairing(k1: usize, k2: usize) -> usize {
    (k1 + k2) * (k1 + k2 + 1) / 2 + k2
}

