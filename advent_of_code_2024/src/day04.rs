use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Direction {
    HorizontalForward,
    HorizontalBackward,
    VerticalForward,
    VerticalBackward,
    DiagonalForward1,
    DiagonalForward2,
    DiagonalBackward1,
    DiagonalBackward2,
}

impl Direction {
    fn get_offsets(&self,rows: i32) -> (i32, i32, i32, i32) {
        match self {
            Direction::HorizontalForward => (0, 1, 2, 3),
            Direction::HorizontalBackward => (0, -1, -2, -3),
            Direction::VerticalForward => (0, 1*rows as i32, 2*rows as i32, 3*rows as i32),
            Direction::VerticalBackward => (0, -1*rows as i32, -2*rows as i32, -3*rows as i32),

            Direction::DiagonalForward1 => (0, 1*rows as i32+1, 2*rows as i32+2, 3*rows as i32+3),
            Direction::DiagonalForward2 => (0, -1*rows as i32+1, -2*rows as i32+2, -3*rows as i32+3),

            Direction::DiagonalBackward1 => (0, -1*rows as i32-1, -2*rows as i32-2, -3*rows as i32-3),
            Direction::DiagonalBackward2 => (0, 1*rows as i32-1, 2*rows as i32-2, 3*rows as i32-3),
        }
    }

    fn iter() -> impl Iterator<Item = Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::HorizontalForward,
            Direction::HorizontalBackward,
            Direction::VerticalForward,
            Direction::VerticalBackward,
            Direction::DiagonalForward1,
            Direction::DiagonalForward2,
            Direction::DiagonalBackward1,
            Direction::DiagonalBackward2,
        ];
        DIRECTIONS.iter().cloned()
    }
}

pub fn solve() {
    part1();
    part2();
}

fn part2() {
    let matrix = read_input("inputs/day04.txt");
    let cols = matrix.len() as i32;
    let rows = matrix[0].len() as i32;
    let flat: Vec<char> = matrix.into_iter().flatten().collect();
    let matrix = Matrix::new(flat, rows as usize, cols as usize);
    let count = traverse_matrix_3x3_flat(&matrix);
    println!("Part 2: {}", count);
}
struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Matrix<T> {
    fn new(data: Vec<T>, rows: usize, cols: usize) -> Self {
        assert_eq!(data.len(), rows * cols, "Data length must match dimensions");
        Matrix { data, rows, cols }
    }

    fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[row * self.cols + col])
        } else {
            None
        }
    }
}

fn traverse_matrix_3x3_flat(matrix: &Matrix<char>) -> i32 {
    let mut total_count = 0;
    
    for i in 0..matrix.rows.saturating_sub(2) {
        for j in 0..matrix.cols.saturating_sub(2) {
            let mut count = 0;
            if let (Some(&a), Some(&b), Some(&c)) = (
                matrix.get(i, j),
                matrix.get(i+1, j+1),
                matrix.get(i+2, j+2)
            ) {
                if (a == 'M' && b == 'A' && c == 'S') || (a == 'S' && b == 'A' && c == 'M') {
                    count += 1;
                }
            }
            
            if let (Some(&a), Some(&b), Some(&c)) = (
                matrix.get(i, j+2),
                matrix.get(i+1, j+1),
                matrix.get(i+2, j)
            ) {
                if (a == 'M' && b == 'A' && c == 'S') || (a == 'S' && b == 'A' && c == 'M') {
                    count += 1;
                }
            }
            if count == 2 {
                total_count += count / 2;
            }
        }
    }
    
    total_count
}
fn part1() {
    let matrix = read_input("inputs/day04.txt");
    let cols = matrix.len() as i32;
    let rows = matrix[0].len() as i32;
    let flat: Vec<char> = matrix.into_iter().flatten().collect();
    let mut seen_positions: HashSet<Vec<i32>> = HashSet::new();

    for i in 0..rows * cols {
        for direction in Direction::iter() {
            let (o1, o2, o3, o4) = direction.get_offsets(rows);
            if !check_in_bounds(i, cols, rows, o1, o2, o3, o4) {
                continue;
            }

            let mut positions = vec![i+o1, i+o2, i+o3, i+o4];
            positions.sort();
            if seen_positions.contains(&positions) {
                continue;
            }
            // println!("For index {:?} valid direction: {:?}", i, direction);
            let a = flat[(i+o1) as usize];
            let b = flat[(i+o2) as usize];
            let c = flat[(i+o3) as usize];
            let d = flat[(i+o4) as usize];
            if (a == 'X' && b == 'M' && c == 'A' && d == 'S') || (a == 'S' && b == 'A' && c == 'M' && d == 'X') {
                seen_positions.insert(positions);
                // println!("{}{}{}{}", a, b, c, d);
            }
        }
    }
    println!("Part 1: {}", seen_positions.len());

}

fn check_in_bounds(i: i32, cols: i32, rows: i32, o1: i32, o2: i32, o3: i32, o4: i32) -> bool {
    let positions = [i+o1, i+o2, i+o3, i+o4];
    
    // Check if any position is outside the total array bounds
    if positions.iter().any(|&p| p < 0 || p >= cols*rows) {
        return false;
    }

    let start_row = i / rows;
    let start_col = i % rows;
    
    // Get row and column for each position
    let pos_rows: Vec<i32> = positions.iter().map(|&p| p / rows).collect();
    let pos_cols: Vec<i32> = positions.iter().map(|&p| p % rows).collect();

    // For horizontal moves (offsets are small numbers like 0,1,2,3 or 0,-1,-2,-3)
    if o1.abs() < rows && o2.abs() < rows && o3.abs() < rows && o4.abs() < rows {
        // All positions must be in the same row
        return pos_rows.iter().all(|&row| row == start_row);
    }
    
    // For vertical moves (offsets are multiples of rows)
    if o1 % rows == 0 && o2 % rows == 0 && o3 % rows == 0 && o4 % rows == 0 {
        // All positions must be in the same column
        return pos_cols.iter().all(|&col| col == start_col);
    }
    
    // For diagonal moves
    // Check that both row and column changes are consistent
    // and don't go out of bounds
    for col in &pos_cols {
        if *col < 0 || *col >= rows {
            return false;
        }
    }
    
    // Check that row changes match column changes for diagonal consistency
    let row_diffs: Vec<i32> = pos_rows.iter().map(|&r| r - start_row).collect();
    let col_diffs: Vec<i32> = pos_cols.iter().map(|&c| c - start_col).collect();
    
    // For diagonal moves, the absolute changes in rows and columns should be equal
    row_diffs.iter().zip(col_diffs.iter())
        .all(|(&dr, &dc)| dr.abs() == dc.abs())
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}