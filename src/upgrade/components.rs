use bevy::prelude::*;
use rand::Rng;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct UpgradeDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub upgrade_type: UpgradeType,
}

#[derive(Debug, Clone)]
pub enum UpgradeType {
    DamageUp(f32),
    BulletSpeedUp(f32),
    ShotSpeedUp(f32),
    PickupRangeUp(f32),
    MoveSpeedUp(f32),
    MultishotUp(u32),
    PiercingUp(u32),
}

impl UpgradeType {
    pub fn get_category(&self) -> &'static str {
        match self {
            UpgradeType::DamageUp(_) => "damage",
            UpgradeType::BulletSpeedUp(_) => "projectile_speed",
            UpgradeType::ShotSpeedUp(_) => "fire_rate",
            UpgradeType::PickupRangeUp(_) => "pickup_range",
            UpgradeType::MoveSpeedUp(_) => "move_speed",
            UpgradeType::MultishotUp(_) => "multishot",
            UpgradeType::PiercingUp(_) => "piercing",
        }
    }

    pub fn get_title(&self) -> &'static str {
        match self {
            UpgradeType::DamageUp(_) => "Damage Up",
            UpgradeType::BulletSpeedUp(_) => "Projectile Speed",
            UpgradeType::ShotSpeedUp(_) => "Fire Rate",
            UpgradeType::PickupRangeUp(_) => "Pickup Range Up",
            UpgradeType::MoveSpeedUp(_) => "Move Speed Up",
            UpgradeType::MultishotUp(_) => "Multishot",
            UpgradeType::PiercingUp(_) => "Piercing",
        }
    }

    pub fn get_description(&self) -> String {
        match self {
            UpgradeType::DamageUp(amount) => format!("+{:.1} Damage", amount),
            UpgradeType::BulletSpeedUp(amount) => format!("+{:.0} Projectile Speed", amount),
            UpgradeType::ShotSpeedUp(amount) => format!("-{:.2}s Fire Delay", amount),
            UpgradeType::PickupRangeUp(amount) => format!("+{:.0} Pickup Range", amount),
            UpgradeType::MoveSpeedUp(amount) => format!("+{:.0} Move Speed", amount),
            UpgradeType::MultishotUp(amount) => format!("+{} Projectiles", amount),
            UpgradeType::PiercingUp(amount) => format!("+{} Piercing", amount),
        }
    }
}

#[derive(Resource)]
pub struct UpgradePool {
    pub upgrades: Vec<UpgradeDefinition>,
}

impl UpgradePool {
    pub fn new() -> Self {
        Self {
            upgrades: vec![
                UpgradeDefinition {
                    id: "damage_small",
                    name: "Power Shot",
                    description: "Increase bullet damage",
                    upgrade_type: UpgradeType::DamageUp(2.0),
                },
                UpgradeDefinition {
                    id: "damage_large",
                    name: "Devastating Strike",
                    description: "Greatly increase bullet damage",
                    upgrade_type: UpgradeType::DamageUp(5.0),
                },
                UpgradeDefinition {
                    id: "shot_speed_small",
                    name: "Rapid Fire",
                    description: "Shoot more frequently",
                    upgrade_type: UpgradeType::ShotSpeedUp(0.05),
                },
                UpgradeDefinition {
                    id: "shot_speed_large",
                    name: "Machine Gun",
                    description: "Shoot much more frequently",
                    upgrade_type: UpgradeType::ShotSpeedUp(0.15),
                },
                UpgradeDefinition {
                    id: "bullet_speed_small",
                    name: "Swift Shot",
                    description: "Bullets travel faster",
                    upgrade_type: UpgradeType::BulletSpeedUp(100.0),
                },
                UpgradeDefinition {
                    id: "bullet_speed_large",
                    name: "Lightning Rounds",
                    description: "Bullets travel much faster",
                    upgrade_type: UpgradeType::BulletSpeedUp(200.0),
                },
                UpgradeDefinition {
                    id: "pickup_range_small",
                    name: "Magnet",
                    description: "Increase item pickup range",
                    upgrade_type: UpgradeType::PickupRangeUp(20.0),
                },
                UpgradeDefinition {
                    id: "pickup_range_large",
                    name: "Strong Magnet",
                    description: "Greatly increase item pickup range",
                    upgrade_type: UpgradeType::PickupRangeUp(50.0),
                },
                UpgradeDefinition {
                    id: "move_speed_small",
                    name: "Fleet Foot",
                    description: "Increase movement speed",
                    upgrade_type: UpgradeType::MoveSpeedUp(30.0),
                },
                UpgradeDefinition {
                    id: "move_speed_large",
                    name: "Lightning Speed",
                    description: "Greatly increase movement speed",
                    upgrade_type: UpgradeType::MoveSpeedUp(70.0),
                },
                UpgradeDefinition {
                    id: "multishot_small",
                    name: "Double Shot",
                    description: "Shoot an additional projectile",
                    upgrade_type: UpgradeType::MultishotUp(1),
                },
                UpgradeDefinition {
                    id: "multishot_large",
                    name: "Spread Shot",
                    description: "Shoot several additional projectiles",
                    upgrade_type: UpgradeType::MultishotUp(2),
                },
                UpgradeDefinition {
                    id: "piercing_small",
                    name: "Penetrating Shot",
                    description: "Bullets pierce through enemies",
                    upgrade_type: UpgradeType::PiercingUp(1),
                },
                UpgradeDefinition {
                    id: "piercing_large",
                    name: "Armor Piercing",
                    description: "Bullets pierce through multiple enemies",
                    upgrade_type: UpgradeType::PiercingUp(2),
                },
            ],
        }
    }

    pub fn get_random_upgrades(&self, count: usize) -> Vec<UpgradeDefinition> {
        let mut rng = rand::thread_rng();
        let mut upgrades = Vec::new();
        let mut used_categories = HashSet::new();
        let mut available_upgrades = self.upgrades.clone();

        // Shuffle to ensure randomness within categories
        use rand::seq::SliceRandom;
        available_upgrades.shuffle(&mut rng);

        // Select upgrades ensuring no duplicate categories
        for upgrade in available_upgrades {
            if upgrades.len() >= count {
                break;
            }

            let category = upgrade.upgrade_type.get_category();
            if !used_categories.contains(category) {
                used_categories.insert(category);
                upgrades.push(upgrade);
            }
        }

        // If we couldn't find enough unique categories, fill with random ones
        while upgrades.len() < count && upgrades.len() < self.upgrades.len() {
            let index = rng.gen_range(0..self.upgrades.len());
            let upgrade = &self.upgrades[index];
            if !upgrades.iter().any(|u| u.id == upgrade.id) {
                upgrades.push(upgrade.clone());
            }
        }

        upgrades
    }
}

#[derive(Component)]
pub struct ShownUpgrades {
    pub upgrades: Vec<UpgradeDefinition>,
}

#[derive(Component)]
pub struct ChosenUpgrade {
    pub upgrade: UpgradeDefinition,
}
