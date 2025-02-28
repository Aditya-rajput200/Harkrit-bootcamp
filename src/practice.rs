pub trait Summary {
  fn summarize(&self) -> String;
}
struct aser {
  name: String,
  age: i32,
}
impl Summary for aser {
  fn summarize(&self) -> String {
    format!("{}", self.name)
  }
}
