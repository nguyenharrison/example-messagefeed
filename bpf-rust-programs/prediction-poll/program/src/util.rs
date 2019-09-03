use crate::result::{ProgramError, ProgramResult};
use prediction_poll_data::DataType;
use solana_sdk_bpf_utils::entrypoint::{SolKeyedAccount, SolPubkey};

// TODO retrieve this key from the `solana-sdk`
// https://github.com/solana-labs/example-messagefeed/issues/126
pub const CLOCK_KEY: [u8; 32] = [
    6, 167, 213, 23, 24, 199, 116, 201, 40, 86, 99, 152, 105, 29, 94, 182, 139, 94, 184, 163, 155,
    75, 109, 92, 115, 85, 91, 33, 0, 0, 0, 0,
];

pub fn expect_n_accounts(info: &mut [SolKeyedAccount], n: usize) -> ProgramResult<()> {
    if info.len() < n {
        Err(ProgramError::InvalidInput)
    } else {
        Ok(())
    }
}

pub fn expect_signed(account: &SolKeyedAccount) -> ProgramResult<()> {
    if !account.is_signer {
        return Err(ProgramError::MissingSigner);
    }
    Ok(())
}

pub fn expect_owned_by(account: &SolKeyedAccount, key: &SolPubkey) -> ProgramResult<()> {
    if account.owner != key {
        return Err(ProgramError::InvalidAccount);
    }
    Ok(())
}

pub fn expect_data_type(account: &SolKeyedAccount, data_type: DataType) -> ProgramResult<()> {
    if DataType::from(account.data[0]) as u8 != data_type as u8 {
        return Err(ProgramError::InvalidDataType);
    }
    Ok(())
}

pub fn expect_new_account(account: &SolKeyedAccount) -> ProgramResult<()> {
    expect_data_type(account, DataType::Unset).map_err(|_| ProgramError::AccountNotNew)
}

pub fn expect_key(account: &SolKeyedAccount, key: &SolPubkey) -> ProgramResult<()> {
    if account.key != key {
        return Err(ProgramError::InvalidKey);
    }
    Ok(())
}

pub fn expect_min_size(data: &[u8], min_size: usize) -> ProgramResult<()> {
    if data.len() < min_size {
        return Err(ProgramError::AccountDataTooSmall);
    }
    Ok(())
}

pub fn expect_gt<T: PartialOrd>(left: T, right: T) -> ProgramResult<()> {
    if left <= right {
        return Err(ProgramError::InvalidInput);
    }
    Ok(())
}