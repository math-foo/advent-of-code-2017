use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let filename = "input";
    
    let mut f = File::open(filename).expect("file not found");
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let mut twos = HashMap::new();
    let mut threes = HashMap::new();

    for line in contents.lines() {
        let image = line.split("=>").nth(0).unwrap().trim();
        let enhanced_image = line.split("=>").nth(1).unwrap().trim();
        if image.len() == 5 {
            twos.insert(two_image_to_code(image),enhanced_image);
        } else {
            let three_code = three_image_to_code(image);
            threes.insert(three_code, enhanced_image);
        }
    }

    let starting_image = ".#./..#/###";
    let mut current_vector = image_to_vec(starting_image);

    for _ in 0..5 {
        let mut new_vector = Vec::new();
        if current_vector.iter().count() % 2 == 0 {
            for rows in current_vector.chunks(2) {
                let first_row = rows[0].clone();
                let second_row = rows[1].clone();

                let mut new_first_row = Vec::new();
                let mut new_second_row = Vec::new();
                let mut new_third_row = Vec::new();

                for two_block in first_row.iter().zip(second_row.iter()).collect::<Vec<_>>().chunks(2) {
                    let mut new_two_str = String::from("");
                    new_two_str.push(*two_block[0].0);
                    new_two_str.push(*two_block[0].1);
                    new_two_str.push('/');
                    new_two_str.push(*two_block[1].0);
                    new_two_str.push(*two_block[1].1);

                    let new_code = two_image_to_code(&new_two_str);
                    let new_three_str = twos.get(&new_code).unwrap();
                    let new_three_vec = image_to_vec(new_three_str);
                    new_first_row.extend(&new_three_vec[0]);
                    new_second_row.extend(&new_three_vec[1]);
                    new_third_row.extend(&new_three_vec[2]);
                }

                new_vector.push(new_first_row);
                new_vector.push(new_second_row);
                new_vector.push(new_third_row);
            }
        } else {
            for rows in current_vector.chunks(3) {
                let first_row = rows[0].clone();
                let second_row = rows[1].clone();
                let third_row = rows[2].clone();

                let mut new_first_row = Vec::new();
                let mut new_second_row = Vec::new();
                let mut new_third_row = Vec::new();
                let mut new_fourth_row = Vec::new();

                for three_block in first_row.iter().zip(second_row.iter()).zip(third_row.iter()).collect::<Vec<_>>().chunks(3) {
                    let mut new_three_str = String::from("");
                    new_three_str.push(*(three_block[0].0).0);
                    new_three_str.push(*(three_block[0].0).1);
                    new_three_str.push(*three_block[0].1);
                    new_three_str.push('/');
                    new_three_str.push(*(three_block[1].0).0);
                    new_three_str.push(*(three_block[1].0).1);
                    new_three_str.push(*three_block[1].1);
                    new_three_str.push('/');
                    new_three_str.push(*(three_block[2].0).0);
                    new_three_str.push(*(three_block[2].0).1);
                    new_three_str.push(*three_block[2].1);

                    let new_code = three_image_to_code(&new_three_str);
                    let new_four_str = threes.get(&new_code).unwrap();
                    let new_four_vec = image_to_vec(new_four_str);
                    new_first_row.extend(&new_four_vec[0]);
                    new_second_row.extend(&new_four_vec[1]);
                    new_third_row.extend(&new_four_vec[2]);
                    new_fourth_row.extend(&new_four_vec[3]);
                }

                new_vector.push(new_first_row);
                new_vector.push(new_second_row);
                new_vector.push(new_third_row);
                new_vector.push(new_fourth_row);
            }
        }

        current_vector = new_vector;
    }

    let result = current_vector.iter().map(|line| line.iter().filter(|x| **x == '#').count()).fold(0, |sum, i| sum +i);
    println!("{:?}", result);
}


fn image_to_vec(img: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for row in img.split('/') {
        let mut row_vec = Vec::new();
        for c in row.chars() {
            row_vec.push(c);
        }

        result.push(row_vec);
    }

    result
}

fn two_image_to_code(img: &str) -> u32 {
    let count = img.matches("#").count();

    if count != 2 {
        return count as u32;
    } else {
        if (img.chars().nth(0).unwrap() == img.chars().nth(1).unwrap()) || (img.chars().nth(0).unwrap() == img.chars().nth(3).unwrap()) {
            return 5;
        } else {
            return 2;
        }
    }
}

fn three_image_to_code(img: &str) -> u32 {
    let corner_lu = img.chars().nth(0).unwrap();
    let corner_ru = img.chars().nth(2).unwrap();
    let corner_lb = img.chars().nth(8).unwrap();
    let corner_rb = img.chars().nth(10).unwrap();
    let corner_str = format!("{}{}{}{}", corner_lu, corner_ru, corner_lb, corner_rb);

    let corner_count = corner_str.matches("#").count();

    let side_u = img.chars().nth(1).unwrap();
    let side_l = img.chars().nth(4).unwrap();
    let side_r = img.chars().nth(6).unwrap();
    let side_b = img.chars().nth(9).unwrap();
    let side_str = format!("{}{}{}{}", side_u, side_r, side_l, side_b);

    let side_count = side_str.matches("#").count();

    let middle_c = img.chars().nth(5).unwrap();

    let mut code = corner_count;
    if middle_c == '#' {
        code += 1000;
    }

    // no rotational differences due to corners
    if corner_count == 0 || corner_count == 4 {
        if side_count == 2 {
            if side_u == side_b {
                code += 50;
            } else {
                code += 20
            }
        } else {
            code += side_count * 10;
        }
    } else if corner_count == 1 || corner_count == 3 {
        code += 100;

        if side_count == 0 || side_count == 4 {
            code += side_count * 10;
        } else if side_count == 1 || side_count == 3 {
            code += side_count * 10;
            if corner_lu == corner_ru && corner_lu == corner_lb {
                if side_u == side_l {
                    code += 40;
                }
            } else if corner_lu == corner_ru && corner_ru == corner_rb {
                if side_u == side_r {
                    code += 40;
                }
            } else if corner_lu == corner_lb && corner_lb == corner_rb {
                if side_l == side_b {
                    code += 40;
                }
            } else {
                if side_r == side_b {
                    code += 40;
                }
            }
        } else {
            if side_r == side_l {
                code += 20;
            } else {
                if corner_lu == corner_ru && corner_lu == corner_lb {
                    if side_u == side_l {
                        code += 80;
                        if side_u == corner_lu {
                            code += 10;
                        }
                    } else {
                        code += 60;
                    }
                } else if corner_lu == corner_ru && corner_ru == corner_rb {
                    if side_u == side_r {
                        code += 80;
                        if side_u == corner_lu {
                            code += 10;
                        }
                    } else {
                        code += 60;
                    }
                } else if corner_lu == corner_lb && corner_lb == corner_rb {
                    if side_l == side_b {
                        code += 80;
                        if side_l == corner_lu {
                            code += 10;
                        }
                    } else {
                        code += 60;
                    }
                } else {
                    if side_r == side_b {
                        code += 80;
                        if side_r == corner_ru {
                            code += 10;
                        }
                    } else {
                        code += 60;
                    }
                }

            }
        }
    } else {
        code += 200;
        if corner_lu == corner_rb {
            code += 3;

            if side_count == 0 || side_count == 4 || side_count == 1 || side_count == 3 {
                code += side_count * 10;
            } else {
                if side_u == side_b {
                    code += 20;
                } else {
                    code += 60;
                    if side_u == side_l {
                        if side_u == corner_lu {
                            code += 10;
                        }
                    } else if side_u == side_r {
                        if side_u == corner_ru {
                            code += 10;
                        }
                    }
                }
            }
        } else {
            if side_count == 0 || side_count == 4 {
                code += side_count * 10;
            } else if side_count == 2 {
                if corner_lu == side_u && corner_ru == side_u {
                    if side_u == side_b {
                        if side_u == '#' {
                            code += 60;
                        } else {
                            code += 50;
                        }
                    } else {
                        code += 20;
                    }
                } else if corner_lu == side_l && corner_lb == side_l {
                    if side_u == side_b {
                        if side_l == '#' {
                            code += 60;
                        } else {
                            code += 50;
                        }
                    } else {
                        code += 20;
                    }
                } else if corner_ru == side_r && corner_rb == side_r {
                    if side_u == side_b {
                        if side_r == '#' {
                            code += 60;
                        } else {
                            code += 50;
                        }
                    } else {
                        code += 20;
                    }
                } else if corner_rb == side_b && corner_lb == side_b {
                    if side_u == side_b {
                        if side_b == '#' {
                            code += 60;
                        } else {
                            code += 50;
                        }
                    } else {
                        code += 20;
                    }
                } else {
                    code += 10;
                }
            } else {
                if side_count == 1 {
                    code += 100;
                }

                if corner_lu == corner_ru {
                    if side_u == side_l && side_u == side_r {
                        code += 30;
                    } else if side_u == side_l || side_u == side_r {
                        code += 70;
                    } else {
                        code += 90;
                    }
                } else if corner_lu == corner_lb {
                    if side_l == side_u && side_l == side_b {
                        code += 30;
                    } else if side_l == side_u || side_l == side_b {
                        code += 70;
                    } else {
                        code += 90;
                    }
                } else if corner_lb == corner_rb {
                    if side_b == side_l && side_b == side_r {
                        code += 30;
                    } else if side_b == side_l || side_b == side_r {
                        code += 70;
                    } else {
                        code += 90;
                    }
                } else {
                    if side_r == side_u && side_r == side_b {
                        code += 30;
                    } else if side_r == side_u || side_r == side_b {
                        code += 70;
                    } else {
                        code += 90;
                    }
                }
            }
        }
    }

    code as u32
}
