struct Burst {}

struct MachineSet {}

struct BurstBuilder {
    descriptions: Vec<MachineSet>,
}

impl Default for BurstBuilder {
    fn default() -> Self {
        BurstBuilder {
            descriptions: Vec::new(),
        }
    }
}

impl BurstBuilder {
    pub fn add_set(&mut self, description: MachineSet) {}

    pub fn run() {}
}

fn main() {
    let mut b = BurstBuilder::default();
}
