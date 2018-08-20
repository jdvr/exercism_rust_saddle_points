pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut points_by_rows: Vec<(usize, usize)> = Vec::new();
    let mut points_by_columns: Vec<(usize, usize)> = Vec::new();
    let mut result: Vec<(usize, usize)> = Vec::new();


    for row_number in 0..input.len(){
        let mut row_max = input[row_number][0];
        let mut points_by_row: Vec<(usize, usize)> = Vec::new();
        for col_number in 0..input[row_number].len() {
            if row_max <= input[row_number][col_number] {
                points_by_row.push((row_number, col_number));
                row_max = input[row_number][col_number];
            }
        }
        points_by_rows.append(&mut points_by_row);
    }

    for col_number in 0..input[0].len(){
        let mut col_min = u64::max_value();
        let mut points_by_column: Vec<(usize, usize)> = Vec::new();
        for row_number in 0..input.len() {
            if col_min >= input[row_number][col_number] {
                points_by_column.push((row_number, col_number));
                col_min = input[row_number][col_number];
            }
        }
        points_by_columns.append(&mut points_by_column);
    }

    for point in points_by_rows {
        if points_by_columns.contains(&point) {
            result.push(point)
        }
    }
    result
}