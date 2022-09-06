
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 1;

fn main() {
  let (ready, missiles): (i32, i32) = (READY_AMOUNT, STARTING_MISSILES);
  let ready: i32 = READY_AMOUNT;
  
  println!("Firing {} of my {} missiles...", ready, missiles);
  println!("{} missiles left", missiles - ready);
}
