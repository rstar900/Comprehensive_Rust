use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

// Implementing the `Read` trait for `RotDecoder`.
impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let mut bytes_read: usize = 0;
        let mut idx: usize = 0;
        
        for byte in self.input.by_ref().bytes() {
           bytes_read += match byte.unwrap() {
            
                val => {
                    if val.is_ascii_alphabetic() {
                        if val.is_ascii_uppercase() {
                            buf[idx] = 'A' as u8 + (val - ('A' as u8) + self.rot) % 26;
                        } else {
                            buf[idx] = 'a' as u8 + (val - ('a' as u8) + self.rot) % 26;
                        }
                    } else {
                       buf[idx] = val;
                    }
                    
                    idx += 1;
                    
                    1
                }
            };
        }
        
        Ok(bytes_read)
    }
}
  
fn main() {
    let mut rot =
        RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}
