#![feature(drain_filter)]


#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let size: usize = input.iter().map(|v| v.len()).sum();
    if size  == 0 {
        return vec![];
    }

    saddle_points(row_maxs(input), col_mins(input))
}

fn row_maxs(input: &[Vec<u64>]) -> Vec<Point> {
    (0..input.len()).map(|row_number| {
        let row_max = input[row_number].iter().max().unwrap();
        input[row_number].iter().enumerate().filter(|(_, val)| val == &row_max).map(|(i, _)| Point {x: row_number, y: i}).collect()
    }).flat_map(|v: Vec<Point>| v).collect()
}


fn col_mins(input: &[Vec<u64>]) -> Vec<Point> {
    (0..input[0].len()).map(|col_number| {
        let col_min = (0..input.len()).map(|i| input[i][col_number]).min().unwrap();
        (0..input.len()).map(|i| input[i][col_number]).enumerate().filter(|(_, val)| val == &col_min).map(|(i, _)| Point {x: i, y: col_number}).collect()
    }).flat_map(|v: Vec<Point>| v).collect()
    
}

fn saddle_points(row_points: Vec<Point>, column_points: Vec<Point>) -> Vec<(usize, usize)> {
    row_points.iter().filter(|p|  column_points.contains(&p)).map(|p| (p.x, p.y)).collect()
}