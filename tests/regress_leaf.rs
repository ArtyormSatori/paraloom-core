mod tests {
    use super::*;

    #[test]
    fn regression_dedup_leaf_index() {
        // Scenario: Two deposits with identical commitments
        let commitment = "commitment_test";
        let deposit1 = Deposit::new(commitment, leaf_index_1);
        let deposit2 = Deposit::new(commitment, leaf_index_2);

        let mut pool = Pool::new();
        pool.add_deposit(deposit1);
        pool.add_deposit(deposit2);

        assert!(pool.contains(leaf_index_1));
        assert!(pool.contains(leaf_index_2));
        assert!(pool.credits_correct_for_each_leaf());
    }
}
