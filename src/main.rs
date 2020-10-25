use libc::{c_int, size_t};

use wfa_rs::bindings::*;
use wfa_rs::{affine_wavefront::*, mm_allocator::*, penalties::*};

fn main() {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let pattern = String::from("TCTTTACTCGCGCGTTGGAGAAATACAATAGT");
    let text = String::from("TCTATACTGCGCGTTTGGAGAAATAAAATAGT");

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    let pat_len = pattern.as_bytes().len();
    let text_len = text.as_bytes().len();

    let mut wavefronts = AffineWavefronts::new_complete(pat_len, text_len, &mut penalties, &alloc);

    wavefronts.align(pattern.as_bytes(), text.as_bytes());

    let score = wavefronts.edit_cigar_score(&mut penalties);

    println!("score: {}", score);
    wavefronts.print_cigar(pattern.as_bytes(), text.as_bytes());
}
