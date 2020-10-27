use wfa_rs::{affine_wavefront::*, bindings::*, mm_allocator::*, penalties::*};

fn new_complete<'a>(
    alloc: &'a MMAllocator,
    pattern_len: usize,
    text_len: usize,
    penalties: &mut AffinePenalties,
) -> AffineWavefronts<'a> {
    AffineWavefronts::new_complete(pattern_len, text_len, penalties, alloc)
}

fn run_complete<'a>(
    alloc: &'a MMAllocator,
    pattern_len: usize,
    text_len: usize,
    pattern: &str,
    text: &str,
    penalties: &mut AffinePenalties,
) -> AffineWavefronts<'a> {
    let mut wavefronts =
        AffineWavefronts::new_complete(pattern_len, text_len, penalties, alloc);

    wavefronts.align(pattern.as_bytes(), text.as_bytes());

    wavefronts
}

// TODO instead of panicking, return a Result
#[test]
#[should_panic]
fn empty_texts() {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let pattern = String::from("");
    let text = String::from("");

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    run_complete(&alloc, 0, 0, &pattern, &text, &mut penalties);
}

// TODO instead of panicking, return a Result
#[test]
#[should_panic]
fn longer_texts() {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let pattern = String::from("TCTTTACTCGCGCGTTGGAGAAATACAATAGT");
    let text = String::from("TCTATACTGCGCGTTTGGAGAAATAAAATAGT");

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    run_complete(&alloc, 10, 10, &pattern, &text, &mut penalties);
}

#[test]
fn shorter_texts() {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let pattern = String::from("TCTTTACTCGCGCGTTGGAGAAATACAATAGT");
    let text = String::from("TCTATACTGCGCGTTTGGAGAAATAAAATAGT");

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    let mut wavefronts = new_complete(
        &alloc,
        pattern.len() + 10,
        text.len() + 10,
        &mut penalties,
    );

    wavefronts.align(pattern.as_bytes(), text.as_bytes());

    let score = wavefronts.edit_cigar_score(&mut penalties);
    assert_eq!(score, -24);

    let cigar = wavefronts.cigar_bytes();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    assert_eq!("MMMXMMMMDMMMMMMMIMMMMMMMMMXMMMMMM", cg_str);
}

#[test]
fn wavefronts_complete_align() {
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

    let mut wavefronts = AffineWavefronts::new_complete(
        pat_len,
        text_len,
        &mut penalties,
        &alloc,
    );

    wavefronts.align(pattern.as_bytes(), text.as_bytes());

    let score = wavefronts.edit_cigar_score(&mut penalties);
    assert_eq!(score, -24);

    let cigar = wavefronts.cigar_bytes();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    assert_eq!("MMMXMMMMDMMMMMMMIMMMMMMMMMXMMMMMM", cg_str);
}

#[test]
fn wavefronts_reduced_align() {
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

    let mut wavefronts = AffineWavefronts::new_reduced(
        pat_len,
        text_len,
        &mut penalties,
        10,
        50,
        &alloc,
    );

    wavefronts.align(pattern.as_bytes(), text.as_bytes());

    let score = wavefronts.edit_cigar_score(&mut penalties);
    assert_eq!(score, -24);

    let cigar = wavefronts.cigar_bytes();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    assert_eq!("MMMXMMMMDMMMMMMMIMMMMMMMMMXMMMMMM", cg_str);
}
