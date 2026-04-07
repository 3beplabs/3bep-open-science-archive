// SHA-256 — Implementacao Rust puro (zero crates externas)
//
// Soberania total: nenhuma dependencia criptografica externa.
// Implementacao fiel ao FIPS 180-4 (NIST, 2015).
//
// Referencia: https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf

/// Constantes de round (primeiros 32 bits da parte fracionaria das
/// raizes cubicas dos primeiros 64 primos)
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
    0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// Valores iniciais de hash (primeiros 32 bits da parte fracionaria
/// das raizes quadradas dos primeiros 8 primos)
const H_INIT: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

#[inline(always)]
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

#[inline(always)]
fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

#[inline(always)]
fn sigma0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

#[inline(always)]
fn sigma1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

#[inline(always)]
fn gamma0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

#[inline(always)]
fn gamma1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

/// Processa um bloco de 64 bytes (512 bits)
fn process_block(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w = [0u32; 64];

    // Preparar message schedule (W)
    for i in 0..16 {
        w[i] = u32::from_be_bytes([
            block[4*i], block[4*i+1], block[4*i+2], block[4*i+3]
        ]);
    }
    for i in 16..64 {
        w[i] = gamma1(w[i-2])
            .wrapping_add(w[i-7])
            .wrapping_add(gamma0(w[i-15]))
            .wrapping_add(w[i-16]);
    }

    // Variaveis de trabalho
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];

    // 64 rounds de compressao
    for i in 0..64 {
        let t1 = h.wrapping_add(sigma1(e))
                  .wrapping_add(ch(e, f, g))
                  .wrapping_add(K[i])
                  .wrapping_add(w[i]);
        let t2 = sigma0(a).wrapping_add(maj(a, b, c));

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }

    // Adicionar ao estado
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

/// Calcula o hash SHA-256 de um slice de bytes.
/// Retorna 32 bytes (256 bits) do digest.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut state = H_INIT;
    let bit_len = (data.len() as u64) * 8;

    // Processar blocos completos de 64 bytes
    let full_blocks = data.len() / 64;
    for i in 0..full_blocks {
        let mut block = [0u8; 64];
        block.copy_from_slice(&data[i*64..(i+1)*64]);
        process_block(&mut state, &block);
    }

    // Padding: adicionar bit '1', zeros, e comprimento em 64 bits big-endian
    let remainder = data.len() % 64;
    let mut last_block = [0u8; 64];
    last_block[..remainder].copy_from_slice(&data[full_blocks*64..]);
    last_block[remainder] = 0x80;

    if remainder >= 56 {
        // Nao cabe o comprimento neste bloco, precisa de mais um
        process_block(&mut state, &last_block);
        last_block = [0u8; 64];
    }

    // Comprimento da mensagem em bits (big-endian, ultimos 8 bytes)
    last_block[56..64].copy_from_slice(&bit_len.to_be_bytes());
    process_block(&mut state, &last_block);

    // Converter estado para bytes
    let mut digest = [0u8; 32];
    for i in 0..8 {
        digest[4*i..4*i+4].copy_from_slice(&state[i].to_be_bytes());
    }
    digest
}

/// Calcula SHA-256 e retorna como string hexadecimal (64 caracteres)
pub fn sha256_hex(data: &[u8]) -> String {
    let digest = sha256(data);
    let mut hex = String::with_capacity(64);
    for byte in &digest {
        hex.push_str(&format!("{:02x}", byte));
    }
    hex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_empty() {
        // SHA-256("") = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        let result = sha256_hex(b"");
        assert_eq!(result, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    }

    #[test]
    fn test_sha256_abc() {
        // SHA-256("abc") = ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
        let result = sha256_hex(b"abc");
        assert_eq!(result, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
    }

    #[test]
    fn test_sha256_longer() {
        // SHA-256("abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")
        // = 248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1
        let result = sha256_hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq");
        assert_eq!(result, "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1");
    }
}
