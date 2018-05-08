pub trait Block {
    fn new() -> Self where Self: Sized;
    fn output(&self) -> String;

    fn tasks(&mut self) {

    }
}
