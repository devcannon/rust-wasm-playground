use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[derive(Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct BinaryMeta {
    size: usize,
    sum_bytes: u32,
}

#[wasm_bindgen]
impl BinaryMeta {
    pub fn get_size(self) -> usize {
        self.size
    }

    pub fn get_sum_bytes(self) -> u32 {
        self.sum_bytes
    }

    pub fn get_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[wasm_bindgen]
pub fn check_binary(data: &[u8]) -> Option<BinaryMeta> {
    if data.len() >= 10 {
        let mut sum: u32 = 0;
        for element in data {
            sum += *element as u32;
        }

        return Some(BinaryMeta {
            size: data.len(),
            sum_bytes: sum,
        });
    }

    None
}

mod test {
    use super::*;

    #[test]
    fn test_sum_not_none() {
        assert_ne!(check_binary(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), None);
    }

    #[test]
    fn test_sum_none() {
        assert_eq!(check_binary(&vec![1, 2, 3, 4, 5, 6]), None);
    }
}
