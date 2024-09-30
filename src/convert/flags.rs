pub struct EntryFlags(u32);

impl EntryFlags {
    const ANIME: u32 = 1 << 0;
    const UNVERIFIED: u32 = 1 << 1;
    const EXTERNAL: u32 = 1 << 2;
    const MOVIE: u32 = 1 << 3;
    const ADULT: u32 = 1 << 4;

    pub const fn new(flag_value: u32) -> Self {
        EntryFlags(flag_value)
    }

    #[inline]
    fn has_flag(&self, val: u32) -> bool {
        (self.0 & val) == val
    }

    pub fn is_anime(&self) -> bool {
        self.has_flag(Self::ANIME)
    }

    pub fn is_unverified(&self) -> bool {
        self.has_flag(Self::UNVERIFIED)
    }

    pub fn is_external(&self) -> bool {
        self.has_flag(Self::EXTERNAL)
    }

    pub fn is_movie(&self) -> bool {
        self.has_flag(Self::MOVIE)
    }

    pub fn is_adult(&self) -> bool {
        self.has_flag(Self::ADULT)
    }
}
