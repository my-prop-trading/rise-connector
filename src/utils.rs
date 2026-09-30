const ETH_ADDRESS_HEX_LEN: usize = 40;

/// `0x` + 40 hex chars; RiseID and RiseAccount both use this form.
pub fn is_eth_address(value: &str) -> bool {
    let Some(hex) = value.strip_prefix("0x") else {
        return false;
    };

    hex.len() == ETH_ADDRESS_HEX_LEN && hex.chars().all(|c| c.is_ascii_hexdigit())
}
