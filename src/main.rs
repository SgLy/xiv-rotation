use std::collections::HashMap;

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
#[derive(Debug, Clone, Copy, Hash)]
struct Status {
    time: u32,
    mp: u32,
    global_cool_down: u32,
    basic_combo: BasicCombo,
    blade_combo: BladeCombo,
    divine_might: DivineMight,

    fight_or_flight_cool_down: u32,
    fight_or_flight: u32,
    circle_of_scorn_cool_down: u32,
    circle_of_scorn: u32,
    requiescat_cool_down: u32,
    requiescat: u32,
    intervene_cool_down: u32,
    intervene: u32,
    intervene_stack: u32,
    expiacion_cool_down: u32,
    expiacion: u32,
    atonement_stack: u32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
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

#[derive(Debug, PartialEq, Eq, Hash)]
enum CoolDownType {
    Global,
    GlobalStandalone,
    OffGlobal,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Action {
    name: ActionName,
    cool_down_type: CoolDownType,
    cast: u32,
    recast: u32,
    mp_cost: u32,
    mp_restore: u32,
    potency: u32,
    secondary_potency: u32,
    max_charges: u32,
}

impl Status {
    pub fn tick(mut self, time: u32) -> Status {
        let new_time = self.time + time;
        if self.time / 30 != new_time / 30 {
            self.mp += 200;
        };
        self
    }
}

impl Action {
    fn apply(&self, status: &Status) -> Result<Status, ()> {
        if let CoolDownType::OffGlobal = self.cool_down_type {
            if status.global_cool_down > self.cast {
                return Err(());
            }
        } else if status.global_cool_down != 0 {
            return Err(());
        }
        let mut ret = *status;
        match self.name {
            ActionName::FastBlade => todo!(),
            ActionName::FightOrFlight => {
                if status.fight_or_flight_cool_down != 0 {
                    Err(())
                } else {
                    ret.fight_or_flight = 200;
                    ret.fight_or_flight_cool_down = self.recast;
                    Ok(ret.tick(self.cast))
                }
            }
            ActionName::RiotBlade => todo!(),
            ActionName::CircleOfScorn => todo!(),
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
        }
    }
}

fn main() {
    let mut actions = HashMap::new();
    actions.insert(
        ActionName::FastBlade,
        Action {
            name: ActionName::FastBlade,
            cool_down_type: CoolDownType::Global,
            cast: 6,
            recast: 25,
            mp_cost: 0,
            mp_restore: 0,
            potency: 2000,
            secondary_potency: 0,
            max_charges: 1,
        },
    );
    actions.insert(
        ActionName::FightOrFlight,
        Action {
            name: ActionName::FightOrFlight,
            cool_down_type: CoolDownType::OffGlobal,
            cast: 6,
            recast: 600,
            mp_cost: 0,
            mp_restore: 0,
            potency: 0,
            secondary_potency: 0,
            max_charges: 1,
        },
    );
    actions.insert(
        ActionName::RiotBlade,
        Action {
            name: ActionName::RiotBlade,
            cool_down_type: CoolDownType::Global,
            cast: 6,
            recast: 25,
            mp_cost: 0,
            mp_restore: 1000,
            potency: 1200,
            secondary_potency: 2800,
            max_charges: 1,
        },
    );
    actions.insert(
        ActionName::CircleOfScorn,
        Action {
            name: ActionName::CircleOfScorn,
            cool_down_type: CoolDownType::OffGlobal,
            cast: 6,
            recast: 300,
            mp_cost: 0,
            mp_restore: 0,
            potency: 1000 + 300 * 5,
            secondary_potency: 0,
            max_charges: 1,
        },
    );
    actions.insert(
        ActionName::GoringBlade,
        Action {
            name: ActionName::GoringBlade,
            cool_down_type: CoolDownType::GlobalStandalone,
            cast: 6,
            recast: 600,
            mp_cost: 0,
            mp_restore: 0,
            potency: 7000,
            secondary_potency: 0,
            max_charges: 1,
        },
    );
    actions.insert(
        ActionName::RoyalAuthority,
        Action {
            name: ActionName::RoyalAuthority,
            cool_down_type: CoolDownType::Global,
            cast: 6,
            recast: 25,
            mp_cost: 0,
            mp_restore: 0,
            potency: 1200,
            secondary_potency: 3800,
            max_charges: 1,
        },
    );
    actions.insert(
        ActionName::HolySpirit,
        Action {
            name: ActionName::HolySpirit,
            cool_down_type: CoolDownType::Global,
            cast: 15,
            recast: 25,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 4500,
            secondary_potency: 6500,
            max_charges: 1,
        },
    );
    actions.insert(
        ActionName::Requiescat,
        Action {
            name: ActionName::Requiescat,
            cool_down_type: CoolDownType::OffGlobal,
            cast: 6,
            recast: 600,
            mp_cost: 0,
            mp_restore: 0,
            potency: 3000,
            secondary_potency: 0,
            max_charges: 1,
        },
    );
    actions.insert(
        ActionName::Intervene,
        Action {
            name: ActionName::Intervene,
            cool_down_type: CoolDownType::OffGlobal,
            cast: 6,
            recast: 300,
            mp_cost: 0,
            mp_restore: 0,
            potency: 1500,
            secondary_potency: 0,
            max_charges: 2,
        },
    );
    actions.insert(
        ActionName::Atonement,
        Action {
            name: ActionName::Atonement,
            cool_down_type: CoolDownType::Global,
            cast: 6,
            recast: 25,
            mp_cost: 0,
            mp_restore: 400,
            potency: 3800,
            secondary_potency: 0,
            max_charges: 3,
        },
    );
    actions.insert(
        ActionName::Confiteor,
        Action {
            name: ActionName::Confiteor,
            cool_down_type: CoolDownType::Global,
            cast: 6,
            recast: 25,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 4000,
            secondary_potency: 9000,
            max_charges: 1,
        },
    );
    actions.insert(
        ActionName::Expiacion,
        Action {
            name: ActionName::Expiacion,
            cool_down_type: CoolDownType::OffGlobal,
            cast: 6,
            recast: 300,
            mp_cost: 0,
            mp_restore: 500,
            potency: 4500,
            secondary_potency: 0,
            max_charges: 1,
        },
    );
    actions.insert(
        ActionName::BladeOfFaith,
        Action {
            name: ActionName::BladeOfFaith,
            cool_down_type: CoolDownType::Global,
            cast: 6,
            recast: 25,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 2000,
            secondary_potency: 7000,
            max_charges: 1,
        },
    );
    actions.insert(
        ActionName::BladeOfTruth,
        Action {
            name: ActionName::BladeOfTruth,
            cool_down_type: CoolDownType::Global,
            cast: 6,
            recast: 25,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 3000,
            secondary_potency: 8000,
            max_charges: 1,
        },
    );
    actions.insert(
        ActionName::BladeOfValor,
        Action {
            name: ActionName::BladeOfValor,
            cool_down_type: CoolDownType::Global,
            cast: 6,
            recast: 25,
            mp_cost: 1000,
            mp_restore: 0,
            potency: 4000,
            secondary_potency: 9000,
            max_charges: 1,
        },
    );
}
