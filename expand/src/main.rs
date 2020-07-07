// #![cfg_attr(not(test), allow(unused))]
#![allow(unused)]

#[macro_use]
#[cfg(test)]
extern crate sif;

fn main() {}

enum NPC {
    Andre,
    Lautrec,
    Siegmeyer,
    Solaire,
}

trait Home {
    fn reigns_from(&self) -> &str;
}

impl Home for NPC {
    fn reigns_from(&self) -> &str {
        match self {
            NPC::Solaire | NPC::Andre => "Astora",
            NPC::Lautrec => "Carim",
            NPC::Siegmeyer => "Catarina",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Home, NPC};

    #[parameterized]
    #[case(NPC::Andre, "Astora")]
    #[case(NPC::Lautrec, "Carim")]
    #[case(NPC::Siegmeyer, "Catarina")]
    #[case(NPC::Solaire, "Astora")]
    fn npc_reigns_from_test(npc: NPC, place: &str) {
        assert_eq!(npc.reigns_from(), place)
    }
}
