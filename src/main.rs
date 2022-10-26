use blockchain::{now, Block, Hashable};

fn main() {
    let mut block = Block::new(
        13,
        0,
        vec![0; 32],
        0,
        "Genesis block!".to_owned(),
        0x00000fffffffffffffffffffffffffff,
    );

    block.hash = block.hash();

    dbg!(&block);

    block.mine();
    dbg!(&block);
}
