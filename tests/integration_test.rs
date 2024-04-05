#[cfg(test)]
mod tests {
    use libmonero::keys::{derive_address, derive_hex_seed, derive_priv_keys, derive_pub_key, generate_seed};
    use libmonero::crypt::cryptonight::cn_slow_hash_original;

    #[test]
    fn seed_generation() {
        let seed = generate_seed("en", "original");
        assert_ne!(seed, vec!["tissue", "raking", "haunted", "huts", "afraid", "volcano", "howls", "liar", "egotistic", "befit", "rounded", "older", "bluntly", "imbalance", "pivot", "exotic", "tuxedo", "amaze", "mostly", "lukewarm", "macro", "vocal", "hounded", "biplane", "rounded"].iter().map(|&s| s.to_string()).collect::<Vec<String>>());
    }
    
    #[test]
    fn key_derivation() {
        // five saved himself oust taunts pebbles fibula organs koala copy dying vein damp dauntless code gags copy roster geek toolbox joyous apart unlikely warped taunts
        let mnemonic = ["five", "saved", "himself", "oust", "taunts", "pebbles", "fibula", "organs", "koala", "copy", "dying", "vein", "damp", "dauntless", "code", "gags", "copy", "roster", "geek", "toolbox", "joyous", "apart", "unlikely", "warped", "taunts"].to_vec().iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let hex_seed = derive_hex_seed(mnemonic);
        assert_eq!(hex_seed.clone(), "6bdaf7a0a8f3f1ce4767d6d9c38b72b48ccc3ffa4f60be91389b1b96403ff20e".to_string());
        let priv_keys = derive_priv_keys(hex_seed);
        let priv_sk = &priv_keys[0];
        let priv_vk = &priv_keys[1];
        assert_eq!(priv_sk, &"6bdaf7a0a8f3f1ce4767d6d9c38b72b48ccc3ffa4f60be91389b1b96403ff20e".to_string());
        assert_eq!(priv_vk, &"490447bf98677377923b4da400fa2b7e6dff6dff0ca24f7ae533a8207fd27c00".to_string());
        let pub_sk = derive_pub_key(priv_sk.clone());
        assert_eq!(pub_sk.clone(), "03970285bf0724d75e0f50bca9a9ea0e8db5091b69403dc944465f8936bde787".to_string());
        let pub_vk = derive_pub_key(priv_keys[1].clone());
        assert_eq!(pub_vk.clone(), "528a736a5079dc9536edb5b6fa0a5209ce820b9734fc0785024670b3d3ba4c69".to_string());
        let addr = derive_address(pub_sk, pub_vk, 0);
        assert_eq!(addr, "41kztevQ9HVd2LMni56Ka13SBt6k9qFH6afYGWyXfWnJPdoEE86mHddRxZxPtAwdZb2e8wsZdiFyxPFMTtaWp14PCxPF3wT".to_string());
    }

    #[cfg(test)]
    #[allow(warnings)]
    fn hashing_cn_slow_original() {
        let input = b"This is a test";
        let output = cn_slow_hash_original(input);
        assert_eq!(
            output,
            "a084f01d1437a09c6985401b60d43554ae105802c5f5d8a9b3253649c0be6605".to_string()
        );
    }
}