use std::collections::HashMap;

#[derive(PartialEq, Hash, Clone, Copy, Debug)]
pub struct Index {
    row: usize,
    col: usize,
}

impl Eq for Index {}

pub type CharMap = HashMap<Index, char>;

pub fn get_char_map(input: &str) -> CharMap {
    return HashMap::from_iter(
        input
            .lines()
            .enumerate()
            .map(|(row_index, line)| 
                line
                    .chars()
                    .enumerate()
                    .map(move |(col_index, char_)|
                        (Index { row: row_index, col: col_index }, char_)
                    )
            )
            .flatten()
    );
}

pub fn get_plus_coords(char_map: &CharMap) -> Vec<Index> {
    return char_map
        .iter()
        .filter(|(_, &char_)| char_ == '+')
        .map(|(&index, _)| index)
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_get_plus_coords {
        use super::*;

        #[test]
        fn single_char() {
            let plus_coords = get_plus_coords(&get_char_map("+"));
            assert_eq!(plus_coords, vec![Index { row: 0, col: 0 }])
        }

        #[test]
        fn four_chars() {
            let plus_coords = get_plus_coords(&get_char_map(
                r#" line 0
0123456  line 1
    +-+  line 2
    | |  line 3
    +-+  line 4
            "#,
            ));
            assert_eq!(
                plus_coords,
                vec![
                    Index { row: 2, col: 4 },
                    Index { row: 2, col: 6 },
                    Index { row: 4, col: 4 },
                    Index { row: 4, col: 6 },
                ])
        }
    }
}
