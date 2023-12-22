use crate::parsing::preprocessing;

//use core::slice;

mod parsing;

//type Index = (usize, usize);

//#[derive(PartialEq, Debug)]
//struct Node {
    //indexes: Vec<Index>,
//}

//#[derive(PartialEq, Debug)]
//struct InfraGraph {
    //nodes: Vec<Node>,
//}

////fn get_infra_graph(input: &str) -> InfraGraph {
////let char_map = preprocessing::get_char_map(input);
////let plus_coords = preprocessing::get_plus_coords(&char_map);
////return find_nodes(plus_coords, char_map);
////}

////fn find_nodes(
////plus_coords: Vec<Index>,
////char_map: preprocessing::CharMap,
////) -> InfraGraph {
////let mut visited = HashSet::new();
////let mut graph = InfraGraph { nodes: vec![] };
////for &index in &plus_coords {
////if visited.contains(&index) {
////continue;
////}
////if let Some(node) = find_node(index, &char_map) {
////visited.extend(node.indexes);
////} else {
////visited.insert(index);
////}
////}
////return InfraGraph { nodes: Vec::new() };
////}

//fn find_top_right_marker(
    //top_left: Index,
    //char_map: &preprocessing::CharMap,
//) -> Option<Index> {
    //let (row_index, start_col_index) = top_left;
    //if let Some((right_col, _)) = char_map[row_index]
        //.iter()
        //.enumerate()
        //.skip(start_col_index + 1)
        //.find(|&(_, &char_)| char_ == '+')
    //{
        //return Some((row_index, right_col));
    //};
    //return None;
//}

//fn find_bottom_right_marker(
    //top_right: Index,
    //char_map: &preprocessing::CharMap,
//) -> Option<Index> {
    //let (start_row_index, col_index) = top_right;
    //if let Some((bottom_row, _)) = char_map
        //.iter()
        //.map(|row| row[col_index])
        //.enumerate()
        //.skip(start_row_index + 1)
        //.find(|&(_, char_)| char_ == '+')
    //{
        //return Some((bottom_row, col_index));
    //};
    //return None;
//}

////fn find_node(index: Index, char_map: &preprocessing::CharMap) -> Option<Node> {
////if let Some(top_right_plus_index) =
////find_top_right_plus_index(
////index, &char_map)
////{
////if let Some(bottom_right_plus_index) =
////find_bottom_right_plus_index(
////top_right_marker.index, &char_map)
////{
////if let Some(bottom_left_plus_index) =
////find_bottom_left_plus_index(
////top_right_marker.index, &char_map)
////{
////if let Some(top_left_plus_index) =
////find_top_left_plus_index(
////bottom_left_marker.index, &char_map)
////{
////if bottom_left_plus_index == index {
////return build_node(
////top_left_plus_index,
////bottom_right_plus_index,
////);
////}
////}
////}
////}
////}
////return None;
////}

//fn build_node(top_left_index: Index, bottom_right_index: Index) -> Node {
    //return Node {
        //indexes: get_indexes(top_left_index, bottom_right_index),
    //};
//}

//fn get_indexes(
    //top_left_index: Index,
    //bottom_right_index: Index,
//) -> Vec<Index> {
    //let (start_row, start_col) = top_left_index;
    //let (end_row, end_col) = bottom_right_index;
    //return (start_row..end_row + 1)
        //.map(|row| (start_col..end_col + 1).map(move |col| (row, col)))
        //.flatten()
        //.collect();
//}

//#[cfg(test)]
//mod tests {
    //use super::*;

    //#[test]
    //fn test_find_top_right_marker() {
        //let input = concat!("foo\n", " +-----+!\n", "bar",);
        //let char_map = preprocessing::get_char_map(input);
        //let top_left_index = (1, 1);
        //assert_eq!(
            //find_top_right_marker(top_left_index, &char_map,),
            //Some((1, 7)),
        //)
    //}

    //#[test]
    //fn test_find_bottom_right_marker() {
        //let input = r#" line 0
//0123456  line 1
    //+-+  line 2
    //| |  line 3
    //| |  line 4
    //+-+  line 5
            //"#;
        //let char_map = preprocessing::get_char_map(input);
        //let top_right_index = (2, 6);
        //assert_eq!(
            //find_bottom_right_marker(top_right_index, &char_map,),
            //Some((5, 6)),
        //)
    //}

    //#[test]
    //fn get_few_indexes() {
        //assert_eq!(
            //get_indexes((1, 2), (4, 6)),
            //vec![
                //(1, 2), (1, 3), (1, 4), (1, 5), (1, 6),
                //(2, 2), (2, 3), (2, 4), (2, 5), (2, 6),
                //(3, 2), (3, 3), (3, 4), (3, 5), (3, 6),
                //(4, 2), (4, 3), (4, 4), (4, 5), (4, 6),
            //],
        //)
    //}

    //#[test]
    //fn test_build_node() {
        //assert_eq!(
            //build_node((1, 1), (2, 2)),
            //Node {
                //indexes: vec![(1, 1), (1, 2), (2, 1), (2, 2),]
            //},
        //)
    //}

    ////#[test]
    ////fn empty_string() {
    ////let input = String::from("");
    ////assert_eq!(get_infra_graph(&input), InfraGraph { nodes: Vec::new() })
    ////}

    ////#[test]
    ////fn smallest_possible_node() {
    ////// cols:
    //////+10       +20
    //////01234567890123456789
    ////let input = r#"
    ////++
    ////++
    ////"#;
    ////assert_eq!(
    ////get_infra_graph(&input),
    ////InfraGraph {
    ////nodes: vec![Node {
    ////indexes: vec![(1, 12), (1, 13), (2, 13), (2, 12)]
    ////}]
    ////},
    ////)
    ////}
//}
