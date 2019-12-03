//# regex = "1.2.1"

use regex::Regex;

const TO_SEARCH: &'static str = "
On 2010-03-14, foo happened. On 2014-10-14, bar happend.
";

fn main() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    for caps in re.captures_iter(TO_SEARCH) {
        println!(
            "year: {}, month: {}, day: {}",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(3).unwrap().as_str(),
        );
    }
}
