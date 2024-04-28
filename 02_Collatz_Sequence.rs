/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
  
  // If n is 0 or less, sequence does not exist
  if n <= 0 {
     return 0;
  }
  
  let mut seq_length: u32 = 1;
  
  while n != 1 {
      // Even Condition
      if n % 2 == 0 {
          n /= 2;
          
      // Odd Condition    
      } else {
          n = 3 * n + 1;
      }
      
     // Increment the sequence length
     seq_length += 1;
  }
  
  seq_length
}

fn main() {
  let collatz = collatz_length(3);
  println!("collatz sequence length for 3 is {collatz}");
}
