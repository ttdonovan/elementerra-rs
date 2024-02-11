use super::*;

use crate::{programs::elementerra::state, Season};

pub fn current_season_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
) -> anyhow::Result<Vec<(Pubkey, Season)>> {
    let accounts = program.accounts::<state::Season>(vec![RpcFilterType::Memcmp(
        Memcmp::new_base58_encoded(10, &[true as u8]),
    )])?;

    let season_accounts = accounts
        .iter()
        .map(|(pubkey, account)| (*pubkey, Season(account.clone())))
        .collect();

    Ok(season_accounts)
}
