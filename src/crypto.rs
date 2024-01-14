use std::str;

/// Cryptographic helpers for generating login keys.
use byteorder::{BigEndian, ByteOrder, NativeEndian};
use getrandom::getrandom;
use num_bigint::{BigUint, ToBigUint};
use num_traits::Num;

const DIFFIE_HELLMAN_PRIME: &str = "eca2e8c85d863dcdc26a429a71a9815ad052f6139669dd659f98ae159d313d13c6bf2838e10a69b6478b64a24bd054ba8248e8fa778703b418408249440b2c1edd28853e240d8a7e49540b76d120d3b1ad2878b1b99490eb4a2a5e84caa8a91cecbdb1aa7c816e8be343246f80c637abc653b893fd91686cf8d32d6cfe5f2a6f";
const DIFFIE_HELLMAN_FUNCOM_PUBKEY: &str = "9c32cc23d559ca90fc31be72df817d0e124769e809f936bc14360ff4bed758f260a0d596584eacbbc2b88bdd410416163e11dbf62173393fbc0c6fefb2d855f1a03dec8e9f105bbad91b3437d8eb73fe2f44159597aa4053cf788d2f9d7012fb8d7c4ce3876f7d6cd5d0c31754f4cd96166708641958de54a6def5657b9f2e92";

/// Generates a login key based on username, password and login seed.
#[must_use]
pub fn generate_key(username: &str, password: &str, login_seed: &str) -> String {
    // Perform a diffie hellman key exchange with all parameters
    let dh_prime = BigUint::from_str_radix(DIFFIE_HELLMAN_PRIME, 16).unwrap();
    let dh_fc_key = BigUint::from_str_radix(DIFFIE_HELLMAN_FUNCOM_PUBKEY, 16).unwrap();
    let dh_generator = 5.to_biguint().unwrap();
    // Our diffie hellman secret key is really just random
    let mut num = [0; 32];
    getrandom(&mut num).unwrap();
    let dh_secret_key = BigUint::from_bytes_le(&num);

    // And we generate our public key for funcom to decode our credentials later
    let dh_pubkey = dh_generator.modpow(&dh_secret_key, &dh_prime);
    // The shared key is used for encryption
    let dh_shared = dh_fc_key.modpow(&dh_secret_key, &dh_prime);
    let mut dh_shared_str = dh_shared.to_str_radix(16);
    dh_shared_str.truncate(32);

    let login_string = format!("{username}|{login_seed}|{password}");

    let mut prefix_buf = vec![0; 8];
    getrandom(&mut prefix_buf).unwrap();

    let length = 8 + 4 + login_string.len();
    let mut to_be_encrypted_buf = Vec::with_capacity(length);
    to_be_encrypted_buf.append(&mut prefix_buf);
    // 32 is the space byte
    let mut pad = vec![32; (8 - length % 8) % 8];

    let mut buf = vec![0; 4];
    BigEndian::write_u32(&mut buf, login_string.len() as u32);
    to_be_encrypted_buf.append(&mut buf);
    to_be_encrypted_buf.extend_from_slice(login_string.as_bytes());
    to_be_encrypted_buf.append(&mut pad);

    let encrypted = encrypt(&dh_shared_str, &to_be_encrypted_buf);
    format!("{}-{}", dh_pubkey.to_str_radix(16), encrypted)
}

fn encrypt(key: &str, source: &[u8]) -> String {
    let key_arr = hex_string_to_le_array(key);
    let mut source_arr = vec![0; source.len() / 4];
    NativeEndian::read_u32_into(source, &mut source_arr);

    let mut ret = String::new();
    let mut cycle = [0; 2];

    for i in (0..source_arr.len()).step_by(2) {
        cycle[0] ^= source_arr[i];
        cycle[1] ^= source_arr[i + 1];

        let mut total: u32 = 0;
        let delta = 0x9e37_79b9;

        for _ in 0..32 {
            total = total.wrapping_add(delta);
            cycle[0] = cycle[0].wrapping_add(
                ((cycle[1] << 4 & 0xffff_fff0).wrapping_add(key_arr[0]))
                    ^ (cycle[1].wrapping_add(total))
                    ^ ((cycle[1] >> 5 & 0x07ff_ffff).wrapping_add(key_arr[1])),
            );
            cycle[0] &= 0xffff_ffff;
            cycle[1] = cycle[1].wrapping_add(
                ((cycle[0] << 4 & 0xffff_fff0).wrapping_add(key_arr[2]))
                    ^ (cycle[0].wrapping_add(total))
                    ^ ((cycle[0] >> 5 & 0x07ff_ffff).wrapping_add(key_arr[3])),
            );
            cycle[1] &= 0xffff_ffff;
        }

        ret += &format!("{:08x}", cycle[0].to_be());
        ret += &format!("{:08x}", cycle[1].to_be());
    }

    ret
}

fn hex_string_to_le_array(hex: &str) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::with_capacity(hex.len() / 8);
    for i in (0..hex.len()).step_by(8) {
        let substr = &hex[i..i + 8];
        let part = u32::from_str_radix(substr, 16).unwrap().swap_bytes();
        out.push(part);
    }
    out
}
