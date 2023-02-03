use std::cmp;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // Even if the next block retained was still in `if/else`, I think there's
        // some readability benefit to isolating this "edge case" instead of tying
        // all the cases together `if/else if/else` because this first check is an
        // "special" or "edge" case that the reader should think of as an exception
        // to the "normal" revive behavior.
        if self.health > 0 {
            return None;
        }

        // If multiple fields depended varied depending on the `self.level < 10`
        // condition, I would have kept the `Player` in separate `if / else` blocks
        // to avoid re-evaluating the condition.
        Some(Player {
            health: 100,
            mana: if self.level < 10 { None } else { Some(100) },
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // Not really necessary, but since the function name or documentation doesn't
        // explicity state what the value being computed is, I assigned the calculation
        // to a variable before returning
        let damage = match self.mana {
            // I'm not really sure this is better, just an alternative way that "flattens"
            // the `if/else` branches and has an example of how matching branch patterns
            // can also have boolean expressions.
            //
            // I tend to avoid nesting as much as possible so it's easier to build the
            // context of what the branching the code is in at a glance, and I can explain it
            // out loud to myself without my eyes jumping to too many different lines:
            // - First case I think "if player mana exists and is greater than or equal to mana cost"
            // - Next I think "...otherwise if mana exists but I don't care about the value anymore"
            // - Finally I think "...otherwise if the player mana does not exist"
            Some(mana) if mana >= mana_cost => {
                self.mana = Some(mana - mana_cost);
                2 * mana_cost
            }
            Some(_) => 0,
            None => {
                // I avoid branching syntax when choosing between two different values,
                // but when the behavior of the code should be the same.
                //
                // Single-line expression position if-else is an alternative:
                //     self.health -= if mana_cost < self.health { mana_cost } else { self.health };
                //
                // But IMO `cmp::min` provides some immedate _semantic_ difference, scanning I read as
                // >> "self health decrement is limited to the least of mana_cost or all health"
                //
                // maybe still a bit stuttery - alternatively, the following could be used
                //     self.health = self.health.saturating_sub(mana_cost);
                //
                // ... which I read as
                // >> "new health is old health minus mana_cost until zero"
                //
                // Similar to my comment about `damage`, this doesn't need to be tied to a named variable
                // since it's fairly clear why health is being subtracted, but a comment here like
                // "mishandling magic causes damage to backfire on the caster" can add context.
                self.health -= cmp::min(mana_cost, self.health);
                0
            }
        };

        damage
    }
}
