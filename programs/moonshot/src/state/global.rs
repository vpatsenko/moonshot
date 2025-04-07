use crate::events::{GlobalUpdateEvent, IntoEvent};
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct GlobalAuthorityInput {
    pub global_authority: Option<Pubkey>,
    pub migration_authority: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace, Debug, PartialEq)]
pub enum ProgramStatus {
    Running,
    SwapOnly,
    SwapOnlyNoLaunch,
    Paused,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Global {
    pub status: ProgramStatus,
    pub initialized: bool,
    pub global_authority: Pubkey,    // can update settings
    pub migration_authority: Pubkey, // can migrate
    pub migrate_fee_amount: u64,
    pub fee_receiver: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub mint_decimals: u8,
    pub meteora_config: Pubkey,
    pub whitelist_enabled: bool,
}

impl Default for Global {
    fn default() -> Self {
        Self {
            status: ProgramStatus::Running,
            initialized: true,
            global_authority: Pubkey::default(),
            migration_authority: Pubkey::default(),
            fee_receiver: Pubkey::default(),
            // Pump.fun initial values
            initial_virtual_token_reserves: 1073000000000000,
            initial_virtual_sol_reserves: 30000000000,
            initial_real_token_reserves: 793100000000000,
            token_total_supply: 1000000000000000,
            mint_decimals: 6,
            migrate_fee_amount: 500,
            whitelist_enabled: true,
            meteora_config: Pubkey::default(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct GlobalSettingsInput {
    pub initial_virtual_token_reserves: Option<u64>,
    pub initial_virtual_sol_reserves: Option<u64>,
    pub initial_real_token_reserves: Option<u64>,
    pub token_total_supply: Option<u64>,
    pub mint_decimals: Option<u8>,
    pub migrate_fee_amount: Option<u64>,
    pub fee_receiver: Option<Pubkey>,
    pub status: Option<ProgramStatus>,
    pub whitelist_enabled: Option<bool>,
    pub meteora_config: Option<Pubkey>,
}

impl Global {
    pub const SEED_PREFIX: &'static str = "global";

    pub fn get_signer<'a>(bump: &'a u8) -> [&'a [u8]; 2] {
        let prefix_bytes = Self::SEED_PREFIX.as_bytes();
        let bump_slice: &'a [u8] = std::slice::from_ref(bump);
        [prefix_bytes, bump_slice]
    }

    pub fn update_settings(&mut self, params: GlobalSettingsInput) {
        if let Some(mint_decimals) = params.mint_decimals {
            self.mint_decimals = mint_decimals;
        }
        if let Some(status) = params.status {
            self.status = status;
        }
        if let Some(initial_virtual_token_reserves) = params.initial_virtual_token_reserves {
            self.initial_virtual_token_reserves = initial_virtual_token_reserves;
        }
        if let Some(initial_virtual_sol_reserves) = params.initial_virtual_sol_reserves {
            self.initial_virtual_sol_reserves = initial_virtual_sol_reserves;
        }
        if let Some(initial_real_token_reserves) = params.initial_real_token_reserves {
            self.initial_real_token_reserves = initial_real_token_reserves;
        }
        if let Some(token_total_supply) = params.token_total_supply {
            self.token_total_supply = token_total_supply;
        }
        if let Some(migrate_fee_amount) = params.migrate_fee_amount {
            self.migrate_fee_amount = migrate_fee_amount;
        }
        if let Some(fee_receiver) = params.fee_receiver {
            self.fee_receiver = fee_receiver;
        }
        if let Some(whitelist_enabled) = params.whitelist_enabled {
            self.whitelist_enabled = whitelist_enabled;
        }
        if let Some(meteora_config) = params.meteora_config {
            self.meteora_config = meteora_config;
        }
    }

    pub fn update_authority(&mut self, params: GlobalAuthorityInput) {
        if let Some(global_authority) = params.global_authority {
            self.global_authority = global_authority;
        }
        if let Some(migration_authority) = params.migration_authority {
            self.migration_authority = migration_authority;
        }
    }
}

impl IntoEvent<GlobalUpdateEvent> for Global {
    fn into_event(&self) -> GlobalUpdateEvent {
        GlobalUpdateEvent {
            global_authority: self.global_authority,
            migration_authority: self.migration_authority,
            status: self.status,
            initial_virtual_token_reserves: self.initial_virtual_token_reserves,
            initial_virtual_sol_reserves: self.initial_virtual_sol_reserves,
            initial_real_token_reserves: self.initial_real_token_reserves,
            token_total_supply: self.token_total_supply,
            mint_decimals: self.mint_decimals,
        }
    }
}
