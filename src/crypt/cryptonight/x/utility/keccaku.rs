//! # Keccak Utilites
//! 
//! This module contains utility functions for Keccak hashing.

use sha3::{Digest, Keccak256};

pub fn keccak(data: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().as_slice().to_vec()
}

const ROUNDS: usize = 24;
const LANE_SIZE: usize = 8;
const STATE_SIZE: usize = 25;

pub fn keccak_f(state: &mut [u8; STATE_SIZE * LANE_SIZE]) {
    for round in 0..ROUNDS {
        theta(state);
        rho_pi(state);
        chi(state);
        iota(state, round);
    }
}

fn theta(state: &mut [u8; STATE_SIZE * LANE_SIZE]) {
    let mut c = [0u8; LANE_SIZE];
    let mut d = [0u8; LANE_SIZE];

    for x in 0..STATE_SIZE {
        for y in 0..LANE_SIZE {
            c[y] ^= state[index(x, y)];
        }
    }

    for x in 0..STATE_SIZE {
        for y in 0..LANE_SIZE {
            d[y] = c[(y + LANE_SIZE - 1) % LANE_SIZE] ^
                   rotate_left(c[(y + 1) % LANE_SIZE], 1);
        }

        for y in 0..LANE_SIZE {
            for z in 0..LANE_SIZE {
                state[index(x, y)] ^= d[z];
            }
        }
    }
}

fn rho_pi(state: &mut [u8; STATE_SIZE * LANE_SIZE]) {
    let mut temp = [0u8; LANE_SIZE];
    let mut x = 1;
    let mut y = 0;

    for t in 0..24 {
        for z in 0..LANE_SIZE {
            temp[z] = state[index(x, y)];
        }

        for t in 0..LANE_SIZE {
            state[index(x, y)] = rotate_left(temp[(t + 1) % LANE_SIZE], (t * (t + 1) / 2) as u32);
            x = y;
            y = (2 * x + 3 * y) % 5;
            x = (5 * x) % 5;
        }
    }
}

fn chi(state: &mut [u8; STATE_SIZE * LANE_SIZE]) {
    let mut temp = [0u8; LANE_SIZE];

    for y in 0..STATE_SIZE {
        for x in 0..5 {
            for z in 0..LANE_SIZE {
                temp[z] = state[index(x, y)];
            }

            for z in 0..LANE_SIZE {
                state[index(x, y)] ^= (!temp[(z + 1) % LANE_SIZE]) & temp[(z + 2) % LANE_SIZE];
            }
        }
    }
}

fn iota(state: &mut [u8; STATE_SIZE * LANE_SIZE], round: usize) {
    state[index(0, 0)] ^= RC[round] as u8;
}

fn index(x: usize, y: usize) -> usize {
    x * LANE_SIZE + y
}

fn rotate_left(value: u8, shift: u32) -> u8 {
    (value << shift) | (value >> (LANE_SIZE as u32 - shift))
}

// Round Constants
const RC: [u64; ROUNDS] = [
    0x0000000000000001, 0x0000000000008082, 0x800000000000808a,
    0x8000000080008000, 0x000000000000808b, 0x0000000080000001,
    0x8000000080008081, 0x8000000000008009, 0x000000000000008a,
    0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
    0x000000008000808b, 0x800000000000008b, 0x8000000000008089,
    0x8000000000008003, 0x8000000000008002, 0x8000000000000080,
    0x000000000000800a, 0x800000008000000a, 0x8000000080008081,
    0x8000000000008080, 0x0000000080000001, 0x8000000080008008
];