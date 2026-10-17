// Implementation file modification for Finding 1...
// Deduplication logic by leaf index

impl Pool {
  fn add_deposit(&mut self, deposit: Deposit) {
      if !self.contains(deposit.leaf_index) {
          self.deposits.push(deposit);
      }
  }
  
  fn contains(&self, leaf_index: usize) -> bool {
      self.deposits.iter().any(|d| d.leaf_index == leaf_index)
  }
}
