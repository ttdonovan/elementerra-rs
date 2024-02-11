use super::*;

use crate::{programs::elementerra::state, Player};

pub fn player_accounts<C: Deref<Target = impl Signer> + Clone>(
    program: &Program<C>,
    authority: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, Player)>> {
    let accounts = program.accounts::<state::Player>(vec![RpcFilterType::Memcmp(
        Memcmp::new_base58_encoded(10, authority.as_ref()),
    )])?;

    let player_accounts = accounts
        .iter()
        .map(|(pubkey, account)| (*pubkey, Player(account.clone())))
        .collect();

    Ok(player_accounts)
}
