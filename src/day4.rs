pub fn run(input: String) -> usize {
    let mut count: usize = 0;
    let mut vec_2d: Vec<Vec<char>> = Vec::new();
    let mut indices_vec: Vec<(usize, usize)> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        line.chars().for_each(|x| row.push(x));
        vec_2d.push(row.clone());
        let row_it = row.iter().enumerate().filter(|(_, &c)| c == 'X');
        for (x, c) in row_it {
            indices_vec.push((y, x));
        }
    }
    //println!("{:?}", vec_2d);
    let height = vec_2d.len();
    let width = vec_2d[0].len();

    // check perpendiculars
    let pattern = ['M', 'A', 'S'];

    for (y_origin, x_origin) in indices_vec {
        // checks all 8 directions + the origin
        let y = y_origin as isize;
        let x = x_origin as isize;
        for y_offset in -1..=1 {
            for x_offset in -1..=1 {
                // failure cases
                if (x + 3 * x_offset).is_negative()
                    || (x + 3 * x_offset) >= width as isize
                    || (y + 3 * y_offset).is_negative()
                    || (y + 3 * y_offset) >= height as isize
                {
                    continue;
                } else {
                    let search_indices = [
                        (y + y_offset, x + x_offset),
                        (y + 2 * y_offset, x + 2 * x_offset),
                        (y + 3 * y_offset, x + 3 * x_offset),
                    ];
                    let mut success: bool = true;
                    for i in 0..3 {
                        if vec_2d[search_indices[i].0 as usize][search_indices[i].1 as usize]
                            != pattern[i]
                        {
                            success = false;
                        }
                    }
                    if success {
                        //println!("match!");
                        count += 1;
                    }
                }
            }
        }
    }

    // check diagonals
    //

    count
}

pub fn get_test_input() -> String {
    String::from(
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    )
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(18, run(get_test_input()))
    }
}