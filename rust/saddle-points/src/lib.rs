pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let x_len = input.len();
    let y_len = if x_len == 0 { 0 } else { input[0].len() };

    let mut saddle_points: Vec<(usize, usize)> = Vec::new();
    for i in 0..x_len {
        for j in 0..y_len {
            if is_saddle_point(input, (i,j)) {
                saddle_points.push((i,j));
            }
        }
    }
    saddle_points
}

pub fn is_saddle_point(input: &[Vec<u64>], coord: (usize, usize)) -> bool {
    let (i, j) = coord;
    let value = input[i][j];
    let row = input[i].iter();
    let column = (0..input.len()).map(|idx| input[idx][j]);
    row.max() == Some(&value) && column.min() == Some(value)
}
