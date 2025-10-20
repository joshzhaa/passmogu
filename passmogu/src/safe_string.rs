use zeroize::Zeroizing;

/// SafeString zeroizes the heap allocated u8 slice when dropped. We're only supporting ASCII,
/// and we want to prohibit reallocations. However, it should be safe to use Vec<SafeString>
/// because reallocations only move the Box pointer around (not leaving secret strings behind).
pub type SafeString = Zeroizing<Box<[u8]>>;
