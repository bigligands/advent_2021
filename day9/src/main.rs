use std::fs;

fn main() {
    let input_path: &'static str = "input.txt";
    let input = fs::read_to_string(input_path).unwrap();
    let mut input_matrix: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        let line_array = line
            .chars() // convert line into iterator of chars
            .map(|c| c.to_digit(10)) // map chars to a digit
            .filter(|c| c.is_some()) // filter non-digits
            .map(|c| c.unwrap() as u8) // retrieve value from Somes
            .collect::<Vec<u8>>(); // collect into a vector

        input_matrix.push(line_array); // push the vector into the matrix
    }

    let buffers = collect_buffers(&input_matrix);

    let mut sum = 0;
    for buff in &buffers[..] {
        sum += buff.scan_buffer_for_low_points();
    }
    println!("total sum: {}", sum);
}

/// A segment of input data split into either 2 or 3 rows depending on position.
enum Buffer<'a> {
    /// First line of the input
    First(&'a [Vec<u8>]),
    /// All lines between the first and last line
    Standard(&'a [Vec<u8>]),
    /// The last line of the input
    Last(&'a [Vec<u8>]),
}

fn collect_buffers<'a>(input_matrix: &'a Vec<Vec<u8>>) -> Vec<Buffer> {
    let mut buffers = Vec::new();
    let bound: usize = input_matrix.iter().count();
    for (i, _) in input_matrix.iter().enumerate() {
        // Depending on i (row number), assign the buffer a position in the input
        // Any line besides the first or last line
        if i as usize != 0 && i as usize != bound - 1 {
            let buffer = Buffer::Standard(&input_matrix[i - 1..=i + 1]);
            buffers.push(buffer);
        // First line of the input
        } else if i == 0 {
            let buffer = Buffer::First(&input_matrix[0..=1]);
            buffers.push(buffer);
        // Last line of the input
        } else {
            let buffer = Buffer::Last(&input_matrix[bound - 2..bound]);
            buffers.push(buffer);
        }
    }
    buffers
}

impl<'a> Buffer<'a> {
    // part 1
    fn scan_buffer_for_low_points(&'a self) -> u32 {
        let all_low_points: Vec<u8> = match self {
            Buffer::First(buff) => {
                let len = buff[0].iter().count();
                let mut low_points: Vec<u8> = Vec::new();
                for (i, _) in buff[0][..].iter().enumerate() {
                    let mut comp_values: Vec<u8> = Vec::new();

                    if i != 0 && i < len - 1 {
                        // behind
                        comp_values.push(buff[0][i - 1]);
                        // besides
                        comp_values.push(buff[0][i + 1]);
                        // below
                        comp_values.push(buff[1][i]);
                    }
                    if i == 0 {
                        // besides
                        comp_values.push(buff[0][i + 1]);
                        // below
                        comp_values.push(buff[1][i]);
                    } else if i == len - 1 {
                        // behind
                        comp_values.push(buff[0][i - 1]);
                        // below
                        comp_values.push(buff[1][i]);
                    }

                    let test_point = buff[0][i];
                    if comp_values.iter().filter(|c| **c <= test_point).count() == 0 {
                        low_points.push(test_point);
                    }
                }
                low_points
            }
            Buffer::Standard(buff) => {
                // want 1 compared to 0 and 2
                let len = buff[0].iter().count();
                let mut low_points: Vec<u8> = Vec::new();
                for (i, _) in buff[0][..].iter().enumerate() {
                    // gather comparison values to compare to test point
                    let mut comp_values: Vec<u8> = Vec::new();
                    if i != 0 && i < len - 1 {
                        // above
                        comp_values.push(buff[0][i]);
                        // below
                        comp_values.push(buff[2][i]);
                        // behind
                        comp_values.push(buff[1][i - 1]);
                        // besides
                        comp_values.push(buff[1][i + 1]);
                    } else if i == 0 {
                        // above
                        comp_values.push(buff[0][i]);
                        // below
                        comp_values.push(buff[2][i]);
                        // beside
                        comp_values.push(buff[1][i + 1]);
                    } else if i == len - 1 {
                        // above
                        comp_values.push(buff[0][i]);
                        // below
                        comp_values.push(buff[2][i]);
                        // behind
                        comp_values.push(buff[1][i - 1]);
                    }

                    let test_point = buff[1][i];
                    if comp_values.iter().filter(|c| **c <= test_point).count() == 0 {
                        low_points.push(test_point);
                    }
                }
                low_points
            }
            Buffer::Last(buff) => {
                let len = buff[0].iter().count();
                let mut low_points: Vec<u8> = Vec::new();
                for (i, _) in buff[0][..].iter().enumerate() {
                    let mut comp_values: Vec<u8> = Vec::new();

                    if i != 0 && i < len - 1 {
                        // behind
                        comp_values.push(buff[1][i - 1]);
                        // besides
                        comp_values.push(buff[1][i + 1]);
                        // above
                        comp_values.push(buff[0][i]);
                    }
                    if i == 0 {
                        // besides
                        comp_values.push(buff[1][i + 1]);
                        // above
                        comp_values.push(buff[0][i]);
                    } else if i == len - 1 {
                        // behind
                        comp_values.push(buff[1][i - 1]);
                        // above
                        comp_values.push(buff[0][i]);
                    }

                    let test_point = buff[1][i];
                    if comp_values.iter().filter(|c| **c <= test_point).count() == 0 {
                        low_points.push(test_point);
                    }
                }
                low_points
            }
        };
        // return 1 + target value as risk level -> sum(target_value) + n
        all_low_points.iter().map(|x| *x as u32).sum::<u32>() + all_low_points.iter().count() as u32
    }
}
