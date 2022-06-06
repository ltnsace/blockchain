use core::blockchain;
use std::thread;
use std::time::Duration;

use indicatif::ProgressBar;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();

    bc.add_block(String::from("a -> b: 5 btc"));
    println!("produce a block !");

    bc.add_block("c -> d: 1 btc".to_string());
    println!("produce a block !");

    for b in bc.blocks {

        println!("++++++++++++++++++++++++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("");
        println!("start mining .... ");
        let pb = ProgressBar::new(1024);
        for _ in 0..1024 {
            pb.inc(1);
            thread::sleep(Duration::from_millis(5));
        }
        pb.finish_with_message("done");
    }
}
