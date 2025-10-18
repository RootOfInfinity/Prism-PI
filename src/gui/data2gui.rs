use std::collections::HashMap;

use slint::{Color, ToSharedString};

use super::{
    BlockData,
    blockdata::{Block, BlockID, ExprID, World},
};

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

fn create_blockdata_from_world(world: &World) -> slint::VecModel<BlockData> {
    let mut rootvec = Vec::new();
    for blk in world.0.values() {
        if blk.is_root {
            rootvec.push(blk.id);
        }
    }

    let mut lengthmap = (HashMap::new(), HashMap::new());
    for root in rootvec.iter() {
        let new_map = ylength_for_block_recur(world, (*world.0.get(&root).unwrap()).clone());
        lengthmap.0.extend(new_map.0);
    }

    let mut locmap = (HashMap::new(), HashMap::<ExprID, u64>::new());
    for root in rootvec {
        let block = (*world.0.get(&root).unwrap()).clone();
        let (x, y) = block.loc;
        locmap
            .0
            .extend(location_for_block_recur(world, &lengthmap, block, x, y).0);
    }

    let mut blkdatavec = Vec::new();
    for block in world.0.values() {
        blkdatavec.push(BlockData {
            block_color: Color::from_rgb_u8(255, 0, 0),
            block_id: block.id as i32,
            block_name: "Created From World".to_shared_string(),
            block_width: 50,
            code: "Default code lol".to_shared_string(),
            x: locmap.0.get(&block.id).unwrap().0 as f32,
            y: locmap.0.get(&block.id).unwrap().1 as f32,
        });
    }
    return blkdatavec.into();
}
