#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::contract]
mod staking {
    use openbrush::traits::Storage;
    use staking_app::errors::StakingError;
    use staking_app::impls::staking::*;
    use staking_app::traits::staking::*;

    #[ink(storage)]
    #[derive(Storage, Default)]
    pub struct StakingContract {
        #[storage_field]
        staking: StakingData,
        pub timestamp: Timestamp
    }

    impl StakingImpl for StakingContract {
        fn block_timestamp(&self) -> Timestamp {
            return self.timestamp;
        }
    }

    impl Staking for StakingContract {
        #[ink(message)]
        fn stake(&mut self, amount: Balance) -> Result<(), StakingError> {
            self.stake_impl(amount)
        }

        #[ink(message)]
        fn unstake(&mut self, amount: Balance) -> Result<(), StakingError> {
            self.unstake_impl(amount)
        }
    }

    impl StakingContract {
        #[ink(constructor)]
        pun fn new(token: AccountId) -> Self {
            let mut contract = Self::default();
            contract.staking.token.set(&token);
            contract
        }


        #[ink(message)]
        pub fn set_timestamp(&mut self, timestamp: Timestamp) {
            self.timestamp = timestamp;
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;
        use openbrush::contracts::psp22::psp22_external::PSP22;
        use psp22_token::StakingTokenRef;
        use staking_app::traits::staking::staking_external::Staking;
        use openbrush::contracts::access_control::accesscontrol_external::AccessControl;
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test(additional_contracts = "../psp22/Cargo.toml")]
        async fn stake_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mint_amount = 1000000;
            let staking_token = StakingTokenRef::new(mint_amount);

            let psp22_account_id = client.instantiate("psp22_token", &ink_e2e::alice(), staking_token, 0, None).await.expect("instantiate failed").account_id;

            let staking_contract = StakingContractRef::new(psp22_account_id);

            let staking_account_id = client.instantiate("staking_contract", &ink_e2e::alice(), staking_contract, 0, None).await.expect("instantiate failed").account_id;
            

            let transfer_amount = 10000;
            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            
            let transfer_alice_bob = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.transfer(bob_account, transfer_amount, Vec::new()))
            

            client.call(&ink_e2e::alice(), transfer_alice_bob, 0, None).await.expect("transfer failed!")
            

            let balance_of_alice = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.balance_of(alice_account));
            let balance_of_alice_res = client.call_dry_run(&ink_e2e::alice(), &balance_of_alice, 0, None).await
            


            let balance_of_bob = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.balance_of(bob_account));
            let balance_of_bob_res = client.call_dry_run(&ink_e2e::bob(), &balance_of_bob, 0, None).await
            


            assert_eq!(balance_of_bob_res.return_value(), transfer_amount);
            assert_eq!(balance_of_alice_res.return_value(), mint_amount - transfer_amount);




            let bob_approve = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.approve(staking_account_id, transfer_amount));
            client.call(&ink_e2e::bob(), bob_approve, 0, None).await.expect("Stake failed")
            





            let bob_stake = build_message::<StakingTokenRef>(staking_account_id.clone()).call(|contract| contract.stake(transfer_amount));
            client.call(&ink_e2e::bob(), bob_stake, 0, None).await.expect("stake failed")
            



            let balance_of_bob_2 = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.balance_of(bob_account));
            let balance_of_bob_res_2 = client.call_dry_run(&ink_e2e::bob(), &balance_of_bob_2, 0, None).await;
            



            let balance_of_contract = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.balance_of(staking_account_id));
            let balance_of_contract_res = client.call_dry_run(&ink_e2e::bob(), &balance_of_contract, 0, None).await;
            


            assert_eq!(balance_of_bob_res_2.return_value(), 0);
            assert_eq!(balance_of_contract_res.return_value(), transfer_amount);



            OK(())
        }



        
        #[ink_e2e::test(additional_contracts = "../psp22/Cargo.toml")]
        async fn unstake_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mint_amount = 1000000;
            let staking_token = StakingTokenRef::new(mint_amount);

            let psp22_account_id = client.instantiate("psp22_token", &ink_e2e::alice(), staking_token, 0, None).await.expect("instantiate failed").account_id;

            let staking_contract = StakingContractRef::new(psp22_account_id);

            let staking_account_id = client.instantiate("staking_contract", &ink_e2e::alice(), staking_contract, 0, None).await.expect("instantiate failed").account_id;
            




            let transfer_amount = 10000;
            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            
            let transfer_alice_bob = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.transfer(bob_account, transfer_amount, Vec::new()))
            

            client.call(&ink_e2e::alice(), transfer_alice_bob, 0, None).await.expect("transfer failed!")
            

            // let balance_of_alice = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.balance_of(alice_account));
            // let balance_of_alice_res = client.call_dry_run(&ink_e2e::alice(), &balance_of_alice, 0, None).await
            


            // let balance_of_bob = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.balance_of(bob_account));
            // let balance_of_bob_res = client.call_dry_run(&ink_e2e::bob(), &balance_of_bob, 0, None).await
            


            // assert_eq!(balance_of_bob_res.return_value(), transfer_amount);
            // assert_eq!(balance_of_alice_res.return_value(), mint_amount - transfer_amount);




            let bob_approve = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.approve(staking_account_id, transfer_amount));
            client.call(&ink_e2e::bob(), bob_approve, 0, None).await.expect("Stake failed")
            





            let bob_stake = build_message::<StakingTokenRef>(staking_account_id.clone()).call(|contract| contract.stake(transfer_amount));
            client.call(&ink_e2e::bob(), bob_stake, 0, None).await.expect("stake failed")
            


            let unstake_amount = 1000;

            let bob_unstake = build_message::<StakingTokenRef>(staking_account_id.clone()).call(|contract| contract.unstake(unstake_amount));
            client.call(&ink_e2e::bob(), bob_unstake, 0, None).await.expect("unstake failed");
            



            let balance_of_bob = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.balance_of(bob_account));
            let balance_of_bob_res = client.call_dry_run(&ink_e2e::bob(), &balance_of_bob, 0, None).await
            



            let balance_of_contract = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.balance_of(staking_account_id));
            let balance_of_contract_res = client.call_dry_run(&ink_e2e::bob(), &balance_of_contract, 0, None).await;
            


            assert_eq!(balance_of_bob_res.return_value(), 0);
            assert_eq!(balance_of_contract_res.return_value(), transfer_amount - unstake_amount);



            OK(())
        }





        
        #[ink_e2e::test(additional_contracts = "../psp22/Cargo.toml")]
        async fn unstake_more_without_role_fails(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let mint_amount = 1000000;
            let staking_token = StakingTokenRef::new(mint_amount);

            let psp22_account_id = client.instantiate("psp22_token", &ink_e2e::alice(), staking_token, 0, None).await.expect("instantiate failed").account_id;

            let staking_contract = StakingContractRef::new(psp22_account_id);

            let staking_account_id = client.instantiate("staking_contract", &ink_e2e::alice(), staking_contract, 0, None).await.expect("instantiate failed").account_id;
            




            let transfer_amount = 10000;
            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            
            let transfer_alice_bob = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.transfer(bob_account, transfer_amount, Vec::new()))
            

            client.call(&ink_e2e::alice(), transfer_alice_bob, 0, None).await.expect("transfer failed!")
            

            // let balance_of_alice = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.balance_of(alice_account));
            // let balance_of_alice_res = client.call_dry_run(&ink_e2e::alice(), &balance_of_alice, 0, None).await
            


            // let balance_of_bob = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.balance_of(bob_account));
            // let balance_of_bob_res = client.call_dry_run(&ink_e2e::bob(), &balance_of_bob, 0, None).await
            


            // assert_eq!(balance_of_bob_res.return_value(), transfer_amount);
            // assert_eq!(balance_of_alice_res.return_value(), mint_amount - transfer_amount);




            let bob_approve = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.approve(staking_account_id, transfer_amount));
            client.call(&ink_e2e::bob(), bob_approve, 0, None).await.expect("Stake failed")
            





            let bob_stake = build_message::<StakingTokenRef>(staking_account_id.clone()).call(|contract| contract.stake(transfer_amount));
            client.call(&ink_e2e::bob(), bob_stake, 0, None).await.expect("stake failed")
            


            let grant_role = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.grant_role(ink::selector_id!("STAKING_CONTRACT"), Some(staking_account_id)));

            client.call(&ink_e2e::alice(), grant_role, 0, None).await.expect("grant role failed!")
            
            
            
            
            let set_time = build_message::<StakingContractRef>(staking_account_id.clone()).call(|contract| contract.set_timestamp(86400 * 365))
            client.call(&ink_e2e::alice(), set_time, 0, None).await.expect("set timestamp failed!")





            let unstake_amount = 1100;

            let bob_unstake = build_message::<StakingTokenRef>(staking_account_id.clone()).call(|contract| contract.unstake(unstake_amount));
            client.call(&ink_e2e::bob(), bob_unstake, 0, None).await.expect("unstake failed");
            





            let balance_of_bob = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.balance_of(bob_account));
            let balance_of_bob_res = client.call_dry_run(&ink_e2e::bob(), &balance_of_bob, 0, None).await
            



            let balance_of_contract = build_message::<StakingTokenRef>(psp22_account_id.clone()).call(|contract| contract.balance_of(staking_account_id));
            let balance_of_contract_res = client.call_dry_run(&ink_e2e::bob(), &balance_of_contract, 0, None).await;
            


            assert_eq!(balance_of_bob_res.return_value(), unstake_amount);
            assert_eq!(balance_of_contract_res.return_value(), 0);



            OK(())
        }



        
    }
}
