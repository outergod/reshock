use bevy_ecs::prelude::*;

use crate::game::{component::*, Status, *};

fn multiplier(attack: &AttackKind, vulnerable: &VulnerableKind) -> u8 {
    match (attack, vulnerable) {
        (_, VulnerableKind::None) => 0,
        (AttackKind::Kinetic, _) | (AttackKind::Beam, _) => 1,
        (AttackKind::Needle, VulnerableKind::Avian) => 2,
        (AttackKind::Needle, VulnerableKind::GorillaTiger) => 2,
        (AttackKind::Needle, VulnerableKind::Humanoid) => 2,
        (AttackKind::Needle, VulnerableKind::Invisible) => 1,
        (AttackKind::Needle, VulnerableKind::Plant) => 2,
        (AttackKind::Needle, VulnerableKind::Virus) => 1,
        (AttackKind::Needle, VulnerableKind::ZeroGrav) => 1,
        (AttackKind::Needle, VulnerableKind::Robot) => 0,
        (AttackKind::Needle, VulnerableKind::RoboticCyborg) => 0,
        (AttackKind::Needle, VulnerableKind::HumanoidCyborg) => 1,
        (AttackKind::Tranquilizer, VulnerableKind::Avian) => 1,
        (AttackKind::Tranquilizer, VulnerableKind::GorillaTiger) => 1,
        (AttackKind::Tranquilizer, VulnerableKind::Humanoid) => 1,
        (AttackKind::Tranquilizer, VulnerableKind::Invisible) => 1,
        (AttackKind::Tranquilizer, VulnerableKind::Plant) => 1,
        (AttackKind::Tranquilizer, VulnerableKind::Virus) => 1,
        (AttackKind::Tranquilizer, VulnerableKind::ZeroGrav) => 1,
        (AttackKind::Tranquilizer, VulnerableKind::Robot) => 1,
        (AttackKind::Tranquilizer, VulnerableKind::RoboticCyborg) => 0,
        (AttackKind::Tranquilizer, VulnerableKind::HumanoidCyborg) => 1,
        (AttackKind::Magnetic, VulnerableKind::Avian) => 0,
        (AttackKind::Magnetic, VulnerableKind::GorillaTiger) => 0,
        (AttackKind::Magnetic, VulnerableKind::Humanoid) => 0,
        (AttackKind::Magnetic, VulnerableKind::Invisible) => 0,
        (AttackKind::Magnetic, VulnerableKind::Plant) => 0,
        (AttackKind::Magnetic, VulnerableKind::Virus) => 0,
        (AttackKind::Magnetic, VulnerableKind::ZeroGrav) => 0,
        (AttackKind::Magnetic, VulnerableKind::Robot) => 4,
        (AttackKind::Magnetic, VulnerableKind::RoboticCyborg) => 2,
        (AttackKind::Magnetic, VulnerableKind::HumanoidCyborg) => 2,
        (AttackKind::Gas, VulnerableKind::Avian) => 1,
        (AttackKind::Gas, VulnerableKind::GorillaTiger) => 1,
        (AttackKind::Gas, VulnerableKind::Humanoid) => 1,
        (AttackKind::Gas, VulnerableKind::Invisible) => 1,
        (AttackKind::Gas, VulnerableKind::Plant) => 1,
        (AttackKind::Gas, VulnerableKind::Virus) => 1,
        (AttackKind::Gas, VulnerableKind::ZeroGrav) => 1,
        (AttackKind::Gas, VulnerableKind::Robot) => 0,
        (AttackKind::Gas, VulnerableKind::RoboticCyborg) => 1,
        (AttackKind::Gas, VulnerableKind::HumanoidCyborg) => 1,
    }
}

pub fn behavior(
    mut action: ResMut<ActiveAction>,
    mut reactions: ResMut<Reactions>,
    vulnerables: Query<&Vulnerable>,
    descriptions: Query<&Description>,
) -> Status {
    let DamageAction {
        actor,
        target,
        weapon,
        mut damage,
    } = match action.0.as_mut() {
        Some(Action::Damage(it)) => it,
        _ => return Status::Continue,
    };

    let mut rng = thread_rng();

    let vulnerable = vulnerables.get(*target).unwrap();

    let penetration = {
        let random = rng.gen_range(0.9..=1.1);
        (damage.penetration as f32 * random) as u8
    };
    let armor = vulnerable.armor.saturating_sub(penetration);
    let multiplier = multiplier(&damage.attack, &vulnerable.kind);
    let crit = damage.offense.saturating_sub(vulnerable.defense) as f32 / 5.0;

    let mut amount = damage.amount as f32;

    amount -= armor as f32;
    amount *= multiplier as f32;

    if crit > rng.gen() {
        let multiplier = rng.gen_range(1.33..=4.0);
        amount *= multiplier;
    }

    let random = rng.gen_range(0.9..=1.1);
    amount *= random;

    damage.amount = amount as u16;

    if let (Ok(actor), Ok(target), Ok(weapon)) = (
        descriptions.get(*actor),
        descriptions.get(*target),
        descriptions.get(*weapon),
    ) {
        let log = Action::Log(format!(
            "{} strikes {} with {}, dealing {} damage",
            actor, target, weapon, damage.amount
        ));

        reactions.0.push(log);
    }

    let action = Action::HealthLoss(HealthLossAction {
        actor: *target,
        amount: amount as u16,
    });

    reactions.0.push(action);

    Status::Continue
}
