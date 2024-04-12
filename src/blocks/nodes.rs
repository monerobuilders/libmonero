/*
 * This file is part of Monero Builders' library libmonero
 *
 * Copyright (c) 2023-2024, Monero Builders (monero.builders)
 * All Rights Reserved
 * The code is distributed under MIT license, see LICENSE file for details.
 * Generated by Monume
 *
 */

/// DaemonNode struct contains all necessary and additional information about a daemon node
pub struct DaemonNode {
    pub url: String,
    pub port: u16,
    pub tls: bool,
}

/// DaemonNode functions etc.
impl DaemonNode {
    /// Returns Cake Wallet's default node
    pub fn cake_wallet_default() -> DaemonNode {
        DaemonNode {
            url: "xmr-node.cakewallet.com".to_string(),
            port: 18081,
            tls: false
        }
    }

    /// Returns Stack Wallet's default node
    pub fn stack_wallet_default() -> DaemonNode {
        DaemonNode {
            url: "monero.stackwallet.com".to_string(),
            port: 18081,
            tls: false
        }
    }

    /// Creates a new DaemonNode from a given URL, port and tls flag
    pub fn new(url: String, port: u16, tls: bool) -> DaemonNode {
        DaemonNode {
            url,
            port,
            tls
        }
    }
}