use anyhow::Result;
use pickledb::PickleDb;

use crate::block::Block;

pub struct BlockChain {
    tip: String,
    db: PickleDb,
}

struct BlockChainIter<'a> {
    curr_hash: String,
    block_chain: &'a BlockChain,
}

impl<'a> BlockChainIter<'a> {
    fn new(block_chain: &BlockChain) -> BlockChainIter {
        BlockChainIter {
            curr_hash: block_chain.tip.clone(),
            block_chain,
        }
    }
}

impl<'a> Iterator for BlockChainIter<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let block = self.block_chain.db.get::<Block>(&self.curr_hash);

        match block {
            Some(block) => {
                let hash = block.prev_block_hash.clone();
                self.curr_hash = hash;
                return Some(block);
            }
            None => None,
        }
    }
}

const DB_PATH: &'static str = "block.db";
impl BlockChain {
    pub fn new() -> Result<BlockChain> {
        let last_data = get_db_last_data();

        if let Some(last) = last_data {
            dbg!(&last.1);
            return Ok(BlockChain {
                tip: last.1,
                db: last.0,
            });
        }

        let mut db = PickleDb::new(
            DB_PATH,
            pickledb::PickleDbDumpPolicy::AutoDump,
            pickledb::SerializationMethod::Bin,
        );
        let genesis_block = Block::new("Genesis", "");

        let hash = genesis_block.hash.to_owned();

        db.set("l", &hash)?;

        db.set(&hash, &genesis_block)?;

        let block_chain = BlockChain {
            tip: hash.to_owned(),
            db: db,
        };
        Ok(block_chain)
    }

    pub fn verify(&self) -> Result<()> {
        let bci = BlockChainIter::new(self);

        for block in bci {
            dbg!(&block);
        }
        Ok(())
    }
}

fn get_db_last_data() -> Option<(PickleDb, String)> {
    let result = PickleDb::load(
        DB_PATH,
        pickledb::PickleDbDumpPolicy::AutoDump,
        pickledb::SerializationMethod::Bin,
    );

    if let Err(_) = result {
        return None;
    }

    let db = result.unwrap();

    let last = db.get::<String>("l");

    last.map(|v| return (db, v))
}
