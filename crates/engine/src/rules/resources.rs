//! Gaining and losing Sanity, Stamina, money and clue tokens.

use data::{Resource, Timing};

use crate::game::Game;
use crate::halt::{Flow, Halt};
use crate::rules::stats::resource_name;

impl Game<'_> {
    /// Current amount of a resource.
    pub fn amount(&self, seat: usize, resource: Resource) -> u32 {
        let inv = self.inv(seat);
        match resource {
            Resource::Sanity => u32::from(inv.sanity),
            Resource::Stamina => u32::from(inv.stamina),
            Resource::Money => u32::from(inv.money),
            Resource::Clues => u32::from(inv.clues),
        }
    }

    /// Sets a resource, capped at the maximum for Sanity and Stamina.
    fn set_amount(&mut self, seat: usize, resource: Resource, value: u32) -> Flow<()> {
        let capped = match resource {
            Resource::Sanity | Resource::Stamina => {
                value.min(u32::from(self.maximum(seat, resource)?))
            }
            _ => value,
        };
        let inv = self.inv_mut(seat);
        match resource {
            Resource::Sanity => inv.sanity = u8::try_from(capped).unwrap_or(u8::MAX),
            Resource::Stamina => inv.stamina = u8::try_from(capped).unwrap_or(u8::MAX),
            Resource::Money => inv.money = u16::try_from(capped).unwrap_or(u16::MAX),
            Resource::Clues => inv.clues = u16::try_from(capped).unwrap_or(u16::MAX),
        }
        Ok(())
    }

    /// Gains a resource. No money nor clue token once the final battle has begun.
    pub fn gain(&mut self, seat: usize, resource: Resource, amount: u32) -> Flow<()> {
        let frozen = matches!(resource, Resource::Money | Resource::Clues);
        if amount == 0 || (frozen && self.state.battle.is_some()) {
            return Ok(());
        }
        let value = self.amount(seat, resource) + amount;
        self.set_amount(seat, resource, value)?;
        self.log(format!(
            "{} gagne {amount} {}.",
            self.name(seat),
            resource_name(resource)
        ));
        Ok(())
    }

    /// Pays a cost: never reduced, and only possible with enough in hand.
    /// Returns whether it was paid.
    pub fn pay(&mut self, seat: usize, resource: Resource, amount: u32) -> Flow<bool> {
        let current = self.amount(seat, resource);
        if current < amount {
            return Ok(false);
        }
        self.set_amount(seat, resource, current - amount)?;
        if amount > 0 {
            self.log(format!(
                "{} paie {amount} {}.",
                self.name(seat),
                resource_name(resource)
            ));
        }
        self.after_loss(seat)?;
        Ok(true)
    }

    /// Loses a resource. Cards that reduce losses are offered first; reaching 0
    /// Sanity or Stamina knocks the investigator out.
    pub fn lose(&mut self, seat: usize, resource: Resource, amount: u32) -> Flow<()> {
        self.lose_both(seat, resource, amount, None)
    }

    /// Loses one resource and possibly a second one at the same time (for
    /// « perdez 1 Résistance et 1 Santé Mentale »: both at 0 together devours).
    pub fn lose_both(
        &mut self,
        seat: usize,
        resource: Resource,
        amount: u32,
        second: Option<(Resource, u32)>,
    ) -> Flow<()> {
        let mut losses = vec![(resource, amount)];
        losses.extend(second);
        for (resource, amount) in losses {
            let amount = self.reduce_loss(seat, resource, amount)?;
            if amount == 0 {
                continue;
            }
            let value = self.amount(seat, resource).saturating_sub(amount);
            self.set_amount(seat, resource, value)?;
            self.log(format!(
                "{} perd {amount} {}.",
                self.name(seat),
                resource_name(resource)
            ));
        }
        self.after_loss(seat)
    }

    /// Offers the cards that reduce a loss of this resource.
    fn reduce_loss(&mut self, seat: usize, resource: Resource, mut amount: u32) -> Flow<u32> {
        while amount > 0 {
            self.scratch.reduce_loss = 0;
            let prompt = format!("Vous allez perdre {amount} {}.", resource_name(resource));
            if !self.offer_abilities(
                seat,
                &|timing| *timing == Timing::OnLoss(resource),
                &prompt,
            )? {
                break;
            }
            amount = amount.saturating_sub(self.scratch.reduce_loss);
        }
        Ok(amount)
    }

    /// Knocks the investigator out if Sanity or Stamina reached 0.
    pub fn after_loss(&mut self, seat: usize) -> Flow<()> {
        let inv = self.inv(seat);
        let (sanity, stamina) = (inv.sanity, inv.stamina);
        let max_gone = self.maximum(seat, Resource::Sanity)? == 0
            || self.maximum(seat, Resource::Stamina)? == 0;
        let in_battle = self.state.battle.is_some() && (sanity == 0 || stamina == 0);
        if max_gone || in_battle || (sanity == 0 && stamina == 0) {
            self.devour(seat)?;
            return Err(Halt::KnockedOut(seat));
        }
        if sanity == 0 || stamina == 0 {
            self.knock_out(seat, sanity == 0)?;
            return Err(Halt::KnockedOut(seat));
        }
        Ok(())
    }
}
