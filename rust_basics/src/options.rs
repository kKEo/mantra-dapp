
enum Option<T> {
   Some(T), 
   None,
}

pub fn main() {
  let base_score: u32 = 85;
  let bonus_points: Option<i32> = Some(5);

  let final_score = match bonus_points {
    Some(points) => base_score + points,
    None => base_score,
  }

  println!("Score: {}", final_score);
}
