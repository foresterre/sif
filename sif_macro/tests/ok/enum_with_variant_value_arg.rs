#[macro_use]
extern crate sif_macro;

enum Color {
    Red(Pigment),
    Yellow,
    Blue(Pigment),
}

impl Color {
    pub fn pigment(&self) -> u32 {
        match self {
            Color::Yellow => 2,
            Color::Red(r) => r.material_id(),
            Color::Blue(b) => b.material_id(),
        }
    }
}

struct Pigment {
    material_id: u32,
}

impl Pigment {
    fn new(id: u32) -> Self {
        Self { material_id: id }
    }

    pub(crate) fn material_id(&self) -> u32 {
        self.material_id
    }
}

impl Default for Pigment {
    fn default() -> Self {
        Self { material_id: 0 }
    }
}

#[parameterized]
#[case(Color::Red(Pigment::new(5)))]
#[case(Color::Yellow)]
#[case(Color::Blue(Pigment::default()))]
#[case(Color::Red(Pigment { material_id: 8 }))]
fn my_test(v: Color) {
    assert_ne!(v.pigment(), 1)
}

fn main() {}
