use envmnt;

pub struct SetProduct {
    name: String,
}

impl SetProduct {
    pub fn new(name: String) -> SetProduct {
        Self { name }
    }
    pub fn run(self) {
        envmnt::set("SG_CURRENT_PRODUCT", self.name);
    }
}
