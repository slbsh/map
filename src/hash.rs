pub trait Hash {
    fn as_hash_bytes(&self) -> &[u8];
    fn bytes_len(&self) -> usize;

    fn hash(&self, level: u8) -> u64 {
        let hash = match level {
            0 => self.hash0(),

            1 => self.hash1(),

            2 if self.bytes_len() % 2 == 0 => self.hash2(),
            2 => self.hash1(),

            3 => self.hash2(),
            _ => unsafe{ std::hint::unreachable_unchecked() },
        };

        match hash { 
            0 => 1,
            _ => hash,
        }
    }

    // [u8; 8]
    fn hash0(&self) -> u64 {
        let bytes = self.as_hash_bytes();
        let mut hash = 0u64;

        for i in 0..8 {
            match bytes.get(i) {
                Some(b) => hash |= (*b as u64) << (i << 3),
                None => break,
            }
        }

        hash
    }

    // FNV-1a <- [u8; 2]
    fn hash1(&self) -> u64 {
        const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
        const FNV_PRIME: u64        = 0x00000100000001b3;

        let bytes = self.as_hash_bytes();
        let mut hash = FNV_OFFSET_BASIS;

        let mut i = 0;
        while i < bytes.len() {
            match i + 1 < bytes.len() {
                true  => hash ^= bytes[i] as u64 | ((bytes[i+1] as u64) << 8),
                false => hash ^= bytes[i] as u64,
            }

            hash = hash.wrapping_mul(FNV_PRIME);
            i += 2;
        }
        
        hash
    }

    fn hash2(&self) -> u64 {
        todo!()
    }
}
