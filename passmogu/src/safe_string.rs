use zeroize::Zeroizing;

// We're only supporting ASCII, and we want to prohibit reallocations.
pub type SafeString = Zeroizing<Box<[u8]>>;
