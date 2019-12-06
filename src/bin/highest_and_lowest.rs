fn main() {
    let s = "1 2 -3 4 5";
    let v: Vec<i32> = s
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let min = v.iter().min().unwrap();
    let max = v.iter().max().unwrap();
    dbg!(min, max);
}
