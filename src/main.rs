use sha2::{Digest, Sha256};
use std::fmt;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use hex;

// 採掘の難易度を定義する
const DIFFICULTY: usize = 2;
// ブロックチェーンにおけるブロックの構造を定義する

// struct and  impl
#[derive(Debug)]
pub struct Block {
    index: u32,            //ブロックの番号
    previous_hash: String, // 前のブロックのハッシュ値
    timestamp: u64,        //ブロックの制作時間
    data: String,          // このブロックのデータ
    nonce: u64,            // マイニングのカウンター
    hash: String,          //　このブロックのハッシュ値
}

impl Block {
    /// ブロックの初期化関数
    pub fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH) //UNIX時間
            .expect("時間がブロックの方向に進んだ")
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

       let hex_result = hex::encode(result); 

        let hash_str = format!("{}", hex_result);
        hash_str
    }

    /// マイニングしてハッシュを決める関数
    pub fn mine_block_with_visual_effects(&mut self) {
        let mut iterations = 0;
        // PoWの最小実装
        loop {
            self.hash = self.calculate_hash();
            // DIFFICULTYマイニングの難しさを決める数値
            // ハッシュの先頭が00になたっらマイニング成功    2                         2
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "0".repeat(DIFFICULTY) { 
                dbg!("DBG",&self.index);
                println!(" ブロックマイニング: {}", self.index);
                break;
            }

            // マイニング回数100以上試しら強制終了
            if iterations > 256 {
                print!(" 採掘作業中...  ");
                thread::sleep(Duration::from_millis(3000));
                // 最後のハッシュ
                // dbg!("DBG",&self.hash);
                println!("calculate_hash: {}", self.hash);
                break;
            }
            self.nonce += 1;
            iterations += 1;
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
        let datetime = chrono::DateTime::from_timestamp(self.timestamp as i64, 0).unwrap();
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
    println!("ブロックチェーンマイニングシミュレーターへようこそ");
    println!(" マイナー名を入力してください :");
    // 空の文字列を作って、入れる箱を用意
    let mut miner_name = String::new();

    // 標準入力
    std::io::stdin()
        .read_line(&mut miner_name)
        .expect(" 入力の読み取りに失敗しました");
    miner_name = miner_name.trim().to_string();

    // ランダムに取引相手として登場するNPC（仮想トレーダ一覧）
    let trader_name = vec!["Bob", "Linda", "Jhon", "Omar", "Eve", "Svetlana", "Jiro"];

    // 新しいブロックチェーンを作成
    let mut noecoin = Blockchain::new();
    println!(" マイニングとトランザクションのシミュレーション");

    //　文字列をコピーして、senderに所有権を渡している
    let mut sender = miner_name.clone();

    for i in 0..trader_name.len() {
        println!("採掘ブロック {}...⛏", i + 1);
        let rescipient = if i < trader_name.len() - 1 { 
            // トレーダがマイニング成功　→　次のトレーダへ
            trader_name[i + 1].to_string()
        } else {
            //　失敗・自分がマイニング
            miner_name.clone()
        };

        // トランザクションを作成
        let transaction = format!("{} そして {}", sender, rescipient);

        //　新しいブロック作成
        let new_block = Block::new((i + 1) as u32, String::new(), transaction.clone());

        //　ブロックチェーンに追加
        noecoin.add_block(new_block);
        println!("取引 {}", transaction);

        // 送金者の更新（次のループへ）
        sender = rescipient;
        println!();

        //　現在のチェーンの長さを取得
        let total_blocks = noecoin.get_total_blocks();
        println!(" ブロックチェーンに追加されたブロックの総数: {}", total_blocks);

        //　報酬計算
        let noecoin_per_block = 1150;
        let noecoin_traded = total_blocks * noecoin_per_block;
        println!("💸 取引されたnoecoinの総数: {}", noecoin_traded);

        let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backtrawis").as_secs();

        let end_datetime = chrono::DateTime::from_timestamp((end_timestamp) as i64, 0 );
        println!(" 終了日時 {:?}", end_datetime);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //マイニングで00が一致しているか
    fn test() {
       let mut block_test = Block::new(0,String::new(), String::new());
       block_test.mine_block_with_visual_effects();
        // 00 00 になるには平均256回回せなければならない
       assert_eq!(&block_test.hash[..DIFFICULTY], "00"); 
    }

    //現在のチェーンの長さが合っているか
    fn test_add_block() {
        let test_add_block = Blockchain::new();
        test_add_block.add_block(new_block);
    }

}

