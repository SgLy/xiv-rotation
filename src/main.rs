use std::{cmp, collections::HashMap, fmt::Debug, hash::Hash};

const ROTATION_DURATION: u32 = 60000;
const ANIMATION_LOCK: u32 = 800;
const GLOBAL_COOLDOWN: u32 = 2500;

#[derive(Debug, Clone, Hash)]
enum BasicCombo {
    None,
    FastBlade,
    RiotBlade,
}
#[derive(Debug, Clone, Hash)]
enum BladeCombo {
    None,
    Confiteor,
    BladeOfFaith,
    BladeOfTruth,
}

#[derive(Debug, Clone, Hash)]
enum DivineMight {
    None,
    Ready,
}

#[derive(Debug, Clone, Hash)]
enum Confiteor {
    None,
    Ready,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum CooldownType {
    Global,
    GlobalStandalone,
    OffGlobal,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    tertiary_potency: u32,
    max_charges: u32,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct ActionStatus {
    name: ActionName,
    cooldown: u32,
    duration: u32,
    charges: u32,
    count: u32,
}

impl Debug for ActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
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
    confiteor: Confiteor,
    action_status: HashMap<ActionName, ActionStatus>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            time: ROTATION_DURATION,
            mp: 10000,
            damage: 0,
            global_cooldown: 0,
            basic_combo: BasicCombo::None,
            blade_combo: BladeCombo::None,
            divine_might: DivineMight::None,
            confiteor: Confiteor::None,
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
    pub fn tick(&mut self, time: u32, action: &Action) {
        self.duration = sub_to_zero(self.duration, time);
        if self.duration == 0 {
            self.count = 0;
        };
        self.cooldown = sub_to_zero(self.cooldown, time);

        let mut time = time;
        while self.charges < action.max_charges && time > 0 {
            if self.cooldown > time {
                self.cooldown -= time;
                time = 0;
            } else {
                time -= self.cooldown;
                self.charges += 1;
                self.cooldown = if self.charges == action.max_charges {
                    0
                } else {
                    action.recast
                }
            }
        }
    }
}

#[derive(Debug)]
enum ActionApplyError {
    NotReady,
    CooldownOvertime,
    MpNotEnough,
    CastOvertime,
}

impl Player {
    pub fn assign_actions(&mut self, actions_map: &ActionsMap) {
        for (action_name, action) in &actions_map.map {
            self.action_status.insert(
                action_name.clone(),
                ActionStatus {
                    name: action_name.clone(),
                    cooldown: 0,
                    duration: 0,
                    charges: action.max_charges,
                    count: 0,
                },
            );
        }
    }

    pub fn recover_mp(&mut self, mp: u32) {
        self.mp += mp;
        if self.mp > 10000 {
            self.mp = 10000
        };
    }

    pub fn tick(&mut self, time: u32, actions_map: &ActionsMap) -> Option<()> {
        if self.time < time {
            return None;
        }
        let new_time = self.time - time;
        self.recover_mp(((self.time / 3000) - (new_time / 3000)) * 200);
        self.time = new_time;
        self.global_cooldown = sub_to_zero(self.global_cooldown, time);
        for (action_name, action) in &actions_map.map {
            let action_status = self.action_status.get_mut(action_name).unwrap();
            action_status.tick(time, action);
            // if action_status.cooldown > self.time {
            //     return None;
            // }
        }
        Some(())
    }

    pub fn hit(&mut self, damage: u32) {
        let fight_or_flight = self.action_status.get(&ActionName::FightOrFlight).unwrap();
        self.damage += if fight_or_flight.duration > 0 {
            damage / 4 * 5
        } else {
            damage
        };
    }

    pub fn apply_action(
        self,
        action_name: &ActionName,
        actions_map: &ActionsMap,
    ) -> Result<Self, ActionApplyError> {
        let action = actions_map.map.get(action_name).unwrap();
        let mut ret = self;

        macro_rules! action_status_mut {
            () => {
                ret.action_status.get_mut(action_name).unwrap()
            };
        }

        let mut wait_time = match action.cooldown_type {
            CooldownType::Global | CooldownType::GlobalStandalone => ret.global_cooldown,
            _ => 0,
        };
        if action_status_mut!().charges == 0 {
            wait_time = cmp::max(
                wait_time,
                match action.cooldown_type {
                    CooldownType::GlobalStandalone | CooldownType::OffGlobal => {
                        action_status_mut!().cooldown
                    }
                    _ => 0,
                },
            );
        }

        if wait_time > 0 && ret.tick(wait_time, actions_map).is_none() {
            return Err(ActionApplyError::CooldownOvertime);
        }

        while ret.mp < action.mp_cost && ret.time >= 3000 {
            ret.tick((ret.time - (ret.time / 3000) - 1) * 3000, actions_map);
        }
        if ret.mp < action.mp_cost {
            return Err(ActionApplyError::MpNotEnough);
        }
        ret.mp -= action.mp_cost;
        ret.recover_mp(action.mp_restore);

        {
            let mut action_status = action_status_mut!();
            if action_status.charges == action.max_charges {
                action_status.cooldown = action.recast;
            }
            action_status.charges -= 1;
            action_status.duration = action.max_duration;
        }

        match action.cooldown_type {
            CooldownType::Global | CooldownType::GlobalStandalone => {
                ret.global_cooldown = GLOBAL_COOLDOWN;
            }
            _ => {}
        };

        let mut potency = action.potency;
        let mut cast = action.cast;

        match action.name {
            ActionName::FastBlade => {
                ret.basic_combo = BasicCombo::FastBlade;
                ret.blade_combo = BladeCombo::None;
            }
            ActionName::FightOrFlight => {}
            ActionName::RiotBlade => {
                if let BasicCombo::FastBlade = ret.basic_combo {
                    potency = action.secondary_potency;
                    ret.basic_combo = BasicCombo::RiotBlade;
                } else {
                    ret.basic_combo = BasicCombo::None;
                }
                ret.blade_combo = BladeCombo::None;
            }
            ActionName::CircleOfScorn => {}
            ActionName::GoringBlade => {}
            ActionName::RoyalAuthority => {
                if let BasicCombo::RiotBlade = ret.basic_combo {
                    potency = action.secondary_potency;
                    let atonement_status =
                        ret.action_status.get_mut(&ActionName::Atonement).unwrap();
                    let atonement = actions_map.map.get(&ActionName::Atonement).unwrap();
                    atonement_status.count = atonement.max_count;
                    atonement_status.duration = atonement.max_duration;
                    ret.divine_might = DivineMight::Ready;
                }
                ret.basic_combo = BasicCombo::None;
                ret.blade_combo = BladeCombo::None;
            }
            ActionName::HolySpirit => {
                let requiescat_status = ret.action_status.get_mut(&ActionName::Requiescat).unwrap();
                if let DivineMight::Ready = ret.divine_might {
                    potency = action.secondary_potency;
                    cast = ANIMATION_LOCK;
                    ret.divine_might = DivineMight::None;
                } else if requiescat_status.count > 0 {
                    requiescat_status.count -= 1;
                    potency = action.tertiary_potency;
                    cast = ANIMATION_LOCK;
                }
            }
            ActionName::Requiescat => {
                action_status_mut!().count = action.max_count;
                action_status_mut!().duration = action.max_duration;
                ret.confiteor = Confiteor::Ready;
            }
            ActionName::Intervene => {}
            ActionName::Atonement => {
                if action_status_mut!().count == 0 {
                    return Err(ActionApplyError::NotReady);
                }
                action_status_mut!().count -= 1;
            }
            ActionName::Confiteor => {
                let requiescat_status = ret.action_status.get_mut(&ActionName::Requiescat).unwrap();
                if let Confiteor::None = ret.confiteor {
                    return Err(ActionApplyError::NotReady);
                }
                ret.confiteor = Confiteor::None;
                ret.blade_combo = BladeCombo::Confiteor;
                ret.basic_combo = BasicCombo::None;
                if requiescat_status.count > 0 {
                    requiescat_status.count -= 1;
                    potency = action.secondary_potency;
                }
            }
            ActionName::Expiacion => {}
            ActionName::BladeOfFaith => {
                let requiescat_status = ret.action_status.get_mut(&ActionName::Requiescat).unwrap();
                if let BladeCombo::Confiteor = ret.blade_combo {
                    if requiescat_status.count > 0 {
                        requiescat_status.count -= 1;
                        potency = action.secondary_potency;
                    }
                    ret.blade_combo = BladeCombo::BladeOfFaith;
                    ret.basic_combo = BasicCombo::None;
                } else {
                    return Err(ActionApplyError::NotReady);
                }
            }
            ActionName::BladeOfTruth => {
                let requiescat_status = ret.action_status.get_mut(&ActionName::Requiescat).unwrap();
                if let BladeCombo::BladeOfFaith = ret.blade_combo {
                    if requiescat_status.count > 0 {
                        requiescat_status.count -= 1;
                        potency = action.secondary_potency;
                    }
                    ret.blade_combo = BladeCombo::BladeOfTruth;
                    ret.basic_combo = BasicCombo::None;
                } else {
                }
            }
            ActionName::BladeOfValor => {
                let requiescat_status = ret.action_status.get_mut(&ActionName::Requiescat).unwrap();
                if let BladeCombo::BladeOfTruth = ret.blade_combo {
                    if requiescat_status.count > 0 {
                        requiescat_status.count -= 1;
                        potency = action.secondary_potency;
                    }
                    ret.blade_combo = BladeCombo::None;
                    ret.basic_combo = BasicCombo::None;
                } else {
                    return Err(ActionApplyError::NotReady);
                }
            }
        };

        ret.hit(potency);
        if ret.tick(cast, actions_map).is_none() {
            return Err(ActionApplyError::CastOvertime);
        }
        Ok(ret)
    }
}

#[derive(Debug, Default)]
struct ActionsMap {
    map: HashMap<ActionName, Action>,
}

impl ActionsMap {
    pub fn add(mut self, action: Action) -> Self {
        self.map.insert(action.name.clone(), action);
        self
    }
}

fn main() {
    let mut player = Player::default();
    let actions_map = ActionsMap::default()
        .add(Action {
            name: ActionName::FastBlade,
            cooldown_type: CooldownType::Global,
            cast: ANIMATION_LOCK,
            recast: GLOBAL_COOLDOWN,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 0,
            potency: 2000,
            secondary_potency: 0,
            tertiary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::FightOrFlight,
            cooldown_type: CooldownType::OffGlobal,
            cast: ANIMATION_LOCK,
            recast: 60000,
            max_duration: 20000,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 0,
            potency: 0,
            secondary_potency: 0,
            tertiary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::RiotBlade,
            cooldown_type: CooldownType::Global,
            cast: ANIMATION_LOCK,
            recast: GLOBAL_COOLDOWN,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 1000,
            potency: 1200,
            secondary_potency: 2800,
            tertiary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::CircleOfScorn,
            cooldown_type: CooldownType::OffGlobal,
            cast: ANIMATION_LOCK,
            recast: 30000,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 0,
            potency: 1000 + 300 * 5,
            secondary_potency: 0,
            tertiary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::GoringBlade,
            cooldown_type: CooldownType::GlobalStandalone,
            cast: ANIMATION_LOCK,
            recast: 60000,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 0,
            potency: 7000,
            secondary_potency: 0,
            tertiary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::RoyalAuthority,
            cooldown_type: CooldownType::Global,
            cast: ANIMATION_LOCK,
            recast: GLOBAL_COOLDOWN,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 0,
            potency: 1200,
            secondary_potency: 3800,
            tertiary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::HolySpirit,
            cooldown_type: CooldownType::Global,
            cast: 1500,
            recast: GLOBAL_COOLDOWN,
            max_duration: 0,
            max_count: 0,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 3000,
            secondary_potency: 4500,
            tertiary_potency: 6500,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::Requiescat,
            cooldown_type: CooldownType::OffGlobal,
            cast: ANIMATION_LOCK,
            recast: 60000,
            max_duration: 30000,
            max_count: 4,
            mp_cost: 0,
            mp_restore: 0,
            potency: 3000,
            secondary_potency: 0,
            tertiary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::Intervene,
            cooldown_type: CooldownType::OffGlobal,
            cast: ANIMATION_LOCK,
            recast: 30000,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 0,
            potency: 1500,
            secondary_potency: 0,
            tertiary_potency: 0,
            max_charges: 2,
        })
        .add(Action {
            name: ActionName::Atonement,
            cooldown_type: CooldownType::Global,
            cast: ANIMATION_LOCK,
            recast: GLOBAL_COOLDOWN,
            max_duration: 30000,
            max_count: 3,
            mp_cost: 0,
            mp_restore: 400,
            potency: 3800,
            secondary_potency: 0,
            tertiary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::Confiteor,
            cooldown_type: CooldownType::Global,
            cast: ANIMATION_LOCK,
            recast: GLOBAL_COOLDOWN,
            max_duration: 0,
            max_count: 0,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 4000,
            secondary_potency: 9000,
            tertiary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::Expiacion,
            cooldown_type: CooldownType::OffGlobal,
            cast: ANIMATION_LOCK,
            recast: 30000,
            max_duration: 0,
            max_count: 0,
            mp_cost: 0,
            mp_restore: 500,
            potency: 4500,
            secondary_potency: 0,
            tertiary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::BladeOfFaith,
            cooldown_type: CooldownType::Global,
            cast: ANIMATION_LOCK,
            recast: GLOBAL_COOLDOWN,
            max_duration: 0,
            max_count: 0,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 2000,
            secondary_potency: 7000,
            tertiary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::BladeOfTruth,
            cooldown_type: CooldownType::Global,
            cast: ANIMATION_LOCK,
            recast: GLOBAL_COOLDOWN,
            max_duration: 0,
            max_count: 0,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 3000,
            secondary_potency: 8000,
            tertiary_potency: 0,
            max_charges: 1,
        })
        .add(Action {
            name: ActionName::BladeOfValor,
            cooldown_type: CooldownType::Global,
            cast: ANIMATION_LOCK,
            recast: GLOBAL_COOLDOWN,
            max_duration: 0,
            max_count: 0,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 4000,
            secondary_potency: 9000,
            tertiary_potency: 0,
            max_charges: 1,
        });

    player.assign_actions(&actions_map);

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
    ];

    for action in action_sequence {
        let last_time = player.time;
        let last_damage = player.damage;
        player = player.apply_action(&action, &actions_map).unwrap();
        if let CooldownType::OffGlobal = actions_map.map.get(&action).unwrap().cooldown_type {
            print!("  ");
        }
        println!(
            "{:?} -> time: {} (+{}), damage: {} (+{}), mp: {}, fof: {} / {}",
            action,
            ROTATION_DURATION - player.time,
            last_time - player.time,
            player.damage,
            player.damage - last_damage,
            player.mp,
            player
                .action_status
                .get(&ActionName::FightOrFlight)
                .unwrap()
                .duration,
            player
                .action_status
                .get(&ActionName::FightOrFlight)
                .unwrap()
                .cooldown,
        );
    }
}
