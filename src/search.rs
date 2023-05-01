// use min_max_heap::MinMaxHeap;
use std::collections::{BinaryHeap, HashMap};

use enum_map::EnumMap;

use crate::{calculate_hash, Action, ActionName, Player};

const MAX_TIME: u32 = 10000;

const ACTION_NAME_LIST: [ActionName; 15] = [
    ActionName::FastBlade,
    ActionName::FightOrFlight,
    ActionName::RiotBlade,
    ActionName::CircleOfScorn,
    ActionName::GoringBlade,
    ActionName::RoyalAuthority,
    ActionName::HolySpirit,
    ActionName::Requiescat,
    ActionName::Intervene,
    ActionName::Atonement,
    ActionName::Confiteor,
    ActionName::Expiacion,
    ActionName::BladeOfFaith,
    ActionName::BladeOfTruth,
    ActionName::BladeOfValor,
];

pub fn search(actions_map: &EnumMap<ActionName, Action>) {
    let mut player = Player::default();
    player.assign_actions(actions_map);

    let h = calculate_hash(&player);

    // let mut heap = MinMaxHeap::new();
    let mut heap = BinaryHeap::new();
    let mut damages = HashMap::new();
    let mut history = HashMap::new();
    damages.insert(h, player.damage);
    history.insert(h, (0u64, ActionName::None));
    heap.push(player);

    let mut cnt = 0;
    let mut ans = 0;
    let mut best_h: u64 = 0;

    while !heap.is_empty() {
        // let mut player = heap.pop_max().unwrap();
        let mut player = heap.pop().unwrap();
        let h = calculate_hash(&player);
        player.damage = *damages.get(&h).unwrap();
        if player.damage > ans {
            ans = player.damage;
            best_h = h;
            println!("new best! {}", ans);
        }
        cnt += 1;
        if cnt % 50000 == 0 {
            println!("{} {}", cnt, heap.len());
        }
        for action_name in &ACTION_NAME_LIST {
            let new_player = player.apply_action(action_name, actions_map);
            if let Ok(new_player) = new_player {
                if new_player.time <= MAX_TIME {
                    let new_h = calculate_hash(&new_player);
                    if !damages.contains_key(&new_h) {
                        damages.insert(new_h, new_player.damage);
                        history.insert(new_h, (h, *action_name));
                        heap.push(new_player);
                    } else if *damages.get(&new_h).unwrap() > new_player.damage {
                        damages.insert(new_h, new_player.damage);
                        history.insert(new_h, (h, *action_name));
                    }
                }
            }
        }
    }

    println!("Done, highest={}, count={}", ans, cnt);

    let mut current_h = best_h;
    let mut current_action: ActionName;
    let mut action_history = vec![];
    loop {
        (current_h, current_action) = *history.get(&current_h).unwrap();
        if let ActionName::None = current_action {
            break;
        } else {
            action_history.push(current_action);
        }
    }
    action_history.reverse();
    println!("{:#?}", action_history);
}
