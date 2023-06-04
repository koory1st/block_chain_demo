use std::io::Read;

use siamesedb::{filedb::FileDb, DbMapDbString, DbXxx, DbXxxBase, DbXxxObjectSafe};

use crate::block::Block;

pub struct BlockChain {
    tip: String,
    db: FileDb,
}

impl BlockChain {
    pub fn new() -> std::io::Result<BlockChain> {
        let db_name = "doc-test1.siamesedb";

        let db = siamesedb::open_file(db_name)?;

        let mut block_map = db.db_map_string("block")?;

        let last = block_map.get_string("l")?;

        if last.is_none() {
            let genesis_block = Block::new("Genesis", "");

            let hash = genesis_block.hash.to_owned();

            block_map.put_string("l", &hash)?;

            let block_serial = bincode::serialize(&genesis_block).unwrap();
            let block_serial = block_serial.as_slice();
            block_map.put(&hash, block_serial)?;

            let block_chain = BlockChain {
                tip: hash.to_owned(),
                db: db,
            };
            return Ok(block_chain);
        }

        let block_chain = BlockChain {
            tip: last.unwrap(),
            db: db,
        };
        Ok(block_chain)
    }
}
