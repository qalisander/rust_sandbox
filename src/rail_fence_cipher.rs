// https://www.codewars.com/kata/58c5577d61aefcf3ff000081/train/rust

use itertools::Itertools;

fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    (0..num_rails).flat_map(|rail_id| {
         text.chars().enumerate().filter_map(move |(id, ch)| {
            let modulo = (num_rails - 1) * 2;
            if (id + rail_id) % modulo * 2 == 0 || (modulo + id - rail_id) % modulo * 2 == 0
            {
                Some(ch)
            } else {
                None
            }
        })
    }).collect::<String>()
}

fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3), "WECRLTEERDSOEEFEAOCAIVDEN");
        // assert_eq!(decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3), "WEAREDISCOVEREDFLEEATONCE");
        assert_eq!(encode_rail_fence_cipher("Hello, World!", 3), "Hoo!el,Wrdl l");
        // assert_eq!(decode_rail_fence_cipher("Hoo!el,Wrdl l", 3), "Hello, World!");
    }
}
