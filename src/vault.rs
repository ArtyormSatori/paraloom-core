// Implementation for Finding 2 (realized delta credit calculation)

impl Vault {
    pub fn credit(&mut self, amount: u32) {
        self.balance += amount;
    }

    pub fn contains_credit(&self, amount: u32) -> bool {
        self.balance >= amount
    }
}
