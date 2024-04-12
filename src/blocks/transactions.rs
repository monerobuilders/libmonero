/*
 * This file is part of Monero Builders' library libmonero
 *
 * Copyright (c) 2023-2024, Monero Builders (monero.builders)
 * All Rights Reserved
 * The code is distributed under MIT license, see LICENSE file for details.
 * Generated by Monume
 *
 */

use std::collections::HashMap;

/// Transactions struct contains all the information about a single transaction
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub timestamp: u64,
    pub block_height: u64,
    pub tx_hash: String,
    pub tx_fee: u64,
    pub additional_data: HashMap<String, String>
}

/* 
pub fn check_output() {
}
*/