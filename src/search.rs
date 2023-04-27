// use std::collections::BinaryHeap;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};

use crate::{ActionName, ActionsMap, Player};

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

const MAX_TIME: u32 = 7500;

pub fn search(actions_map: &ActionsMap) {
    let mut player = Player::default();
    player.assign_actions(actions_map);

    // let mut heap = BinaryHeap::new();
    // heap.push(player);

    let h = calculate_hash(&player);

    let mut queue = VecDeque::new();
    let mut damages = HashMap::new();
    let mut history = HashMap::new();
    damages.insert(h, player.damage);
    history.insert(h, (0u64, ActionName::None));
    queue.push_back(player);

    let mut cnt = 0;
    let mut ans = 0;
    let mut best_h: u64 = 0;

    while !queue.is_empty() {
        let mut player = queue.pop_front().unwrap();
        let h = calculate_hash(&player);
        player.damage = *damages.get(&h).unwrap();
        if player.damage > ans {
            ans = player.damage;
            best_h = h;
            println!("new best! {}", ans);
        }
        cnt += 1;
        if cnt % 10000 == 0 {
            println!("{} {}", cnt, queue.len());
        }
        for action_name in actions_map.map.keys() {
            let new_player = player.apply_action(action_name, actions_map);
            if let Ok(new_player) = new_player {
                if new_player.time <= MAX_TIME {
                    let new_h = calculate_hash(&new_player);
                    if !damages.contains_key(&new_h) {
                        damages.insert(new_h, new_player.damage);
                        history.insert(new_h, (h, action_name.clone()));
                        queue.push_back(new_player);
                    } else if *damages.get(&new_h).unwrap() > new_player.damage {
                        damages.insert(new_h, new_player.damage);
                        history.insert(new_h, (h, action_name.clone()));
                    }
                }
            }
        }
    }

    println!("Done, highest={}", ans);

    let mut current_h = best_h;
    let mut current_action: ActionName;
    let mut action_history = vec![];
    loop {
        (current_h, current_action) = history.get(&current_h).unwrap().clone();
        if let ActionName::None = current_action {
            break;
        } else {
            action_history.push(current_action.clone());
        }
    }
    action_history.reverse();
    println!("{:#?}", action_history);
}
