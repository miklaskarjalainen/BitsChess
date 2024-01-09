const SIZE_IN_KB: u64 = 32;
const ENTRY_COUNT: u64 = (1024*SIZE_IN_KB) / (std::mem::size_of::<RepetitionEntry>() as u64);

#[derive(Debug, Clone, Copy)]
struct RepetitionEntry {
    zobrist_hash: u64,
    repetitions: u8
}

impl RepetitionEntry {
    fn new() -> Self {
        RepetitionEntry {
            zobrist_hash: 0,
            repetitions: 0
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RepetitionTable {
    entries: [RepetitionEntry; ENTRY_COUNT as usize]
}

impl std::fmt::Display for RepetitionTable {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::from("{");
        
        for entry in &self.entries {
            if entry.zobrist_hash == 0 || entry.repetitions == 0 {
                continue;
            }
            str.push_str(format!(" {:?},", entry).as_str());
        }

        str.push(' ');
        str.push('}');

        formatter.pad(str.as_str())
    }
}

impl RepetitionTable {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            entries: [RepetitionEntry::new(); ENTRY_COUNT as usize]
        }
    }

    #[inline(always)]
    pub fn increment_existing_repetition(&mut self, hash: u64) -> bool {
        let key = (hash % ENTRY_COUNT) as usize;
        if self.entries[key].zobrist_hash == hash {
            self.entries[key].repetitions += 1;
            return true;
        }
        false
    }

    #[inline(always)]
    pub fn increment_repetition(&mut self, hash: u64) -> bool {
        let key = (hash % ENTRY_COUNT) as usize;
        
        // increment existing
        if self.entries[key].zobrist_hash == hash {
            self.entries[key].repetitions += 1;
            return true;
        }

        // overwrites
        self.entries[key].zobrist_hash = hash;
        self.entries[key].repetitions = 1;
        true
    }

    #[inline(always)]
    pub fn decrement_repetition(&mut self, hash: u64) -> bool {
        let key = (hash % ENTRY_COUNT) as usize;
        if self.entries[key].zobrist_hash == hash {
            self.entries[key].repetitions -= 1;
            return true;
        }
        false
    }

    #[inline(always)]
    pub const fn get_repetitions(&self, hash: u64) -> Option<u8> {
        let key = (hash % ENTRY_COUNT) as usize;
        if self.entries[key].zobrist_hash == hash {
            return Some(self.entries[key].repetitions);
        }
        None
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        for i in &mut self.entries {
            *i = RepetitionEntry::new();
        }
    }
}

