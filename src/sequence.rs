use enum_map::EnumMap;

use crate::{Action, ActionName, CooldownType, Player};

pub fn play_sequence(actions_map: &EnumMap<ActionName, Action>) {
    let mut player = Player::default();
    player.assign_actions(actions_map);

    let action_sequence: Vec<ActionName> = vec![
        ActionName::FastBlade,
        ActionName::RiotBlade,
        ActionName::RoyalAuthority,
        ActionName::FightOrFlight,
        ActionName::Requiescat,
        ActionName::GoringBlade,
        ActionName::Expiacion,
        ActionName::CircleOfScorn,
        ActionName::Confiteor,
        ActionName::Intervene,
        ActionName::BladeOfFaith,
        ActionName::Intervene,
        ActionName::BladeOfTruth,
        ActionName::BladeOfValor,
        ActionName::HolySpirit,
        ActionName::Atonement,
        ActionName::Atonement,
        ActionName::Atonement,
        ActionName::FastBlade,
        ActionName::RiotBlade,
        ActionName::RoyalAuthority,
        ActionName::Atonement,
        ActionName::Expiacion,
        ActionName::CircleOfScorn,
        ActionName::Atonement,
        ActionName::Atonement,
        ActionName::HolySpirit,
        ActionName::FastBlade,
        ActionName::RiotBlade,
        ActionName::RoyalAuthority,
        ActionName::Atonement,
        ActionName::Atonement,
    ];

    for action in &action_sequence {
        let last_time = player.time;
        let last_damage = player.damage;
        player = player.apply_action(action, actions_map).unwrap();
        if let CooldownType::OffGlobal = actions_map[*action].cooldown_type {
            print!("  ");
        }
        println!(
            "{:?} -> time: {} (+{}), damage: {} (+{}), mp: {}, intervene: {} / {}",
            action,
            player.time,
            player.time - last_time,
            (player.damage as f64) / (player.time as f64) * 1000f64,
            player.damage - last_damage,
            player.mp,
            player.action_status[ActionName::Intervene].charges,
            player.action_status[ActionName::Intervene].cooldown,
        );
    }
}
