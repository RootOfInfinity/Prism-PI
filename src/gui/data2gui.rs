use std::collections::HashMap;

use super::blockdata::{Block, BlockID, ExprID, World};

fn ylength_for_blocks(world: &World) -> (HashMap<BlockID, u64>, HashMap<ExprID, u64>) {
    todo!()
}

fn ylength_for_block_recur(
    world: &World,
    block: Block,
) -> (HashMap<BlockID, u64>, HashMap<ExprID, u64>) {
    todo!()
}

fn location_for_blocks(
    world: &World,
    length: &(HashMap<BlockID, u64>, HashMap<ExprID, u64>),
) -> (HashMap<BlockID, (u64, u64)>, HashMap<ExprID, (u64, u64)>) {
    todo!()
}

fn location_for_block_recur(
    world: &World,
    length: &(HashMap<BlockID, u64>, HashMap<ExprID, u64>),
    block: Block,
    x: u64,
    y: u64,
) -> (HashMap<BlockID, (u64, u64)>, HashMap<ExprID, (u64, u64)>) {
    todo!()
}
