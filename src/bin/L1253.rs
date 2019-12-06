struct Solution;

impl Solution {
    pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut upper_row = vec![];
        let mut lower_row = vec![];
        let mut two_count_u = 0;
        if upper + lower
            != colsum.iter().fold(0, |acc, n| {
                if *n == 2 {
                    two_count_u = two_count_u + 1;
                }
                acc + n
            })
        {
            return vec![];
        }
        if two_count_u > upper || two_count_u > lower {
            return vec![];
        }

        for m in colsum.iter() {
            if *m == 0 {
                upper_row.push(0);
                lower_row.push(0);
            } else if *m == 2 {
                two_count_u = two_count_u - 1;
                upper_row.push(1);
                lower_row.push(1);
                upper = upper - 1;
                lower = lower - 1;
            } else {
                if upper > two_count_u {
                    upper_row.push(1);
                    lower_row.push(0);
                    upper = upper - 1;
                } else {
                    upper_row.push(0);
                    lower_row.push(1);
                    lower = lower - 1;
                }
            }
        }
        println!("{} {}", upper, lower);

        vec![upper_row, lower_row]
    }
}

fn main() {
    let upper = 2;
    let lower = 1;
    let colsum = vec![1, 1, 1];
    println!("{:?}", Solution::reconstruct_matrix(upper, lower, colsum))
}
