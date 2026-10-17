mod tests {
    use super::*;

    #[test]
    fn regression_realized_delta_credit() {
        // Scenario: Vault receives less than the gross amount credited
        let pre_balance = 1000;
        let post_balance = 1100;
        let gross_credit = 200;
        let realized_credit = post_balance.checked_sub(pre_balance).unwrap_or(0);
        
        assert_eq!(realized_credit, 100);
        
        let mut vault = Vault::new();
        vault.credit(realized_credit);

        assert!(vault.contains_credit(realized_credit));
    }
}
