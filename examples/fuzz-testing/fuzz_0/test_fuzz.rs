use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod types;
use types::*;

/// Fuzz test for claw-shark (Launch Vault) and claw-shark-curve (Bonding Curve) programs.
///
/// Targets:
/// - Arithmetic overflow in bonding curve price calculations
/// - Invalid account combinations
/// - Edge cases (buy with 0 amount, sell more than owned, etc.)
/// - State machine invariant violations
/// - Vault balance underflow/overflow
#[derive(FuzzTestMethods)]
struct FuzzTest {
    trident: Trident,
    fuzz_accounts: AccountAddresses,
}

#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            trident: Trident::default(),
            fuzz_accounts: AccountAddresses::default(),
        }
    }

    #[init]
    fn start(&mut self) {
        // Each iteration starts with a clean state
    }

    // =========================================================================
    // BONDING CURVE PROGRAM: claw_shark_curve
    // =========================================================================

    /// Flow: Create a bonding curve launch with valid parameters
    #[flow]
    fn curve_create_launch_valid(&mut self) {
        let founder = self.fuzz_accounts.founder.get_or_create_account(
            0,
            &mut self.trident,
            10_000_000_000, // 10 SOL
        );
        let launch_pda = Pubkey::find_program_address(
            &[b"launch", founder.as_ref(), b"FuzzToken"],
            &claw_shark_curve::program_id(),
        )
        .0;

        let ix = claw_shark_curve::CreateLaunchInstruction::data(
            claw_shark_curve::CreateLaunchInstructionData::new(
                "FuzzToken".to_string(),
                "FUZZ".to_string(),
                1_000_000_000_000, // 1M tokens (6 decimals)
                1_000_000,         // 1 USDC initial price
                100,               // slope: 1% per token
                69_000_000_000,    // $69K migration threshold
            ),
        )
        .accounts(claw_shark_curve::CreateLaunchInstructionAccounts::new(
            launch_pda, founder,
        ))
        .instruction();

        let _ = self.trident.process_instruction(ix, &[&founder]);
    }

    /// Flow: Create launch with extreme values to test overflow guards
    #[flow]
    fn curve_create_launch_extreme_values(&mut self) {
        let founder = self.fuzz_accounts.founder.get_or_create_account(
            1,
            &mut self.trident,
            10_000_000_000,
        );
        let launch_pda = Pubkey::find_program_address(
            &[b"launch", founder.as_ref(), b"MaxToken"],
            &claw_shark_curve::program_id(),
        )
        .0;

        // Extreme values: u64::MAX for supply and price to trigger overflow
        let ix = claw_shark_curve::CreateLaunchInstruction::data(
            claw_shark_curve::CreateLaunchInstructionData::new(
                "MaxToken".to_string(),
                "MAX".to_string(),
                u64::MAX,   // Maximum possible supply
                u64::MAX,   // Maximum initial price
                u64::MAX,   // Maximum slope
                u64::MAX,   // Maximum threshold
            ),
        )
        .accounts(claw_shark_curve::CreateLaunchInstructionAccounts::new(
            launch_pda, founder,
        ))
        .instruction();

        let _ = self.trident.process_instruction(ix, &[&founder]);
    }

    /// Flow: Buy tokens with 0 amount (should fail with ZeroAmount)
    #[flow]
    fn curve_buy_zero_amount(&mut self) {
        let buyer = self.fuzz_accounts.buyer.get_or_create_account(
            0,
            &mut self.trident,
            10_000_000_000,
        );
        let launch = self
            .fuzz_accounts
            .launch
            .get_or_create_account(0, &mut self.trident, 0);
        let purchase = self
            .fuzz_accounts
            .purchase
            .get_or_create_account(0, &mut self.trident, 0);

        let ix = claw_shark_curve::BuyTokensInstruction::data(
            claw_shark_curve::BuyTokensInstructionData::new(0), // zero amount
        )
        .accounts(claw_shark_curve::BuyTokensInstructionAccounts::new(
            launch, purchase, Pubkey::default(), buyer,
        ))
        .instruction();

        // Should be rejected by the program
        let result = self.trident.process_instruction(ix, &[&buyer]);
        assert!(
            result.is_err(),
            "buy_tokens with 0 amount should fail"
        );
    }

    /// Flow: Buy tokens with u64::MAX to trigger overflow in price calculation
    #[flow]
    fn curve_buy_overflow_amount(&mut self) {
        let buyer = self.fuzz_accounts.buyer.get_or_create_account(
            1,
            &mut self.trident,
            10_000_000_000,
        );
        let launch = self
            .fuzz_accounts
            .launch
            .get_or_create_account(0, &mut self.trident, 0);
        let purchase = self
            .fuzz_accounts
            .purchase
            .get_or_create_account(1, &mut self.trident, 0);

        let ix = claw_shark_curve::BuyTokensInstruction::data(
            claw_shark_curve::BuyTokensInstructionData::new(u64::MAX),
        )
        .accounts(claw_shark_curve::BuyTokensInstructionAccounts::new(
            launch, purchase, Pubkey::default(), buyer,
        ))
        .instruction();

        // Should be rejected due to overflow checks
        let result = self.trident.process_instruction(ix, &[&buyer]);
        assert!(
            result.is_err(),
            "buy_tokens with u64::MAX should fail with overflow"
        );
    }

    /// Flow: Sell more tokens than exist on the curve (underflow)
    #[flow]
    fn curve_sell_more_than_owned(&mut self) {
        let seller = self.fuzz_accounts.seller.get_or_create_account(
            0,
            &mut self.trident,
            10_000_000_000,
        );
        let launch = self
            .fuzz_accounts
            .launch
            .get_or_create_account(0, &mut self.trident, 0);

        let ix = claw_shark_curve::SellTokensInstruction::data(
            claw_shark_curve::SellTokensInstructionData::new(u64::MAX),
        )
        .accounts(claw_shark_curve::SellTokensInstructionAccounts::new(
            launch, seller,
        ))
        .instruction();

        // Should be rejected with underflow
        let result = self.trident.process_instruction(ix, &[&seller]);
        assert!(
            result.is_err(),
            "sell_tokens exceeding tokens_sold should underflow"
        );
    }

    /// Flow: Sell tokens with 0 amount (should fail with ZeroAmount)
    #[flow]
    fn curve_sell_zero_amount(&mut self) {
        let seller = self.fuzz_accounts.seller.get_or_create_account(
            1,
            &mut self.trident,
            10_000_000_000,
        );
        let launch = self
            .fuzz_accounts
            .launch
            .get_or_create_account(0, &mut self.trident, 0);

        let ix = claw_shark_curve::SellTokensInstruction::data(
            claw_shark_curve::SellTokensInstructionData::new(0),
        )
        .accounts(claw_shark_curve::SellTokensInstructionAccounts::new(
            launch, seller,
        ))
        .instruction();

        let result = self.trident.process_instruction(ix, &[&seller]);
        assert!(
            result.is_err(),
            "sell_tokens with 0 amount should fail"
        );
    }

    /// Flow: Buy tokens after migration (should fail with AlreadyMigrated)
    #[flow]
    fn curve_buy_after_migration(&mut self) {
        // Setup: create launch, buy enough to trigger migration, then buy again
        let founder = self.fuzz_accounts.founder.get_or_create_account(
            2,
            &mut self.trident,
            10_000_000_000,
        );
        let launch_pda = Pubkey::find_program_address(
            &[b"launch", founder.as_ref(), b"MigrateTest"],
            &claw_shark_curve::program_id(),
        )
        .0;

        // Create launch with very low migration threshold
        let create_ix = claw_shark_curve::CreateLaunchInstruction::data(
            claw_shark_curve::CreateLaunchInstructionData::new(
                "MigrateTest".to_string(),
                "MIG".to_string(),
                1_000_000_000_000,
                1_000_000,
                100,
                1, // very low threshold to trigger migration
            ),
        )
        .accounts(claw_shark_curve::CreateLaunchInstructionAccounts::new(
            launch_pda, founder,
        ))
        .instruction();

        let _ = self.trident.process_instruction(create_ix, &[&founder]);

        // Buy enough to trigger migration
        let buyer = self.fuzz_accounts.buyer.get_or_create_account(
            2,
            &mut self.trident,
            10_000_000_000,
        );
        let purchase_pda = Pubkey::find_program_address(
            &[
                b"purchase",
                launch_pda.as_ref(),
                buyer.as_ref(),
                &0u64.to_le_bytes(),
            ],
            &claw_shark_curve::program_id(),
        )
        .0;

        let buy_ix = claw_shark_curve::BuyTokensInstruction::data(
            claw_shark_curve::BuyTokensInstructionData::new(100_000_000), // 100 USDC
        )
        .accounts(claw_shark_curve::BuyTokensInstructionAccounts::new(
            launch_pda,
            purchase_pda,
            Pubkey::default(),
            buyer,
        ))
        .instruction();

        let _ = self.trident.process_instruction(buy_ix, &[&buyer]);

        // Try to buy again after migration
        let purchase_pda_2 = Pubkey::find_program_address(
            &[
                b"purchase",
                launch_pda.as_ref(),
                buyer.as_ref(),
                &1u64.to_le_bytes(),
            ],
            &claw_shark_curve::program_id(),
        )
        .0;

        let buy_after_ix = claw_shark_curve::BuyTokensInstruction::data(
            claw_shark_curve::BuyTokensInstructionData::new(1_000_000),
        )
        .accounts(claw_shark_curve::BuyTokensInstructionAccounts::new(
            launch_pda,
            purchase_pda_2,
            Pubkey::default(),
            buyer,
        ))
        .instruction();

        let result = self.trident.process_instruction(buy_after_ix, &[&buyer]);
        assert!(
            result.is_err(),
            "buy_tokens after migration should fail"
        );
    }

    /// Flow: Get price on a launch (read-only, should never fail)
    #[flow]
    fn curve_get_price(&mut self) {
        let launch = self
            .fuzz_accounts
            .launch
            .get_or_create_account(0, &mut self.trident, 0);

        let ix = claw_shark_curve::GetPriceInstruction::data(
            claw_shark_curve::GetPriceInstructionData::new(),
        )
        .accounts(claw_shark_curve::GetPriceInstructionAccounts::new(launch))
        .instruction();

        let _ = self.trident.process_instruction(ix, &[]);
    }

    // =========================================================================
    // AGENT VAULT OPERATIONS: claw_shark_curve
    // =========================================================================

    /// Flow: Create vault with valid parameters
    #[flow]
    fn vault_create_valid(&mut self) {
        let owner = self.fuzz_accounts.owner.get_or_create_account(
            0,
            &mut self.trident,
            10_000_000_000,
        );
        let vault_pda = Pubkey::find_program_address(
            &[b"vault", owner.as_ref()],
            &claw_shark_curve::program_id(),
        )
        .0;

        let ix = claw_shark_curve::CreateVaultInstruction::data(
            claw_shark_curve::CreateVaultInstructionData::new(
                10_000_000_000, // max 10K USDC per project
                50_000_000_000, // daily limit 50K USDC
                60,             // min score 60
                true,           // require consensus
            ),
        )
        .accounts(claw_shark_curve::CreateVaultInstructionAccounts::new(
            vault_pda, owner,
        ))
        .instruction();

        let _ = self.trident.process_instruction(ix, &[&owner]);
    }

    /// Flow: Deposit to vault with overflow amount
    #[flow]
    fn vault_deposit_overflow(&mut self) {
        let owner = self.fuzz_accounts.owner.get_or_create_account(
            0,
            &mut self.trident,
            10_000_000_000,
        );
        let vault_pda = Pubkey::find_program_address(
            &[b"vault", owner.as_ref()],
            &claw_shark_curve::program_id(),
        )
        .0;

        let ix = claw_shark_curve::DepositVaultInstruction::data(
            claw_shark_curve::DepositVaultInstructionData::new(u64::MAX),
        )
        .accounts(claw_shark_curve::DepositVaultInstructionAccounts::new(
            vault_pda, owner,
        ))
        .instruction();

        // First deposit of u64::MAX might succeed, but second should overflow
        let _ = self.trident.process_instruction(ix.clone(), &[&owner]);

        let ix2 = claw_shark_curve::DepositVaultInstruction::data(
            claw_shark_curve::DepositVaultInstructionData::new(1),
        )
        .accounts(claw_shark_curve::DepositVaultInstructionAccounts::new(
            vault_pda, owner,
        ))
        .instruction();

        let result = self.trident.process_instruction(ix2, &[&owner]);
        assert!(
            result.is_err(),
            "deposit after u64::MAX balance should overflow"
        );
    }

    /// Flow: Withdraw more than balance (should fail with InsufficientBalance)
    #[flow]
    fn vault_withdraw_more_than_balance(&mut self) {
        let owner = self.fuzz_accounts.owner.get_or_create_account(
            1,
            &mut self.trident,
            10_000_000_000,
        );
        let vault_pda = Pubkey::find_program_address(
            &[b"vault", owner.as_ref()],
            &claw_shark_curve::program_id(),
        )
        .0;

        // Create vault first
        let create_ix = claw_shark_curve::CreateVaultInstruction::data(
            claw_shark_curve::CreateVaultInstructionData::new(
                1_000_000, 1_000_000, 50, false,
            ),
        )
        .accounts(claw_shark_curve::CreateVaultInstructionAccounts::new(
            vault_pda, owner,
        ))
        .instruction();

        let _ = self.trident.process_instruction(create_ix, &[&owner]);

        // Try to withdraw from empty vault
        let withdraw_ix = claw_shark_curve::WithdrawVaultInstruction::data(
            claw_shark_curve::WithdrawVaultInstructionData::new(1_000_000),
        )
        .accounts(claw_shark_curve::WithdrawVaultInstructionAccounts::new(
            vault_pda, owner,
        ))
        .instruction();

        let result = self.trident.process_instruction(withdraw_ix, &[&owner]);
        assert!(
            result.is_err(),
            "withdraw from empty vault should fail"
        );
    }

    /// Flow: Withdraw with unauthorized signer
    #[flow]
    fn vault_withdraw_unauthorized(&mut self) {
        let owner = self.fuzz_accounts.owner.get_or_create_account(
            0,
            &mut self.trident,
            10_000_000_000,
        );
        let attacker = self.fuzz_accounts.seller.get_or_create_account(
            2,
            &mut self.trident,
            10_000_000_000,
        );
        let vault_pda = Pubkey::find_program_address(
            &[b"vault", owner.as_ref()],
            &claw_shark_curve::program_id(),
        )
        .0;

        // Attacker tries to withdraw using owner's vault but attacker as signer
        let ix = claw_shark_curve::WithdrawVaultInstruction::data(
            claw_shark_curve::WithdrawVaultInstructionData::new(1_000_000),
        )
        .accounts(claw_shark_curve::WithdrawVaultInstructionAccounts::new(
            vault_pda, attacker, // wrong owner
        ))
        .instruction();

        let result = self.trident.process_instruction(ix, &[&attacker]);
        assert!(
            result.is_err(),
            "unauthorized withdrawal should fail"
        );
    }

    // =========================================================================
    // LAUNCH VAULT PROGRAM: claw_shark
    // =========================================================================

    /// Flow: Create launch vault with valid parameters
    #[flow]
    fn vault_create_launch_valid(&mut self) {
        let authority = self.fuzz_accounts.authority.get_or_create_account(
            0,
            &mut self.trident,
            10_000_000_000,
        );
        let launch_pda = Pubkey::find_program_address(
            &[b"launch", authority.as_ref(), b"TestProject"],
            &claw_shark::program_id(),
        )
        .0;

        let ix = claw_shark::CreateLaunchInstruction::data(
            claw_shark::CreateLaunchInstructionData::new(
                "TestProject".to_string(),
                100_000_000_000, // 100K USDC goal
                Some(500_000_000_000), // 500K cap
                i64::MAX,        // far future deadline
                85,              // quality score
            ),
        )
        .accounts(claw_shark::CreateLaunchInstructionAccounts::new(
            launch_pda, authority,
        ))
        .instruction();

        let _ = self.trident.process_instruction(ix, &[&authority]);
    }

    /// Flow: Create launch with name exceeding MAX_NAME_LEN (64 chars)
    #[flow]
    fn vault_create_launch_name_too_long(&mut self) {
        let authority = self.fuzz_accounts.authority.get_or_create_account(
            1,
            &mut self.trident,
            10_000_000_000,
        );
        let long_name = "A".repeat(65); // exceeds 64 char limit
        let launch_pda = Pubkey::find_program_address(
            &[b"launch", authority.as_ref(), long_name.as_bytes()],
            &claw_shark::program_id(),
        )
        .0;

        let ix = claw_shark::CreateLaunchInstruction::data(
            claw_shark::CreateLaunchInstructionData::new(
                long_name,
                100_000_000,
                None,
                i64::MAX,
                80,
            ),
        )
        .accounts(claw_shark::CreateLaunchInstructionAccounts::new(
            launch_pda, authority,
        ))
        .instruction();

        let result = self.trident.process_instruction(ix, &[&authority]);
        assert!(
            result.is_err(),
            "create_launch with name > 64 chars should fail"
        );
    }

    /// Flow: Create launch with raise_goal = 0 (should fail)
    #[flow]
    fn vault_create_launch_zero_goal(&mut self) {
        let authority = self.fuzz_accounts.authority.get_or_create_account(
            2,
            &mut self.trident,
            10_000_000_000,
        );
        let launch_pda = Pubkey::find_program_address(
            &[b"launch", authority.as_ref(), b"ZeroGoal"],
            &claw_shark::program_id(),
        )
        .0;

        let ix = claw_shark::CreateLaunchInstruction::data(
            claw_shark::CreateLaunchInstructionData::new(
                "ZeroGoal".to_string(),
                0, // invalid: zero goal
                None,
                i64::MAX,
                50,
            ),
        )
        .accounts(claw_shark::CreateLaunchInstructionAccounts::new(
            launch_pda, authority,
        ))
        .instruction();

        let result = self.trident.process_instruction(ix, &[&authority]);
        assert!(
            result.is_err(),
            "create_launch with 0 raise_goal should fail"
        );
    }

    /// Flow: Create launch with quality_score > 100 (should fail)
    #[flow]
    fn vault_create_launch_invalid_score(&mut self) {
        let authority = self.fuzz_accounts.authority.get_or_create_account(
            3,
            &mut self.trident,
            10_000_000_000,
        );
        let launch_pda = Pubkey::find_program_address(
            &[b"launch", authority.as_ref(), b"BadScore"],
            &claw_shark::program_id(),
        )
        .0;

        let ix = claw_shark::CreateLaunchInstruction::data(
            claw_shark::CreateLaunchInstructionData::new(
                "BadScore".to_string(),
                100_000_000,
                None,
                i64::MAX,
                255, // invalid: > 100
            ),
        )
        .accounts(claw_shark::CreateLaunchInstructionAccounts::new(
            launch_pda, authority,
        ))
        .instruction();

        let result = self.trident.process_instruction(ix, &[&authority]);
        assert!(
            result.is_err(),
            "create_launch with score > 100 should fail"
        );
    }

    /// Flow: Commit funds with 0 amount (should fail)
    #[flow]
    fn vault_commit_zero_funds(&mut self) {
        let funder = self.fuzz_accounts.funder.get_or_create_account(
            0,
            &mut self.trident,
            10_000_000_000,
        );
        let launch = self
            .fuzz_accounts
            .launch
            .get_or_create_account(1, &mut self.trident, 0);
        let commitment = self
            .fuzz_accounts
            .commitment
            .get_or_create_account(0, &mut self.trident, 0);
        let vault_token = self
            .fuzz_accounts
            .vault_token_account
            .get_or_create_account(0, &mut self.trident, 0);
        let funder_token = self
            .fuzz_accounts
            .funder_token_account
            .get_or_create_account(0, &mut self.trident, 0);

        let ix = claw_shark::CommitFundsInstruction::data(
            claw_shark::CommitFundsInstructionData::new(0), // zero amount
        )
        .accounts(claw_shark::CommitFundsInstructionAccounts::new(
            launch,
            commitment,
            funder,
            vault_token,
            funder_token,
        ))
        .instruction();

        let result = self.trident.process_instruction(ix, &[&funder]);
        assert!(
            result.is_err(),
            "commit_funds with 0 amount should fail"
        );
    }

    /// Flow: Finalize launch before deadline (should fail)
    #[flow]
    fn vault_finalize_before_deadline(&mut self) {
        let authority = self.fuzz_accounts.authority.get_or_create_account(
            0,
            &mut self.trident,
            10_000_000_000,
        );
        let launch = self
            .fuzz_accounts
            .launch
            .get_or_create_account(1, &mut self.trident, 0);

        let ix = claw_shark::FinalizeLaunchInstruction::data(
            claw_shark::FinalizeLaunchInstructionData::new(),
        )
        .accounts(claw_shark::FinalizeLaunchInstructionAccounts::new(
            launch, authority,
        ))
        .instruction();

        let result = self.trident.process_instruction(ix, &[&authority]);
        // If launch deadline is in the future, this should fail
        // with LaunchNotExpired
        assert!(
            result.is_err(),
            "finalize_launch before deadline should fail"
        );
    }

    /// Flow: Record agent score with score > 100 (should fail)
    #[flow]
    fn vault_record_agent_invalid_score(&mut self) {
        let oracle = self.fuzz_accounts.oracle.get_or_create_account(
            0,
            &mut self.trident,
            10_000_000_000,
        );
        let launch = self
            .fuzz_accounts
            .launch
            .get_or_create_account(1, &mut self.trident, 0);
        let agent_record = self
            .fuzz_accounts
            .agent_record
            .get_or_create_account(0, &mut self.trident, 0);

        let ix = claw_shark::RecordAgentScoreInstruction::data(
            claw_shark::RecordAgentScoreInstructionData::new(
                "agent-fuzz-1".to_string(),
                255, // invalid: > 100
                3,
            ),
        )
        .accounts(claw_shark::RecordAgentScoreInstructionAccounts::new(
            launch,
            agent_record,
            oracle,
        ))
        .instruction();

        let result = self.trident.process_instruction(ix, &[&oracle]);
        assert!(
            result.is_err(),
            "record_agent_score with score > 100 should fail"
        );
    }

    /// Flow: Record agent score with verdict > 6 (should fail)
    #[flow]
    fn vault_record_agent_invalid_verdict(&mut self) {
        let oracle = self.fuzz_accounts.oracle.get_or_create_account(
            1,
            &mut self.trident,
            10_000_000_000,
        );
        let launch = self
            .fuzz_accounts
            .launch
            .get_or_create_account(1, &mut self.trident, 0);
        let agent_record = self
            .fuzz_accounts
            .agent_record
            .get_or_create_account(1, &mut self.trident, 0);

        let ix = claw_shark::RecordAgentScoreInstruction::data(
            claw_shark::RecordAgentScoreInstructionData::new(
                "agent-fuzz-2".to_string(),
                50,
                7, // invalid: > 6
            ),
        )
        .accounts(claw_shark::RecordAgentScoreInstructionAccounts::new(
            launch,
            agent_record,
            oracle,
        ))
        .instruction();

        let result = self.trident.process_instruction(ix, &[&oracle]);
        assert!(
            result.is_err(),
            "record_agent_score with verdict > 6 should fail"
        );
    }

    // =========================================================================
    // CROSS-FLOW SCENARIOS (Stateful)
    // =========================================================================

    /// Flow: Full bonding curve lifecycle (create -> buy -> sell -> get_price)
    #[flow]
    fn curve_full_lifecycle(&mut self) {
        let founder = self.fuzz_accounts.founder.get_or_create_account(
            3,
            &mut self.trident,
            10_000_000_000,
        );
        let launch_pda = Pubkey::find_program_address(
            &[b"launch", founder.as_ref(), b"LifecycleToken"],
            &claw_shark_curve::program_id(),
        )
        .0;

        // Step 1: Create launch
        let create_ix = claw_shark_curve::CreateLaunchInstruction::data(
            claw_shark_curve::CreateLaunchInstructionData::new(
                "LifecycleToken".to_string(),
                "LIFE".to_string(),
                1_000_000_000_000,
                1_000_000,
                50,
                69_000_000_000,
            ),
        )
        .accounts(claw_shark_curve::CreateLaunchInstructionAccounts::new(
            launch_pda, founder,
        ))
        .instruction();

        let _ = self.trident.process_instruction(create_ix, &[&founder]);

        // Step 2: Buy tokens
        let buyer = self.fuzz_accounts.buyer.get_or_create_account(
            3,
            &mut self.trident,
            10_000_000_000,
        );
        let purchase_pda = Pubkey::find_program_address(
            &[
                b"purchase",
                launch_pda.as_ref(),
                buyer.as_ref(),
                &0u64.to_le_bytes(),
            ],
            &claw_shark_curve::program_id(),
        )
        .0;

        let buy_ix = claw_shark_curve::BuyTokensInstruction::data(
            claw_shark_curve::BuyTokensInstructionData::new(10_000_000), // 10 USDC
        )
        .accounts(claw_shark_curve::BuyTokensInstructionAccounts::new(
            launch_pda,
            purchase_pda,
            Pubkey::default(),
            buyer,
        ))
        .instruction();

        let _ = self.trident.process_instruction(buy_ix, &[&buyer]);

        // Step 3: Get price (should reflect purchased tokens)
        let price_ix = claw_shark_curve::GetPriceInstruction::data(
            claw_shark_curve::GetPriceInstructionData::new(),
        )
        .accounts(claw_shark_curve::GetPriceInstructionAccounts::new(
            launch_pda,
        ))
        .instruction();

        let _ = self.trident.process_instruction(price_ix, &[]);

        // Step 4: Sell tokens
        let sell_ix = claw_shark_curve::SellTokensInstruction::data(
            claw_shark_curve::SellTokensInstructionData::new(1_000_000), // sell some tokens
        )
        .accounts(claw_shark_curve::SellTokensInstructionAccounts::new(
            launch_pda, buyer,
        ))
        .instruction();

        let _ = self.trident.process_instruction(sell_ix, &[&buyer]);
    }

    /// Flow: Vault deposit/withdraw cycle (create -> deposit -> withdraw)
    #[flow]
    fn vault_deposit_withdraw_cycle(&mut self) {
        let owner = self.fuzz_accounts.owner.get_or_create_account(
            2,
            &mut self.trident,
            10_000_000_000,
        );
        let vault_pda = Pubkey::find_program_address(
            &[b"vault", owner.as_ref()],
            &claw_shark_curve::program_id(),
        )
        .0;

        // Create vault
        let create_ix = claw_shark_curve::CreateVaultInstruction::data(
            claw_shark_curve::CreateVaultInstructionData::new(
                5_000_000_000,
                20_000_000_000,
                70,
                false,
            ),
        )
        .accounts(claw_shark_curve::CreateVaultInstructionAccounts::new(
            vault_pda, owner,
        ))
        .instruction();

        let _ = self.trident.process_instruction(create_ix, &[&owner]);

        // Deposit
        let deposit_ix = claw_shark_curve::DepositVaultInstruction::data(
            claw_shark_curve::DepositVaultInstructionData::new(1_000_000_000), // 1K USDC
        )
        .accounts(claw_shark_curve::DepositVaultInstructionAccounts::new(
            vault_pda, owner,
        ))
        .instruction();

        let _ = self.trident.process_instruction(deposit_ix, &[&owner]);

        // Withdraw exact balance
        let withdraw_ix = claw_shark_curve::WithdrawVaultInstruction::data(
            claw_shark_curve::WithdrawVaultInstructionData::new(1_000_000_000),
        )
        .accounts(claw_shark_curve::WithdrawVaultInstructionAccounts::new(
            vault_pda, owner,
        ))
        .instruction();

        let _ = self.trident.process_instruction(withdraw_ix, &[&owner]);

        // Try to withdraw again from empty vault (should fail)
        let withdraw_again_ix = claw_shark_curve::WithdrawVaultInstruction::data(
            claw_shark_curve::WithdrawVaultInstructionData::new(1),
        )
        .accounts(claw_shark_curve::WithdrawVaultInstructionAccounts::new(
            vault_pda, owner,
        ))
        .instruction();

        let result = self.trident.process_instruction(withdraw_again_ix, &[&owner]);
        assert!(
            result.is_err(),
            "second withdrawal from empty vault should fail"
        );
    }

    // =========================================================================
    // PRICE CALCULATION EDGE CASES
    // =========================================================================

    /// Flow: Test bonding curve with slope = 0 (flat price)
    #[flow]
    fn curve_flat_price(&mut self) {
        let founder = self.fuzz_accounts.founder.get_or_create_account(
            4,
            &mut self.trident,
            10_000_000_000,
        );
        let launch_pda = Pubkey::find_program_address(
            &[b"launch", founder.as_ref(), b"FlatPrice"],
            &claw_shark_curve::program_id(),
        )
        .0;

        let ix = claw_shark_curve::CreateLaunchInstruction::data(
            claw_shark_curve::CreateLaunchInstructionData::new(
                "FlatPrice".to_string(),
                "FLAT".to_string(),
                1_000_000_000,
                1_000_000,
                0, // zero slope = constant price
                69_000_000_000,
            ),
        )
        .accounts(claw_shark_curve::CreateLaunchInstructionAccounts::new(
            launch_pda, founder,
        ))
        .instruction();

        let _ = self.trident.process_instruction(ix, &[&founder]);
    }

    /// Flow: Test bonding curve with initial_price = 0 (potential division issue)
    #[flow]
    fn curve_zero_initial_price(&mut self) {
        let founder = self.fuzz_accounts.founder.get_or_create_account(
            5,
            &mut self.trident,
            10_000_000_000,
        );
        let launch_pda = Pubkey::find_program_address(
            &[b"launch", founder.as_ref(), b"FreeToken"],
            &claw_shark_curve::program_id(),
        )
        .0;

        // initial_price = 0 could cause division by zero in buy_tokens
        let create_ix = claw_shark_curve::CreateLaunchInstruction::data(
            claw_shark_curve::CreateLaunchInstructionData::new(
                "FreeToken".to_string(),
                "FREE".to_string(),
                1_000_000_000,
                0, // zero initial price -- division by zero risk
                100,
                69_000_000_000,
            ),
        )
        .accounts(claw_shark_curve::CreateLaunchInstructionAccounts::new(
            launch_pda, founder,
        ))
        .instruction();

        let _ = self.trident.process_instruction(create_ix, &[&founder]);

        // Now buy tokens with zero initial price -> tokens_out = usdc_amount * 1_000_000 / 0
        let buyer = self.fuzz_accounts.buyer.get_or_create_account(
            4,
            &mut self.trident,
            10_000_000_000,
        );
        let purchase_pda = Pubkey::find_program_address(
            &[
                b"purchase",
                launch_pda.as_ref(),
                buyer.as_ref(),
                &0u64.to_le_bytes(),
            ],
            &claw_shark_curve::program_id(),
        )
        .0;

        let buy_ix = claw_shark_curve::BuyTokensInstruction::data(
            claw_shark_curve::BuyTokensInstructionData::new(1_000_000),
        )
        .accounts(claw_shark_curve::BuyTokensInstructionAccounts::new(
            launch_pda,
            purchase_pda,
            Pubkey::default(),
            buyer,
        ))
        .instruction();

        let result = self.trident.process_instruction(buy_ix, &[&buyer]);
        assert!(
            result.is_err(),
            "buy_tokens with 0 initial price should fail (division by zero)"
        );
    }

    /// Flow: Sell tokens that would cause usdc_collected underflow
    #[flow]
    fn curve_sell_usdc_underflow(&mut self) {
        let founder = self.fuzz_accounts.founder.get_or_create_account(
            6,
            &mut self.trident,
            10_000_000_000,
        );
        let launch_pda = Pubkey::find_program_address(
            &[b"launch", founder.as_ref(), b"UnderflowTest"],
            &claw_shark_curve::program_id(),
        )
        .0;

        // Create launch
        let create_ix = claw_shark_curve::CreateLaunchInstruction::data(
            claw_shark_curve::CreateLaunchInstructionData::new(
                "UnderflowTest".to_string(),
                "UND".to_string(),
                1_000_000_000_000,
                1_000_000,
                50,
                69_000_000_000,
            ),
        )
        .accounts(claw_shark_curve::CreateLaunchInstructionAccounts::new(
            launch_pda, founder,
        ))
        .instruction();

        let _ = self.trident.process_instruction(create_ix, &[&founder]);

        // Buy a small amount
        let buyer = self.fuzz_accounts.buyer.get_or_create_account(
            5,
            &mut self.trident,
            10_000_000_000,
        );
        let purchase_pda = Pubkey::find_program_address(
            &[
                b"purchase",
                launch_pda.as_ref(),
                buyer.as_ref(),
                &0u64.to_le_bytes(),
            ],
            &claw_shark_curve::program_id(),
        )
        .0;

        let buy_ix = claw_shark_curve::BuyTokensInstruction::data(
            claw_shark_curve::BuyTokensInstructionData::new(1_000_000), // 1 USDC
        )
        .accounts(claw_shark_curve::BuyTokensInstructionAccounts::new(
            launch_pda,
            purchase_pda,
            Pubkey::default(),
            buyer,
        ))
        .instruction();

        let _ = self.trident.process_instruction(buy_ix, &[&buyer]);

        // Try to sell a huge amount of tokens (more than possible, causing
        // usdc_collected underflow after fee subtraction)
        let sell_ix = claw_shark_curve::SellTokensInstruction::data(
            claw_shark_curve::SellTokensInstructionData::new(u64::MAX / 2),
        )
        .accounts(claw_shark_curve::SellTokensInstructionAccounts::new(
            launch_pda, buyer,
        ))
        .instruction();

        let result = self.trident.process_instruction(sell_ix, &[&buyer]);
        assert!(
            result.is_err(),
            "sell exceeding tokens_sold should fail with underflow"
        );
    }

    #[end]
    fn end(&mut self) {
        // Cleanup at end of each iteration
    }
}

fn main() {
    // Run 2000 iterations, each with up to 50 instruction flows
    FuzzTest::fuzz(2000, 50);
}
