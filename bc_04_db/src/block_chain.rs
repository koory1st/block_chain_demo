use redb::{Database, Error};

struct BlockChain {
    tip: String,
    db: Database,
}

impl BlockChain {
    pub fn new() -> Result<BlockChain, Error> {
        let db = Database::create("my_db.redb")?;

        unimplemented!()
    }
}
