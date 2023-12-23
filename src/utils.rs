use regex::Regex;

// Returns true if given address is a valid Monero address
pub fn is_valid_addr(address: &str) -> bool {
    let r = Regex::new(r"^(4|8)[0-9a-zA-Z]{95}$").unwrap();
    r.is_match(address)
}