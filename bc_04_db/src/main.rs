use block_chain::BlockChain;

mod block;
mod block_chain;
mod proof_of_work;

fn main() {
    let bc = BlockChain::new().unwrap();

    let result = bc.verify();
}
