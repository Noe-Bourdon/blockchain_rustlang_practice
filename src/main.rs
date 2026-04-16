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

    /// ハッシュの計算
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

    /// マイニングしてハッシュを決める関数
    fn mine_block_with_visual_effects(&mut self) {
        let iterations = 0;
        // PoWの最小実装
        loop {
            self.hash = self.calculate_hash();
            // DIFFICULTYマイニングの難しさを決める数値
            // ハッシュの先頭が00になたっらマイニング成功
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00".repeat(DIFFICULTY) {
                println!(" Block mined: {}", self.index);
                break;
            }

            // マイニング回数100以上試しら強制終了
            if iterations > 100 {
                print!(" Mining in progress...  ");
                thread::sleep(Duration::from_millis(3000));
                // 最後のハッシュ
                println!("calculate_hash: {}",self.hash);
                break;
            }
            self.nonce += 1;
        }
    }
}

impl fmt::Display for Block {
    /// Blockの表示フォーマットの設定
    /// ```rust
    /// index{} ブロックの番号
    /// date{} ブロックに入ってるデータ
    /// datetime{} そのブロックが作られて日時
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);
        write!(
            f,
            "Block {}: {} at {}",
            self.index, self.data, datetime
        )
    }
}



fn main() {
    println!("Hello, world!");
}
