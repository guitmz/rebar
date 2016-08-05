pub trait Block {
    fn new(icon: Option<&str>) -> Self where Self: Sized;
    fn output(&self) -> String;
}
