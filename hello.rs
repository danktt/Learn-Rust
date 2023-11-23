fn main() {
  println!("First time using rust")



  let mut changeName = 1

  println!("The type of 'variable' is: {}", std::any::type_name::<_>(changeName))

  changeName = "Danilo Miranda"

  println!("The type of 'variable' is: {}", std::any::type_name::<_>(changeName))
}

