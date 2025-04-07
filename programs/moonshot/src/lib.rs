use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;
pub mod util;
use instructions::{
    add_wl::*, create_bonding_curve::*, create_pool::*, initialize::*, lock_pool::*, remove_wl::*,
    set_params::*, swap::*,
};
use state::bonding_curve::CreateBondingCurveParams;
use state::global::*;

declare_id!("GbguYRqMUzErdhvxLL2dNGqi8wLzWnkp87wd7MnCqkZ3");

#[program]
pub mod moonshot {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: GlobalSettingsInput) -> Result<()> {
        Initialize::handler(ctx, params)
    }

    pub fn set_params(ctx: Context<SetParams>, params: GlobalSettingsInput) -> Result<()> {
        SetParams::handler(ctx, params)
    }

    pub fn create_pool(ctx: Context<InitializePoolWithConfig>) -> Result<()> {
        instructions::initialize_pool_with_config(ctx)
    }

    pub fn lock_pool(ctx: Context<LockPool>) -> Result<()> {
        instructions::lock_pool(ctx)
    }

    pub fn add_wl(ctx: Context<AddWl>, new_creator: Pubkey) -> Result<()> {
        AddWl::handler(ctx, new_creator)
    }

    pub fn remove_wl(_ctx: Context<RemoveWl>) -> Result<()> {
        Ok(())
    }

    #[access_control(ctx.accounts.validate(&params))]
    pub fn create_bonding_curve(
        ctx: Context<CreateBondingCurve>,
        params: CreateBondingCurveParams,
    ) -> Result<()> {
        CreateBondingCurve::handler(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&params))]
    pub fn swap(ctx: Context<Swap>, params: SwapParams) -> Result<()> {
        Swap::handler(ctx, params)
    }
}
