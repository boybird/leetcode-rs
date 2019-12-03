pub fn main() {
  let text = "a中国a".as_bytes();
  dbg!(text);
  for c in "a中国a".chars() {
    dbg!(c);
  }
}
