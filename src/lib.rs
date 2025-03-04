// This attribute ensures that the main function is not included unless we are testing or exporting the ABI.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
extern crate alloc;

// Importing necessary modules from the stylus_sdk crate.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

sol_storage! {
    #[entrypoint]
    pub struct Defi  {
        uint256 price; // This is similar to declaring a property in a TypeScript class.

        uint256 total_supply;

        mapping(address => uint256) balances;
    }
}

#[public]
impl Defi {
    #[payable]
    pub fn mint(&mut self) {
        let eth_sent = self.vm().msg_value();
        self.total_supply.set(self.total_supply.get() + eth_sent);

        // let sender = self.vm().msg_sender();

        // Increasing receiver balance
        // let mut to_balance = self.balances.setter(sender);
        // let new_to_balance = to_balance.get() + eth_sent;
        // to_balance.set(new_to_balance);

        // let balance = self.balances.get(sender);
        // self.balances.insert(sender, &balance + eth_sent);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_defi() {
        use stylus_sdk::testing::*;
        let vm = TestVM::default();
        let mut contract = Defi::from(&vm);

        assert_eq!(U256::ZERO, contract.total_supply.get());

        vm.set_value(U256::from(102));

        contract.mint();
        assert_eq!(U256::from(102), contract.total_supply.get());
        // assert_eq!(U256::from(102), '0xaD89439C290f0045529c5cf610766ff1A3EF5D53');
    }
}
