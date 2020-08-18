use search_insert::search_linear;

// https://leetcode.com/problems/search-insert-position/

fn main() {

    // let vec1 = vec![1,3,5,6];
    // println!("{}", search_insert(vec1, target));

    let a = vec![1, 2, 3, 4, 5];
    let target = 5;
    println!("{:?}", search_linear(a, target));

}
