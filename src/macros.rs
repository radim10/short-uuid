/// Generate a short UUID v4 using default flickrBase58 alphabet
/// ## Usage
/// ```rust
/// use short_uuid::short;
/// let generated = short!();
/// ```
#[macro_export]
macro_rules! short {
    () => {
        $crate::ShortUuid::generate()
    };
}
