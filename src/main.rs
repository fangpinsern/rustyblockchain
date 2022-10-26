use blockchain::{now, Block, Blockchain, Hashable};

fn main() {
    let difficulty = 0x0000ffffffffffffffffffffffffffff;
    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        0,
        "Genesis block!".to_owned(),
        difficulty,
    );

    // block.hash = block.hash();

    dbg!(&block);

    block.mine();
    dbg!("mined genesis block{}", &block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    for i in 1..=10 {
        let mut block = Block::new(i, 0, last_hash, 0, "Another block".to_owned(), difficulty);

        block.mine();

        last_hash = block.hash.clone();

        println!("mined another block {:?}", &block);
        blockchain.blocks.push(block);
    }
}
