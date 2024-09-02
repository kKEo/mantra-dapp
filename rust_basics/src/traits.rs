struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {

    fn area(&self) -> u32 {
       self.width * self.height
    }

    fn is_squer(&self) -> bool {
       self.width == self.height
    }

}

trait Shape {
   fn area(&self) -> f64;
   fn name(&self) -> &str;
}

impl Shape for Circle {
  fn area(&self) -> f64 {
     std::f64::consts::PI * self.radius * self.radius
  }

  fn name(&self) -> &str {
     "circle"
  }

}
