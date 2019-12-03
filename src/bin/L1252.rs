struct Solution;
impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for i in 0..n {
            for j in 0..m {
                let mut s = 0;
                for point in indices.iter() {
                    if point[0] == i {
                        s = s + 1;
                    }
                    if point[1] == j {
                        s = s + 1;
                    }
                }
                if s % 2 != 0 {
                    result = result + 1;
                }
            }
        }
        result
    }
}

fn main() {
    let _r = Solution::odd_cells(48, 37, vec![vec![40, 5]]);
    println!("{}", _r);
}
