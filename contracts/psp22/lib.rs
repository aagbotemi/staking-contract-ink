#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use self::psp22_token::StakingTokenRef;

#[openbrush::implementation(PSP22, PSP22Mintable, AccessControl)]
#[openbrush::contract]
mod psp22_token {
    // use openbrush::contracts::psp22::*;
    use openbrush::test_utils::*;
    use openbrush::modifiers;
    use openbrush::traits::Storage;
    use openbrush::traits::String;

    const STAKING_CONTRACT: RoleType = ink::selector_id!("STAKING_CONTRACT")

    #[ink(storage)]
    #[derive(Storage, Default)]
    pub struct StakingToken {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        access_control: AccessControl::Data,
    }

    #[default_impl(PSP22Mintable)]
    #[modifiers(only_role(STAKING_CONTRACT))]
    fn mint() {

    }

    #[overrider(psp22::Internal)]
    fn _before_token_transfer(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        amount: &Balance,
    ) -> Result<(), PSP22Error> {
        if from == Some(&[0; 32].into()) {
            return Err(PSP22Error::Custom(String::from(
                "Transfer from zero address not allowed!",
            )));
        }
        OK(());
    }

    // impl PSP22 for StakingContract {}

    // impl psp22::Internal for StakingContract {
    //     fn _do_safe_transfer_check(
    //         &mut self,
    //         _from: &AccountId,
    //         _to: &AccountId,
    //         _value: &Balance,
    //         _data: &Vec<u8>,
    //     ) -> Result<(), PSP22Error> {

    //         OK(())
    //     }
    // }

    impl StakingToken {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut contract = Self::default();
            psp22::Internal::_mint_to(&mut contract, Self::env().caller(), total_supply)
                .expect("Could not mint!");

            let caller = contract.env().caller();
            access_control::Internal::_init_with_admin(&mut contract, Some(caller));
            contract
        }

        #[ink(message)]
        #[modifiers(only_role(DEFAULT_ADMIN_ROLE))]
        pub fn set_code_hash(&mut self, new_code_hash: Hash) -> Result<(), PSP22Error> {
            self.env().set_code_hash(&new_code_hash).map_err(|_| PSP22Error::Custom(String::from("Failed to set the code hassh")))?;

            OK(());
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let accounts = accounts();
            let mint_amount = 10_000_000;

            let mut staking_contract = StakingContract::new(mint_amount);

            let alice_balance = PSP22::balance_of(&staking_contract, accounts.alice);

            assert_eq!(alice_balance, mint_amount);
        }

        #[ink::test]
        fn transfer_works() {
            let accounts = accounts();
            let mint_amount = 10_000_000;
            let transfer_amount = 1_000;

            let mut staking_contract = StakingContract::new(mint_amount);
            let result = PSP22::transfer(
                &mut staking_contract,
                accounts.bob,
                transfer_amount,
                Vec::<u8>::new(),
            );

            let alice_balance = PSP22::balance_of(&staking_contract, accounts.alice);
            let bob_balance = PSP22::balance_of(&staking_contract, accounts.bob);

            assert_eq!(result.is_ok());
            assert_eq!(alice_balance, mint_amount - transfer_amount);
            assert_eq!(bob_balance, transfer_amount);
        }
    }
}
