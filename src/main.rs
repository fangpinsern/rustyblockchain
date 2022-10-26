use blockchain::{now, Block, Hashable};

fn main() {
    let mut block = Block::new(13, 0, vec![0; 32], 0, "Helloworld".to_owned());

    dbg!(&block);

    let h = block.hash();

    dbg!(&h);

    block.hash = h;

    dbg!(&block);
}
