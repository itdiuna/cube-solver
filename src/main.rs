use std::fmt;
use std::cmp;

const BOARD_SIZE: usize = 6;

const BLOCK_SIZE: usize = 4;

const DENSITY_FACTOR: f32 = 4 as f32 / 27 as f32;

const NUM_OF_BLOCK_TYPES: usize = 12;

type BoardType = [[[bool; BOARD_SIZE]; BOARD_SIZE]; BOARD_SIZE];

type BlockTypes = [BlockType; NUM_OF_BLOCK_TYPES];

fn main() {
    println!("Hello, world!");

    let mut board: Board = Board::create_empty_board();

    let block_types: BlockTypes = [
        BlockType {
            p1: build_point(0, 0, 0),
            p2: build_point(1, 0, 0),
            p3: build_point(1, 1, 0),
            p4: build_point(2, 0, 0)
        },
        BlockType {
            p1: build_point(0, 1, 0),
            p2: build_point(1, 1, 0),
            p3: build_point(1, 0, 0),
            p4: build_point(1, 2, 0)
        },
        BlockType {
            p1: build_point(0, 1, 0),
            p2: build_point(1, 0, 0),
            p3: build_point(1, 1, 0),
            p4: build_point(2, 1, 0)
        },
        BlockType {
            p1: build_point(0, 0, 0),
            p2: build_point(0, 1, 0),
            p3: build_point(0, 2, 0),
            p4: build_point(1, 1, 0)
        },
        BlockType {
            p1: build_point(0, 0, 0),
            p2: build_point(1, 0, 0),
            p3: build_point(1, 0, 1),
            p4: build_point(2, 0, 0)
        },
        BlockType {
            p1: build_point(0, 0, 1),
            p2: build_point(1, 0, 1),
            p3: build_point(1, 0, 0),
            p4: build_point(1, 0, 2)
        },
        BlockType {
            p1: build_point(0, 0, 1),
            p2: build_point(1, 0, 0),
            p3: build_point(1, 0, 1),
            p4: build_point(2, 0, 1)
        },
        BlockType {
            p1: build_point(0, 0, 0),
            p2: build_point(0, 0, 1),
            p3: build_point(0, 0, 2),
            p4: build_point(1, 0, 1)
        },
        BlockType {
            p1: build_point(0, 0, 0),
            p2: build_point(0, 0, 1),
            p3: build_point(0, 1, 1),
            p4: build_point(0, 0, 2)
        },
        BlockType {
            p1: build_point(0, 1, 0),
            p2: build_point(0, 1, 1),
            p3: build_point(0, 0, 1),
            p4: build_point(0, 2, 1)
        },
        BlockType {
            p1: build_point(0, 1, 0),
            p2: build_point(0, 0, 1),
            p3: build_point(0, 1, 1),
            p4: build_point(0, 1, 2)
        },
        BlockType {
            p1: build_point(0, 0, 0),
            p2: build_point(0, 1, 0),
            p3: build_point(0, 2, 0),
            p4: build_point(0, 1, 1)
        }
    ];

    board.fill_with(block_types);
    //let blockTypes: [dyn BlockPosition; 3] = [BlockX{}, BlockY{}, BlockZ{}];
//    let blockTypes = [impl Block { }, impl Block { } ];
}

struct Board {
    points: BoardType
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut heights = [[0; BOARD_SIZE]; BOARD_SIZE];
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                for z in 0..BOARD_SIZE {
                    if self.points[x][y][z] {
                        heights[y][z] = x+1;
                    }
                }
            }
        }
        write!(f, "\n---");
        for x in 0..BOARD_SIZE {
            write!(f, "\n{:?}", heights[x]);
        }
        write!(f, "\n---")
    }
}

impl Board {
    fn create_empty_board() -> Board {
        Board {
            points: [[[false; BOARD_SIZE]; BOARD_SIZE]; BOARD_SIZE]
        }
    }

    fn fill_with(&mut self, block_types: BlockTypes) {
        self.fill_next(&block_types, 0, &build_point(0, 0, 0));
    }

    fn fill_next(&mut self, block_types: &BlockTypes, next_block_type_index: usize, next_point_start: &Point) -> bool {
        let points = &block_types[next_block_type_index].create_block_position(next_point_start).points;
//        println!("check points: {:?}", points);
        for point in points {
            if point.x == BOARD_SIZE || point.y == BOARD_SIZE || point.z == BOARD_SIZE || self.points[point.x][point.y][point.z] { return false; }
        }

        for point in points {
            self.points[point.x][point.y][point.z] = true;
        }

        if self.full() { 
            println!("points: {:?}", points);
            return true;
        };

        if self.dense() {
            for x in 0..BOARD_SIZE {
                for y in 0..BOARD_SIZE {
                    for z in 0..BOARD_SIZE {
                        for block_type in 0..NUM_OF_BLOCK_TYPES {
                            if self.fill_next(&block_types, block_type, &build_point(x, y, z)) {
                                println!("points: {:?}", points);
                                return true;
                            }
                        }
                    }
                }
            }
        }

        for point in points {
            self.points[point.x][point.y][point.z] = false;
        }
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

    fn dense(&self) -> bool {
        let mut busy_count: i32 = 0;
        let mut spread: usize = 0;

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                for z in 0..BOARD_SIZE {
                    if self.points[x][y][z] {
                        busy_count += 1;
                        spread = cmp::max(cmp::max(spread, x), cmp::max(y, z));
                    }
                }
            }
        }

       // println!("busy_count: {}, spread: {}, board: {:?}", busy_count, spread, &self);
        return busy_count >= ((spread+1).pow(3) as f32 * DENSITY_FACTOR) as i32;
    }
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    z: usize
}

fn build_point(x: usize, y: usize, z: usize) -> Point {
    Point {
        x,
        y,
        z
    }
}

struct BlockPosition {
    points: [Point; BLOCK_SIZE]
}

struct BlockType {
    p1: Point,
    p2: Point,
    p3: Point,
    p4: Point
}

impl BlockType {
    fn create_block_position(&self, start: &Point) -> BlockPosition {
        BlockPosition {
            points: [
                build_point(self.p1.x+start.x, self.p1.y+start.y, self.p1.z+start.z),
                build_point(self.p2.x+start.x, self.p2.y+start.y, self.p2.z+start.z),
                build_point(self.p3.x+start.x, self.p3.y+start.y, self.p3.z+start.z),
                build_point(self.p4.x+start.x, self.p4.y+start.y, self.p4.z+start.z)
            ]
        }
    }
}

