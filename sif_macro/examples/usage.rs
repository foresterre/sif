#[cfg(test)]
mod test_environment_scoped {
    use sif::parameterized;

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

    #[parameterized]
    #[case(NPC::Andre, "Astora")]
    #[case(NPC::Lautrec, "Carim")]
    #[case(NPC::Siegmeyer, "Catarina")]
    #[case(NPC::Solaire, "Astrora")]
    fn npc_reigns_from_test(npc: NPC, place: &str) {
        assert_eq!(npc.reigns_from(), place)
    }
}

#[cfg(not(test))]
fn main() {
    use std::io::Write;

    let source = include_bytes!("usage.rs");
    eprintln!("Please pipe the stdout of this executable to an environment where you can run it with cfg(test) enabled (or copy the source in examples/usage.rs instead of piping it)");
    std::io::stdout()
        .write(source)
        .expect("Unable to write to stdout");
}
