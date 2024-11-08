use cosmwasm_std::Addr;
use interface::strategy::{ExecuteMsg, OwnerProposal, QueryMsg};
use osmosis_test_tube::{Account, Module, RunnerError, Wasm};
use testing::{deployment::get_default_instantiation_msg, setup::TestEnv};

const PROPOSAL_DURATION: u64 = 1000;

#[test]
fn test_update_owner() {
    let env = TestEnv::new();

    let wasm = Wasm::new(&env.app);

    let strategy_addr =
        env.deploy_strategy_contract(&wasm, Some(get_default_instantiation_msg(&env)));

    let mut msg = env.default_fund_instantiation_msg();
    msg.controller = strategy_addr.to_string();

    let vault_addr = env.deploy_fund_contract(&wasm, msg);

    env.set_vault_strategy(&wasm, &strategy_addr, &vault_addr, &env.signer)
        .unwrap();
    env.set_open_fund(&wasm, &vault_addr, &env.signer).unwrap();

    // claim before a proposal is made
    {
        let err = wasm
            .execute(
                &strategy_addr,
                &ExecuteMsg::ClaimOwnership {},
                &[],
                &env.signer,
            )
            .unwrap_err();
        assert_eq!(
            err,
            RunnerError::ExecuteError {
                msg: "failed to execute message; message index: 0: Generic error: Proposal not found: execute wasm contract failed".to_string()
            }
        );
    }

    // propose new owner
    wasm.execute(
        &strategy_addr,
        &ExecuteMsg::ProposeNewOwner {
            new_owner: env.traders[0].address(),
            duration: PROPOSAL_DURATION,
        },
        &[],
        &env.signer,
    )
    .unwrap();

    let owner: Addr = wasm.query(&strategy_addr, &QueryMsg::Owner {}).unwrap();
    assert_eq!(owner, env.signer.address());

    // reject claim by incorrect new owner
    {
        let err = wasm
            .execute(
                &strategy_addr,
                &ExecuteMsg::ClaimOwnership {},
                &[],
                &env.signer,
            )
            .unwrap_err();
        assert_eq!(
            err,
            RunnerError::ExecuteError {
                msg: "failed to execute message; message index: 0: Unauthorized: execute wasm contract failed".to_string()
            }
        );
    }

    // let proposal expire
    env.app.increase_time(PROPOSAL_DURATION + 1);

    // proposal fails due to expiry
    {
        let err = wasm
            .execute(
                &strategy_addr,
                &ExecuteMsg::ClaimOwnership {},
                &[],
                &env.traders[0],
            )
            .unwrap_err();
        assert_eq!(
            err,
            RunnerError::ExecuteError {
                msg: "failed to execute message; message index: 0: Generic error: Proposal expired: execute wasm contract failed".to_string()
            }
        );
    }

    let owner: Addr = wasm.query(&strategy_addr, &QueryMsg::Owner {}).unwrap();
    assert_eq!(owner, env.signer.address());

    // propose new owner
    wasm.execute(
        &strategy_addr,
        &ExecuteMsg::ProposeNewOwner {
            new_owner: env.traders[0].address(),
            duration: PROPOSAL_DURATION,
        },
        &[],
        &env.signer,
    )
    .unwrap();

    let owner: Addr = wasm.query(&strategy_addr, &QueryMsg::Owner {}).unwrap();
    assert_eq!(owner, env.signer.address());

    // proposal fails due to expiry
    {
        let err = wasm
            .execute(
                &strategy_addr,
                &ExecuteMsg::RejectOwner {},
                &[],
                &env.traders[0],
            )
            .unwrap_err();
        assert_eq!(
            err,
            RunnerError::ExecuteError {
                msg: "failed to execute message; message index: 0: Unauthorized: execute wasm contract failed".to_string()
            }
        );
    }

    // proposal fails due to expiry
    {
        wasm.execute(
            &strategy_addr,
            &ExecuteMsg::RejectOwner {},
            &[],
            &env.signer,
        )
        .unwrap();
    }

    // propose new owner
    wasm.execute(
        &strategy_addr,
        &ExecuteMsg::ProposeNewOwner {
            new_owner: env.traders[0].address(),
            duration: PROPOSAL_DURATION,
        },
        &[],
        &env.signer,
    )
    .unwrap();

    let block_time = env.app.get_block_time_seconds();

    let owner: Addr = wasm.query(&strategy_addr, &QueryMsg::Owner {}).unwrap();
    assert_eq!(owner, env.signer.address());

    // query ownership proposal
    {
        let proposal: OwnerProposal = wasm
            .query(&strategy_addr, &QueryMsg::GetOwnershipProposal {})
            .unwrap();

        assert_eq!(proposal.owner, env.traders[0].address());
        assert_eq!(proposal.expiry, block_time as u64 + PROPOSAL_DURATION);
    }

    // claim ownership
    {
        wasm.execute(
            &strategy_addr,
            &ExecuteMsg::ClaimOwnership {},
            &[],
            &env.traders[0],
        )
        .unwrap();
    }

    let owner: Addr = wasm.query(&strategy_addr, &QueryMsg::Owner {}).unwrap();
    assert_eq!(owner, env.traders[0].address());
}
