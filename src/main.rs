use std::fmt;
use std::cmp;

const BOARD_SIZE: usize = 6;

const ALL_POINTS: usize = BOARD_SIZE.pow(3);

const POINT_ORDER_SIZE: usize = (BOARD_SIZE + 1).pow(3) * 8;

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
    neighbours: [[[usize; BOARD_SIZE]; BOARD_SIZE]; BOARD_SIZE],
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
        let mut board = Board {
            points: [[[false; BOARD_SIZE]; BOARD_SIZE]; BOARD_SIZE],
            neighbours: [[[6; BOARD_SIZE]; BOARD_SIZE]; BOARD_SIZE],
        };

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                board.neighbours[0][x][y] -= 1;
                board.neighbours[x][0][y] -= 1;
                board.neighbours[x][y][0] -= 1;

                board.neighbours[BOARD_SIZE - 1][x][y] -= 1;
                board.neighbours[x][BOARD_SIZE - 1][y] -= 1;
                board.neighbours[x][y][BOARD_SIZE - 1] -= 1;
            }
        }

        board
    }

    fn full_layers(&self) -> usize {
        for z in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                for x in 0..BOARD_SIZE {
                    if !self.points[x][y][z] {
                        return z;
                    }
                }
            }
        }
        return BOARD_SIZE;
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

    fn update(&mut self, point: &Point, new_val: bool) {
        self.points[point.x as usize][point.y as usize][point.z as usize] = new_val;
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

        let offsets = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];
        for point in points {
            self.occupy(point);
            for offset in offsets {
                let offset_x = point.x as i32 + offset.0;
                let offset_y = point.y as i32 + offset.1;
                let offset_z = point.z as i32 + offset.2;
                if cmp::max(cmp::max(offset_x, offset_y), cmp::max(offset_z, 0)) < (BOARD_SIZE as i32) &&
                    cmp::min(cmp::min(offset_x, offset_y), cmp::min(offset_z, 0)) > -1 {
//                                println!("+block: {:?}", block);
                    //                           println!("{} {} {}", offset_x as usize, offset_y as usize, offset_z as usize);
                    if offset_x == 1 && offset_y == 0 && offset_z == 0 {
//                                println!("board: {:?}", self);
//                                println!("+block: {:?}", block);
//                                println!("point: {:?}", point);
//                            println!("{} {} {}", offset_x as usize, offset_y as usize, offset_z as usize);
//                            println!("{:?}", self.neighbours[offset_x as usize][offset_y as usize][offset_z as usize]);
                    }
                    self.neighbours[offset_x as usize][offset_y as usize][offset_z as usize] -= 1;
                }
            }
        }

        return true;
    }

    fn take(&mut self, block: &BlockPosition) {
        let offsets = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];
        for point in &block.points {
            self.release(point);
            for offset in offsets {
                let offset_x = point.x as i32 + offset.0;
                let offset_y = point.y as i32 + offset.1;
                let offset_z = point.z as i32 + offset.2;
                if cmp::max(cmp::max(offset_x, offset_y), cmp::max(offset_z, 0)) < (BOARD_SIZE as i32) &&
                    cmp::min(cmp::min(offset_x, offset_y), cmp::min(offset_z, 0)) > -1 {
                    if offset_x == 1 && offset_y == 0 && offset_z == 0 {
//                                println!("-block: {:?}", block);
//                            println!("{} {} {}", offset_x as usize, offset_y as usize, offset_z as usize);
//                            println!("{:?}", self.neighbours[offset_x as usize][offset_y as usize][offset_z as usize]);
                    }
                    self.neighbours[offset_x as usize][offset_y as usize][offset_z as usize] += 1;
                }
            }
        }
    }

    fn has_no_holes(&self) -> bool {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                for z in 0..BOARD_SIZE {
                    if !self.points[x][y][z] {
                        if self.neighbours[x][y][z] == 0 {
                            return false;
                        }
                        if self.neighbours[x][y][z] == 1 {
                            let offsets = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];
                            for offset in offsets {
                                let offset_x = x as i32 + offset.0;
                                let offset_y = y as i32 + offset.1;
                                let offset_z = z as i32 + offset.2;
                                if cmp::max(cmp::max(offset_x, offset_y), cmp::max(offset_z, 0)) < (BOARD_SIZE as i32) &&
                                    cmp::min(cmp::min(offset_x, offset_y), cmp::min(offset_z, 0)) > -1 &&
                                    !self.points[offset_x as usize][offset_y as usize][offset_z as usize] &&
                                    self.neighbours[offset_x as usize][offset_y as usize][offset_z as usize] < 3 {
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }

        return true;
    }

    fn fill_next(&mut self, block_types: &BlockTypes, next_block_type_index: usize, start_point_index: usize, point_order: &[Point; POINT_ORDER_SIZE]) -> bool {
        let starting_point: &Point = &point_order[start_point_index];
        let block = &block_types[next_block_type_index].create_block_position(starting_point);
        // println!("check points (type: {}): {:?}", next_block_type_index, block);
        if self.full_layers() > 4 {
            println!("current: {:?}", self);
        }

        if !self.put(block) { return false; };

        if self.full() {
            println!("solution: {:?}", block);
            return true;
        };

        //println!("{}", start_point_index);
        if self.has_no_holes() {
            for k in (start_point_index + 1)..POINT_ORDER_SIZE {
                let next_point: &Point = &point_order[k];
                if next_point.z - 1 > self.full_layers() as i32 {
                    return false;
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
        }

        self.take(block);
//        println!("removed points: {:?}", points);

        //let fullAfterNext = self.fill_next(blockTypes, 
        return false;
    }

    fn full(&self) -> bool {
        for array2d in self.points {
            for array in array2d {
                for point_busy in array {
                    if !point_busy { return false; }
                }
            }
        }
        return true;
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

