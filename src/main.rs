use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Copy, Hash)]
enum BasicCombo {
    None,
    FastBlade,
    RiotBlade,
}
#[derive(Debug, Clone, Copy, Hash)]
enum BladeCombo {
    None,
    Confiteor,
    BladeOfFaith,
    BladeOfTruth,
}
#[derive(Debug, Clone, Copy, Hash)]
enum DivineMight {
    None,
    Ready,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum ActionName {
    FastBlade,
    FightOrFlight,
    RiotBlade,
    CircleOfScorn,
    GoringBlade,
    RoyalAuthority,
    HolySpirit,
    Requiescat,
    Intervene,
    Atonement,
    Confiteor,
    Expiacion,
    BladeOfFaith,
    BladeOfTruth,
    BladeOfValor,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum CooldownType {
    Global,
    GlobalStandalone,
    OffGlobal,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Action {
    name: ActionName,
    cooldown_type: CooldownType,
    cast: u32,
    recast: u32,
    max_duration: u32,
    max_count: u32,
    mp_cost: u32,
    mp_restore: u32,
    potency: u32,
    secondary_potency: u32,
    max_charges: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct ActionStatus {
    name: ActionName,
    cooldown: u32,
    duration: u32,
    charges: u32,
    count: u32,
}

#[derive(Debug, Clone)]
struct Player {
    time: u32,
    mp: u32,
    damage: u32,
    global_cooldown: u32,
    basic_combo: BasicCombo,
    blade_combo: BladeCombo,
    divine_might: DivineMight,
    action_status: HashMap<ActionName, ActionStatus>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            time: 60000,
            mp: 10000,
            damage: 0,
            global_cooldown: 0,
            basic_combo: BasicCombo::None,
            blade_combo: BladeCombo::None,
            divine_might: DivineMight::None,
            action_status: HashMap::new(),
        }
    }
}

#[inline(always)]
fn sub_to_zero(a: u32, b: u32) -> u32 {
    if a <= b {
        0
    } else {
        a - b
    }
}

impl ActionStatus {
    pub fn tick(mut self, time: u32, action: &Action) -> ActionStatus {
        if self.charges == action.max_charges {
            // do nothing
        } else if self.cooldown > time {
            self.cooldown -= time
        } else if self.charges < action.max_charges - 1 {
            self.charges += 1;
            self.cooldown = action.recast + self.cooldown - time;
        } else if self.charges == action.max_charges - 1 {
            self.cooldown = 0;
            self.charges = action.max_charges;
        }
        self.duration = sub_to_zero(self.duration, time);
        if self.duration == 0 {
            self.count = 0;
        }
        self
    }
}

impl Player {
    pub fn add_action(mut self, action: &Action) {
        self.action_status.insert(
            action.name,
            ActionStatus {
                name: action.name,
                cooldown: 0,
                duration: 0,
                charges: action.max_charges,
                count: 0,
            },
        );
    }

    pub fn recover_mp(mut self, mp: u32) {
        self.mp += mp;
        if self.mp > 10000 {
            self.mp = 10000
        };
    }

    pub fn tick(mut self, time: u32, actions_map: &ActionsMap) -> Option<Self> {
        let new_time = self.time + time;
        self.recover_mp(((new_time / 3000) - self.time / 3000) * 200);
        self.time = new_time;
        self.global_cooldown = sub_to_zero(self.global_cooldown, time);
        for (action_name, action) in &actions_map.map {
            let mut action_status = self.action_status.get(action_name).unwrap();
            action_status.tick(time, &action);
            if action_status.cooldown > self.time {
                return None;
            }
        }
        Some(self)
    }

    pub fn hit(mut self, damage: u32) {
        let fight_or_flight = self.action_status.get(&ActionName::FightOrFlight).unwrap();
        self.damage += if fight_or_flight.duration > 0 {
            damage / 4 * 5
        } else {
            damage
        };
    }

    pub fn apply_action(self, action_name: &ActionName, actions_map: &ActionsMap) -> Option<Self> {
        let action = actions_map.map.get(action_name).unwrap();
        let mut action_status = self.action_status.get(action_name).unwrap();
        let mut ret = self.clone();

        match action.cooldown_type {
            CooldownType::Global | CooldownType::GlobalStandalone => {
                if ret.global_cooldown != 0 {
                    ret.tick(ret.global_cooldown, actions_map)
                } else {
                    Some(ret)
                }
            }
            _ => Some(ret),
        }?;
        match action.cooldown_type {
            CooldownType::GlobalStandalone | CooldownType::OffGlobal => {
                if action_status.cooldown != 0 {
                    ret.tick(action_status.cooldown, actions_map)
                } else {
                    Some(ret)
                }
            }
            _ => Some(ret),
        }?;

        while ret.mp < action.mp_cost {
            ret.tick(((ret.time / 3000) + 1) * 3000 - ret.time, actions_map);
        }
        ret.hit(action.potency);
        action_status.cooldown = action.recast;

        match action.cooldown_type {
            CooldownType::Global | CooldownType::GlobalStandalone => {
                ret.global_cooldown = 2500;
            }
            _ => {}
        };

        match action.name {
            ActionName::FastBlade => {
                ret.basic_combo = BasicCombo::FastBlade;
                ret.blade_combo = BladeCombo::None;
            }
            ActionName::FightOrFlight => {
                ret.hit(action.potency);
                action_status.duration = action.max_duration;
                action_status.cooldown = action.recast;
            }
            ActionName::RiotBlade => {
                if let BasicCombo::FastBlade = ret.basic_combo {
                    self.hit(action.secondary_potency);
                    ret.basic_combo = BasicCombo::RiotBlade;
                } else {
                    self.hit(action.potency);
                }
                ret.blade_combo = BladeCombo::None;
                ret.recover_mp(action.mp_restore);
            }
            ActionName::CircleOfScorn => {
                ret.hit(action.potency);
                action_status.duration = action.max_duration;
                action_status.cooldown = action.recast;
            }
            ActionName::GoringBlade => todo!(),
            ActionName::RoyalAuthority => todo!(),
            ActionName::HolySpirit => todo!(),
            ActionName::Requiescat => todo!(),
            ActionName::Intervene => todo!(),
            ActionName::Atonement => todo!(),
            ActionName::Confiteor => todo!(),
            ActionName::Expiacion => todo!(),
            ActionName::BladeOfFaith => todo!(),
            ActionName::BladeOfTruth => todo!(),
            ActionName::BladeOfValor => todo!(),
        };

        ret.tick(action.cast, actions_map)
    }
}

#[derive(Debug, Default)]
struct ActionsMap {
    map: HashMap<ActionName, Action>,
}

impl ActionsMap {
    pub fn add(mut self, action: Action) -> Self {
        self.map.insert(action.name, action);
        self
    }
}

fn main() {
    let mut player = Player::default();
    let mut actions_map = ActionsMap::default()
        .add(Action {
            name: ActionName::FastBlade,
            cooldown_type: CooldownType::Global,
            cast: 600,
            recast: 2500,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 0,
            potency: 2000,
            secondary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::FightOrFlight,
            cooldown_type: CooldownType::OffGlobal,
            cast: 600,
            recast: 60000,
            max_duration: 20000,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 0,
            potency: 0,
            secondary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::RiotBlade,
            cooldown_type: CooldownType::Global,
            cast: 600,
            recast: 2500,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 1000,
            potency: 1200,
            secondary_potency: 2800,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::CircleOfScorn,
            cooldown_type: CooldownType::OffGlobal,
            cast: 600,
            recast: 30000,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 0,
            potency: 1000 + 300 * 5,
            secondary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::GoringBlade,
            cooldown_type: CooldownType::GlobalStandalone,
            cast: 600,
            recast: 60000,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 0,
            potency: 7000,
            secondary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::RoyalAuthority,
            cooldown_type: CooldownType::Global,
            cast: 600,
            recast: 2500,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 0,
            potency: 1200,
            secondary_potency: 3800,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::HolySpirit,
            cooldown_type: CooldownType::Global,
            cast: 1500,
            recast: 2500,
            max_duration: 0,
            max_count: 0,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 4500,
            secondary_potency: 6500,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::Requiescat,
            cooldown_type: CooldownType::OffGlobal,
            cast: 600,
            recast: 60000,
            max_duration: 30000,
            max_count: 5,
            mp_cost: 0,
            mp_restore: 0,
            potency: 3000,
            secondary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::Intervene,
            cooldown_type: CooldownType::OffGlobal,
            cast: 600,
            recast: 30000,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 0,
            potency: 1500,
            secondary_potency: 0,
            max_charges: 2,
        })
        .add(Action {
            name: ActionName::Atonement,
            cooldown_type: CooldownType::Global,
            cast: 600,
            recast: 2500,
            max_duration: 30000,
            max_count: 3,
            mp_cost: 0,
            mp_restore: 400,
            potency: 3800,
            secondary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::Confiteor,
            cooldown_type: CooldownType::Global,
            cast: 600,
            recast: 2500,
            max_duration: 0,
            max_count: 0,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 4000,
            secondary_potency: 9000,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::Expiacion,
            cooldown_type: CooldownType::OffGlobal,
            cast: 600,
            recast: 30000,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 500,
            potency: 4500,
            secondary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::BladeOfFaith,
            cooldown_type: CooldownType::Global,
            cast: 600,
            recast: 2500,
            max_duration: 0,
            max_count: 0,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 2000,
            secondary_potency: 7000,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::BladeOfTruth,
            cooldown_type: CooldownType::Global,
            cast: 600,
            recast: 2500,
            max_duration: 0,
            max_count: 0,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 3000,
            secondary_potency: 8000,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::BladeOfValor,
            cooldown_type: CooldownType::Global,
            cast: 600,
            recast: 2500,
            max_duration: 0,
            max_count: 0,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 4000,
            secondary_potency: 9000,
            max_charges: 1,
        });

    let action_sequence: Vec<ActionName> = vec![
        ActionName::FastBlade,
        ActionName::RiotBlade,
        ActionName::RoyalAuthority,
        ActionName::FightOrFlight,
        ActionName::Requiescat,
        ActionName::HolySpirit,
        ActionName::Expiacion,
        ActionName::CircleOfScorn,
        ActionName::BladeOfFaith,
        ActionName::BladeOfTruth,
        ActionName::BladeOfValor,
    ];

    for action in action_sequence {
        player = player.apply_action(&action, &actions_map).unwrap();
        println!("{:?} -> {:?}", action, player);
    }
}
