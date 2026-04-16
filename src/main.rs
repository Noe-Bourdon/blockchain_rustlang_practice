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
            self.index, &self.previous_hash, self.timestamp, &self.data, self.nonce
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
                println!("calculate_hash: {}", self.hash);
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
        write!(f, "Block {}: {} at {}", self.index, self.data, datetime)
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    /// ブロックチェーンを初期化する
    fn new() -> Blockchain {
        // 一番最初のブロックを作ってチェーンでつなげる
        let genesis_block = Block::new(0, String::new(), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    /// ブロックをチェーンに追加する
    /// クローンして新しく塗り替える → マイニング計算 → 完成したらチェーンにもう一度入れる
    fn add_block(&mut self, mut new_block: Block) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        new_block.previous_hash = previous_hash;
        new_block.mine_block_with_visual_effects();
        self.chain.push(new_block);
    }

    /// 現在のチェーンの長さを見る
    fn get_total_blocks(&self) -> usize {
        self.chain.len()
    }
}

fn main() {
    println!(" Welcome to Blockchain Mining Simulator ");
    println!(" Enter your miner name :");
    let mut miner_name = String::new();

    std::io::stdin()
        .read_line(&mut miner_name)
        .expect(" Failed to read input");
    miner_name = miner_name.trim().to_string();

    let trader_name = vec!["Bob", "Linda", "Jhon", "Omar", "Eve", "Svetlana", "Jiro"];

    let mut noecoin = Blockchain::new();
    println!(" minig and simulating transactions");

    let mut sender = miner_name.clone();

    for i in 0..trader_name.len() {
        println!("Mining block {}...⛏", i + 1);
        let rescipient = if i < trader_name.len() - 1 {
            trader_name[i + 1].to_string();
        } else {
            miner_name.clone();
        };

        let transaction = format!("{} and to {}", sender, rescipient);

        let new_block = Block::new((i + 1) as u32, String::new(), transaction.clone());

        noecoin.add_block(new_block);
        println!(" Transaction {}", transaction);

        sender = rescipient;
        println!();

        let total_blocks = noecoin.get_total_blocks();
        println!(" Total blocks added to the blockchain: {}", total_blocks);

        let noecoin_per_block = 137;
        let noecoin_traded = total_blocks * noecoin_per_block;
        println!("💸 Total noecoin traded: {}", noecoin_traded);

        
    }
}
