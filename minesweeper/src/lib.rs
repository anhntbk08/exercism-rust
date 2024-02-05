fn count_mine(ch: char) -> u64 {
    if ch == '*' {
        return 1
    }

    return 0
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.len() == 0 {
        return Vec::new()
    }

    let xlength = minefield[0].len();
    let ylength = minefield.len();
    let mut result = vec![];
    let char_array: Vec<Vec<char>> = minefield.iter().map(|str| str.chars().collect()).collect();

    for ix in 0..ylength {
        let mut mine_counts: Vec<String> = Vec::new();
        let mut count;

        for iy in 0..xlength {
            count = 0;

            if char_array[ix][iy] == '*' {
                mine_counts.push(String::from("*"));
                continue;
            }

            if iy >= 1 {
                count +=  count_mine(char_array[ix][iy-1]);
                
                if  ix + 1 < ylength {
                    count +=  count_mine(char_array[ix+1][iy-1]);
                }

                if  ix >= 1 {
                    count +=  count_mine(char_array[ix-1][iy-1]);
                }
            }

            if  iy + 1 < xlength {
                count +=  count_mine(char_array[ix][iy+1]);

                if  ix + 1 < ylength {
                    count +=  count_mine(char_array[ix+1][iy+1]);
                }
                
                if  ix >= 1 {
                    count +=  count_mine(char_array[ix-1][iy+1]);
                }
            }

            if ix >= 1 {
                count +=  count_mine(char_array[ix-1][iy]);
            }

            if  ix + 1 < ylength {
                count +=  count_mine(char_array[ix+1][iy]) 
            }

            mine_counts.push(count.to_string());
        }

        result.push(mine_counts.iter().map(|c| c.replace("0", " ")).collect::<Vec<String>>().join(""))
    }

    result
}


// static NEIGBOURHOOD_OFFSETS: &'static [(i32, i32)] = &[
//     (-1, -1), (0, -1), (1, -1),
//     (-1,  0),          (1,  0),
//     (-1,  1), (0,  1), (1,  1),
// ];

// pub fn annotate(field: &[&str]) -> Vec<String> {
//     let height = field.len() as i32;
//     (0..height).map(|y| {
//         let width = field[y as usize].len() as i32;
//         (0..width).map(|x| {
//             if field[y as usize].as_bytes()[x as usize] == b'*' {
//                 '*'
//             } else {
//                 match NEIGBOURHOOD_OFFSETS.iter()
//                     .map(|&(ox, oy)| (x + ox, y + oy))
//                     .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
//                     .filter(|&(x, y)| field[y as usize].as_bytes()[x as usize] == b'*')
//                     .count() {
//                         0 => ' ',
//                         n => (n as u8 + '0' as u8) as char
//                     }
//             }
//         }).collect()
//     }).collect()
// }