// A struct with fields of different sizes
#[allow(dead_code)]
struct MixedData {
    a: u8,  // 1 byte
    b: u32, // 4 bytes
    c: u16, // 2 bytes
    d: u64, // 8 bytes
}

// The same struct, but with C-compatible layout
#[repr(C)]
#[allow(dead_code)]
struct MixedCData {
    a: u8,  // 1 byte
    b: u32, // 4 bytes
    c: u16, // 2 bytes
    d: u64, // 8 bytes
}

fn main() {
    println!("--- Default Rust Layout (MixedData) ---");
    println!("Size of MixedData: {} bytes", size_of::<MixedData>());
    println!("Alignment of MixedData: {} bytes", align_of::<MixedData>());
    // We cannot easily print the exact internal layout, but we can infer.
    // Rust might reorder a, c, b, d or other combinations to minimize padding.
    // For example, (u8, u16, u32, u64) might pack better than (u8, u32, u16, u64).
    //  A common strategy is to sort fields by decreasing alignment requirements.
    // So, it might arrange them like this:
    // d (u64) -> b (u32) -> c (u16) -> a (u8) = 16 bytes total.

    println!("\n--- C-Compatible Layout (MixedCData - #[repr(C)]) ---");
    println!("Size of MixedCData: {} bytes", size_of::<MixedCData>());
    println!(
        "Alignment of MixedCData: {} bytes",
        align_of::<MixedCData>()
    );

    // Let's manually trace the expected C layout for MixedCData:
    // Field `a`: u8 (1 byte), starts at offset 0
    // Field `b`: u32 (4 bytes), needs 4-byte alignment.
    //    Current offset is 1 (after 'a'). Needs 3 bytes of padding to get to offset 4.
    // Field `c`: u16 (2 bytes), needs 2-byte alignment.
    //    Current offset is 8 (after 'b'). Starts fine at 8.
    // Field `d`: u64 (8 bytes), needs 8-byte alignment.
    //    Current offset is 10 (after 'c'). Needs 6 bytes of padding to get to offset 16.
    // Total size so far is 16 + 8 = 24 bytes.
    // Overall struct alignment is max of fields: 8 bytes (from u64).
    // Total size must be multiple of 8. 24 is a multiple of 8.
    // So, expected size: 24 bytes. Expected alignment: 8 bytes.
}
