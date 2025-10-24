use std::collections::HashMap;

use slint::{Color, ToSharedString};

use crate::gui::blockdata::{BlockType, IfBlk};

use super::{
    BlockData,
    blockdata::{Block, BlockID, ExprID, World},
};

fn ylength_for_block_recur(world: &World, block: Block) -> HashMap<BlockID, u64> {
    let mut new_block_map: HashMap<BlockID, u64> = HashMap::new();
    let cur_len = match block.btype {
        BlockType::FuncStart(_) => 126 / 2,
        BlockType::Declaration(_, _) => 126 / 2,
        BlockType::Assignment(_) => 126 / 2,
        BlockType::Expression(_) => 126 / 2,
        BlockType::Return(_) => 126 / 2,
        BlockType::If(ifblk) => {
            let new_map =
                ylength_for_block_recur(world, world.0.get(&ifblk.if_stuff).unwrap().clone());
            let len: u64 = new_map.values().sum();
            new_block_map.extend(new_map);
            len + 126 / 2
        }
        BlockType::IfElse(ifblk, elseblock) => {
            let mut new_map =
                ylength_for_block_recur(world, world.0.get(&ifblk.if_stuff).unwrap().clone());
            new_map.extend(ylength_for_block_recur(
                world,
                world.0.get(&elseblock).unwrap().clone(),
            ));
            let len: u64 = new_map.values().sum();
            new_block_map.extend(new_map);
            len + 126 / 2 + 126 / 2
        }
        BlockType::While(whileblk) => {
            let new_map =
                ylength_for_block_recur(world, world.0.get(&whileblk.while_stuff).unwrap().clone());
            let len: u64 = new_map.values().sum();
            new_block_map.extend(new_map);
            len + 126 / 2
        }
        BlockType::None => 0,
    };
    new_block_map.insert(block.id, cur_len);
    if block.next != 0 && !matches!(world.0.get(&block.next).unwrap().btype, BlockType::None) {
        new_block_map.extend(ylength_for_block_recur(
            world,
            world.0.get(&block.next).unwrap().clone(),
        ));
    }
    new_block_map
}

fn location_for_block_recur(
    world: &World,
    length: &HashMap<BlockID, u64>,
    block: Block,
    x: u64,
    y: u64,
) -> (HashMap<BlockID, (u64, u64)>, HashMap<ExprID, (u64, u64)>) {
    let mut new_location_map: (HashMap<BlockID, (u64, u64)>, HashMap<ExprID, (u64, u64)>) =
        (HashMap::new(), HashMap::new());
    let cur_loc = match block.btype {
        BlockType::FuncStart(_)
        | BlockType::Declaration(_, _)
        | BlockType::Assignment(_)
        | BlockType::Expression(_)
        | BlockType::Return(_) => (x, y),
        BlockType::If(ifblk) => {
            let ifmap = location_for_block_recur(
                world,
                length,
                world.0.get(&ifblk.if_stuff).unwrap().clone(),
                x + 50,
                y + 126 / 2,
            );
            new_location_map.0.extend(ifmap.0);
            new_location_map.1.extend(ifmap.1);
            (x, y)
        }
        BlockType::IfElse(ifblk, elseblock) => {
            let ifmap = location_for_block_recur(
                world,
                length,
                world.0.get(&ifblk.if_stuff).unwrap().clone(),
                x + 50,
                y + 126 / 2,
            );
            new_location_map.0.extend(ifmap.0);
            new_location_map.1.extend(ifmap.1);
            let elsemap = location_for_block_recur(
                world,
                length,
                world.0.get(&elseblock).unwrap().clone(),
                x + 50,
                y + 126 / 2,
            );
            new_location_map.0.extend(elsemap.0);
            new_location_map.1.extend(elsemap.1);
            (x, y)
        }
        BlockType::While(whileblk) => {
            let whilemap = location_for_block_recur(
                world,
                length,
                world.0.get(&whileblk.while_stuff).unwrap().clone(),
                x + 50,
                y + 126 / 2,
            );
            new_location_map.0.extend(whilemap.0);
            new_location_map.1.extend(whilemap.1);
            (x, y)
        }
        BlockType::None => (0, 0),
    };
    new_location_map.0.insert(block.id, cur_loc);
    if block.next != 0 {
        let next_loc_map = location_for_block_recur(
            world,
            length,
            world.0.get(&block.next).unwrap().clone(),
            x,
            y + length.get(&block.next).unwrap(),
        );
        new_location_map.0.extend(next_loc_map.0);
        new_location_map.1.extend(next_loc_map.1);
    }
    new_location_map
}

pub fn create_blockdata_from_world(world: &mut World) -> slint::VecModel<BlockData> {
    let mut rootvec = Vec::new();
    for blk in world.0.values() {
        if blk.is_root {
            rootvec.push(blk.id);
        }
    }

    let mut lengthmap = HashMap::new();
    for root in rootvec.iter() {
        let new_map = ylength_for_block_recur(world, (*world.0.get(&root).unwrap()).clone());
        lengthmap.extend(new_map);
    }

    let mut locmap = (HashMap::new(), HashMap::<ExprID, u64>::new());
    for root in rootvec {
        let block = (*world.0.get(&root).unwrap()).clone();
        let (x, y) = block.loc;
        locmap
            .0
            .extend(location_for_block_recur(world, &lengthmap, block, x, y).0);
    }

    for (key, (x, y)) in locmap.0.iter() {
        println!("Location of id {}: ({}, {})", key, x, y);
    }

    for block in world.0.values_mut() {
        block.length = *lengthmap.get(&block.id).unwrap();
    }

    let mut blkdatavec = Vec::new();
    for block in world.0.values() {
        blkdatavec.push(match block.btype.clone() {
            BlockType::Expression(s) => BlockData {
                block_color: Color::from_rgb_u8(255, 0, 0),
                block_id: block.id as i32,
                block_name: s.to_shared_string(),
                block_width: 50,
                code: (s + ";").to_shared_string(),
                x: locmap.0.get(&block.id).unwrap().0 as f32,
                y: locmap.0.get(&block.id).unwrap().1 as f32,
            },
            _ => panic!("Got non expression"),
        });
    }
    return blkdatavec.into();
}
