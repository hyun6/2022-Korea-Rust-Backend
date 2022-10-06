fn minus_safe(target: u32, amount: u32) -> u32 {
    if target >= amount {
        target - amount
    } else {
        0
    }
}

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    const CAN_USE_MANA_LEVEL: u32 = 10;

    pub fn revive(&self) -> Option<Player> {
        // check player is dead
        // if player health is over than 1, return None
        if self.is_alive() {
            return None;
        }

        // return new player instance with health 100
        // if player level is over than 10, with mana 100
        // preserve level
        let new_player = Player {
            health: 100,
            mana: if self.can_use_mana_level() {
                Some(100)
            } else {
                None
            },
            level: self.level,
        };
        Some(new_player)
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if self.has_enough_mana(mana_cost) {
                    self.mana = Some(minus_safe(mana, mana_cost))
                } else {
                    return 0;
                }
            }
            None => {
                self.health = minus_safe(self.health, mana_cost);
                return 0;
            }
        }
        // return damage = mana_cost * 2
        return mana_cost * 2;
    }

    fn can_use_mana_level(&self) -> bool {
        self.level >= Player::CAN_USE_MANA_LEVEL
    }

    fn has_enough_mana(&self, mana_cost: u32) -> bool {
        match self.mana {
            Some(v) => return v >= mana_cost,
            None => return false,
        }
    }

    fn is_alive(&self) -> bool {
        self.health != 0
    }
}

#[test]
fn test_reviving_dead_player() {
    let dead_player = Player {
        health: 0,
        mana: Some(0),
        level: 34,
    };
    let revived_player = dead_player
        .revive()
        .expect("reviving a dead player must return Some(player)");
    assert_eq!(revived_player.health, 100);
    assert_eq!(revived_player.mana, Some(100));
    assert_eq!(revived_player.level, dead_player.level);
}

#[test]
fn test_reviving_dead_level9_player() {
    let dead_player = Player {
        health: 0,
        mana: None,
        level: 9,
    };
    let revived_player = dead_player
        .revive()
        .expect("reviving a dead player must return Some(player)");
    assert_eq!(revived_player.health, 100);
    assert_eq!(revived_player.mana, None);
    assert_eq!(revived_player.level, dead_player.level);
}

#[test]
fn test_reviving_dead_level10_player() {
    let dead_player = Player {
        health: 0,
        mana: Some(0),
        level: 10,
    };
    let revived_player = dead_player
        .revive()
        .expect("reviving a dead player must return Some(player)");
    assert_eq!(revived_player.health, 100);
    assert_eq!(revived_player.mana, Some(100));
    assert_eq!(revived_player.level, dead_player.level);
}

#[test]
fn test_reviving_alive_player() {
    let alive_player = Player {
        health: 1,
        mana: None,
        level: 8,
    };
    assert!(alive_player.revive().is_none());
}

#[test]
fn test_cast_spell_with_enough_mana() {
    const HEALTH: u32 = 99;
    const MANA: u32 = 100;
    const LEVEL: u32 = 100;
    const MANA_COST: u32 = 3;

    let mut accomplished_wizard = Player {
        health: HEALTH,
        mana: Some(MANA),
        level: LEVEL,
    };

    assert_eq!(accomplished_wizard.cast_spell(MANA_COST), MANA_COST * 2);
    assert_eq!(accomplished_wizard.health, HEALTH);
    assert_eq!(accomplished_wizard.mana, Some(MANA - MANA_COST));
    assert_eq!(accomplished_wizard.level, LEVEL);
}

#[test]
fn test_cast_spell_with_insufficient_mana() {
    let mut no_mana_wizard = Player {
        health: 56,
        mana: Some(2),
        level: 22,
    };

    let clone = Player { ..no_mana_wizard };

    assert_eq!(no_mana_wizard.cast_spell(3), 0);
    assert_eq!(no_mana_wizard.health, clone.health);
    assert_eq!(no_mana_wizard.mana, clone.mana);
    assert_eq!(no_mana_wizard.level, clone.level);
}

#[test]
fn test_cast_spell_with_no_mana_pool() {
    const MANA_COST: u32 = 10;

    let mut underleveled_player = Player {
        health: 87,
        mana: None,
        level: 6,
    };

    let clone = Player {
        ..underleveled_player
    };

    assert_eq!(underleveled_player.cast_spell(MANA_COST), 0);
    assert_eq!(underleveled_player.health, clone.health - MANA_COST);
    assert_eq!(underleveled_player.mana, clone.mana);
    assert_eq!(underleveled_player.level, clone.level);
}

#[test]
fn test_cast_large_spell_with_no_mana_pool() {
    const MANA_COST: u32 = 30;

    let mut underleveled_player = Player {
        health: 20,
        mana: None,
        level: 6,
    };

    assert_eq!(underleveled_player.cast_spell(MANA_COST), 0);
    assert_eq!(underleveled_player.health, 0);
    assert_eq!(underleveled_player.mana, None);
    assert_eq!(underleveled_player.level, 6);
}
