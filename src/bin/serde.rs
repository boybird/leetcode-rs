//# serde_json = "1.0"
use serde_json::Error as SJ_Error;
use serde_json::Value;

fn main() -> Result<(), SJ_Error> {
    let data = r#"
{
  "name": "Jone Doe",
  "age": 43,
  "phones": [
    "+44 1234567",
    "+44 2345578"
  ]
}"#;
    let v: Value = serde_json::from_str(data)?;
    println!("Please call {} at the number{}", v["name"], v["phones"][0]);
    Ok(())
}
