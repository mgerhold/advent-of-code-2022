fn is_visible(map: &Vec<Vec<u8>>, row: usize, column: usize) -> bool {
    let height = map[row][column];
    if (0..column)
        .rev()
        .map(|current_column| map[row][current_column])
        .all(|current_height| current_height < height)
    {
        return true;
    }
    if (column + 1..map[0].len())
        .map(|current_column| map[row][current_column])
        .all(|current_height| current_height < height)
    {
        return true;
    }
    if (row + 1..map.len())
        .map(|current_row| map[current_row][column])
        .all(|current_height| current_height < height)
    {
        return true;
    }
    (0..row)
        .rev()
        .map(|current_row| map[current_row][column])
        .all(|current_height| current_height < height)
}

fn calculate_scenic_score(map: &Vec<Vec<u8>>, row: usize, column: usize) -> usize {
    let map_height = map.len();
    let map_width = map[0].len();
    let height = map[row][column];
    if row == 0 || row == map_height - 1 || column == 0 || column == map_width - 1 {
        return 0;
    }
    let right_score = (column + 1..map_width - 1)
        .map(|current_column| map[row][current_column])
        .take_while(|current_height| *current_height < height)
        .count()
        + 1;
    let left_score = (1..column)
        .rev()
        .map(|current_column| map[row][current_column])
        .take_while(|current_height| *current_height < height)
        .count()
        + 1;
    let bottom_score = (row + 1..map_height - 1)
        .map(|current_row| map[current_row][column])
        .take_while(|current_height| *current_height < height)
        .count()
        + 1;
    let top_score = (1..row)
        .rev()
        .map(|current_row| map[current_row][column])
        .take_while(|current_height| *current_height < height)
        .count()
        + 1;
    right_score * left_score * bottom_score * top_score
}

fn main() {
    let input = std::fs::read_to_string("real_input.txt").unwrap();
    let mut map = Vec::new();
    for line in input.as_str().split('\n') {
        if line.trim().is_empty() {
            break;
        }
        let digits: Vec<_> = line.as_bytes().iter().map(|byte| *byte - b'0').collect();
        map.push(digits);
    }

    for row in &map {
        for height in row {
            print!("{}", height);
        }
        println!();
    }
    println!();

    let mut count = 0;
    for (row, _) in map.iter().enumerate() {
        for (column, _) in map[row].iter().enumerate() {
            print!(
                "{}",
                if is_visible(&map, row, column) {
                    count += 1;
                    'V'
                } else {
                    ' '
                }
            );
        }
        println!();
    }
    println!("{}", count);

    for row in 0..map.len() {
        for column in 0..map[0].len() {
            let score = calculate_scenic_score(&map, row, column);
            print!("{score}, ");
        }
        println!();
    }

    let max_score = (0..map.len())
        .flat_map(|row| (0..map[0].len()).map(move |column| (row, column)))
        .map(|(row, column)| calculate_scenic_score(&map, row, column))
        .max()
        .unwrap();
    println!("max score: {max_score}");
}
