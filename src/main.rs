use sha2::{Digest, Sha256};
use std::fmt;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

// Defin difficulty of the mining
const DIFFICULTY: usize = 2;
// Defin the structure of a block in the blockchain

// struct and  impl
struct Block {
    index: u32,            //ブロックの番号
    previous_hash: String, // 前のブロックのハッシュ値
    timestamp: u64,        //ブロックの制作時間
    data: String,          // このブロックのデータ
    nonce: u64,            // マイニングのカウンター
    hash: String,          //　このブロックのハッシュ値
}

impl Block {
    /// ブロックの初期化関数
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH) //UNIX時間
            .expect("Time went blockwards")
            .as_secs(); // 時間を"秒(64)に変換するメソッド
        Block {
            index,
            previous_hash,
            timestamp,
            data,
            nonce: 0,
            hash: String::new(),
        }
    }

    fn calculate_hash(&mut self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.index,
            &self.previous_hash,
            self.timestamp,
            &self.data,
            self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        
        let hash_str = format!("{:x}", result);
        hash_str
    }
}

fn main() {
    println!("Hello, world!");
}
