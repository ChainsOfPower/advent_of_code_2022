pub fn count_visible_trees(input: &str) -> usize {
    let trees = parse_trees(input);
    count_visible(&trees)
}

pub fn parse_trees(input: &str) -> Vec<Vec<u32>> {
    let mut result = Vec::new();

    input
        .lines()
        .for_each(|line| {
            result.push(Vec::new());

            line.chars()
                .for_each(|c| {
                    result.last_mut().unwrap().push(c.to_digit(10).unwrap());
                });
        });
    
    result
}

pub fn calculate_the_tree_with_best_visibility(trees: &Vec<Vec<u32>>) -> u32 {
    let mut max = 0;
    
    let rows = trees.len();
    let columns = trees[0].len();

    for i in 1..rows-1 {
        for j in 1..columns-1 {
            let current_tree = trees[i][j];

            let mut up_count = 0;
            for idx in 0..i {
                let idx = i - idx - 1;
                up_count += 1;
                
                if trees[idx][j] >= current_tree {
                    break;
                }
            }

            let mut down_count = 0;
            for idx in i + 1..rows {
                down_count += 1;

                if trees[idx][j] >= current_tree {
                    break;
                }
            }

            let mut left_count = 0;
            for idx in 0..j {
                let idx = j - idx - 1;
                left_count += 1;

                if trees[i][idx] >= current_tree {
                    break;
                }
            }

            let mut right_count = 0;
            for idx in j + 1..columns {
                right_count += 1;

                if trees[i][idx] >= current_tree {
                    break;
                }
            }

            max = max.max(up_count * down_count * left_count * right_count);
        }
    }

    max
}

fn count_visible(trees: &Vec<Vec<u32>>) -> usize {
    let outer_tree_count = trees.len() * 2 + trees[0].len() * 2 - 4;

    let mut visible_inner_indexes_horizontal = get_visible_inner_indexes_horizontal(trees);
    let visible_inner_indexes_vertical = get_visible_inner_indexes_vertical(trees);

    visible_inner_indexes_horizontal.extend(&visible_inner_indexes_vertical);
    visible_inner_indexes_horizontal.sort();
    visible_inner_indexes_horizontal.dedup();
    visible_inner_indexes_horizontal.len() + outer_tree_count
}

fn get_visible_inner_indexes_horizontal(trees: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut indexes = Vec::new();
    for (i, line) in trees.iter().enumerate() {
        let mut largest = &line[0];
        for (j, c) in line.iter().enumerate() {
            if i > 0 && i < trees.len() - 1 && j > 0 && j < line.len() - 1 && c > largest {
                indexes.push((i, j));
                largest = c;
            }
        }

        let mut largest = &line[line.len() - 1];
        for (j, c) in line.iter().rev().enumerate() {
            if i > 0 && i < trees.len() - 1 && j > 0 && j < line.len() - 1 && c > largest {
                indexes.push((i, line.len() - 1 - j));
                largest = c;
            }
        }
    }

    indexes
}

fn get_visible_inner_indexes_vertical(trees: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let rows = trees.len();
    let columns = trees[0].len();

    let mut indexes = Vec::new();

    let iter = (0..columns).map(|column_idx| trees.iter().flatten().skip(column_idx).step_by(rows));
    for (j, column_values) in iter.enumerate() {
        let mut largest = trees[0][j];
        for (i, &row_value) in column_values.enumerate() {
            if i > 0 && i < trees.len() - 1 && j > 0 && j < columns - 1 && row_value > largest {
                indexes.push((i, j));
                largest = row_value;
            }
        }
    }

    let iter = (0..columns).map(|column_idx| trees.iter().flatten().skip(column_idx).step_by(rows));
    for (j, column_values) in iter.enumerate() {
        let mut largest = trees[trees.len() - 1][j];
        for (i, &&row_value) in column_values.collect::<Vec<_>>().iter().rev().enumerate() {
            if i > 0 && i < trees.len() - 1 && j > 0 && j < columns - 1 && row_value > largest {
                indexes.push((trees.len() - 1 - i, j));
                largest = row_value;
            }
        }
    }
    indexes
}
