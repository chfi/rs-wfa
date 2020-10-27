use wfa_rs::{affine_wavefront::*, bindings::*, mm_allocator::*, penalties::*};

use quickcheck::*;

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
) -> Result<AffineWavefronts<'a>, WavefrontError> {
    let mut wavefronts =
        AffineWavefronts::new_complete(pattern_len, text_len, penalties, alloc);

    wavefronts.align(pattern.as_bytes(), text.as_bytes())?;

    Ok(wavefronts)
}

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

    let result = run_complete(&alloc, 0, 0, &pattern, &text, &mut penalties);
    assert!(result.is_ok());
}

#[test]
fn empty_wavefronts() {
    let alloc = MMAllocator::new(BUFFER_SIZE_8M as u64);

    let mut penalties = AffinePenalties {
        match_: 0,
        mismatch: 4,
        gap_opening: 6,
        gap_extension: 2,
    };

    let pattern = String::from("");
    let text = String::from("");

    let mut wavefronts = new_complete(
        &alloc,
        pattern.len() + 10,
        text.len() + 10,
        &mut penalties,
    );

    let result = wavefronts.align(pattern.as_bytes(), text.as_bytes());

    assert!(result.is_ok());

    let score = wavefronts.edit_cigar_score(&mut penalties);
    assert_eq!(score, 0);

    let cigar = wavefronts.cigar_bytes();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    assert_eq!("", cg_str);
}

#[test]
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

    let result = run_complete(&alloc, 10, 10, &pattern, &text, &mut penalties);

    assert!(result.is_err());
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

    let result = wavefronts.align(pattern.as_bytes(), text.as_bytes());

    assert!(result.is_ok());

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

    let result = wavefronts.align(pattern.as_bytes(), text.as_bytes());

    assert!(result.is_ok());

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

    let result = wavefronts.align(pattern.as_bytes(), text.as_bytes());

    assert!(result.is_ok());

    let score = wavefronts.edit_cigar_score(&mut penalties);
    assert_eq!(score, -24);

    let cigar = wavefronts.cigar_bytes();
    let cg_str = std::str::from_utf8(&cigar).unwrap();
    assert_eq!("MMMXMMMMDMMMMMMMIMMMMMMMMMXMMMMMM", cg_str);
}

fn gen_base<G: Gen>(g: &mut G) -> u8 {
    let base = u8::arbitrary(g) & 4;
    match base {
        0 => b'A',
        1 => b'G',
        2 => b'C',
        3 => b'T',
        _ => unreachable!(),
    }
}

fn generate_pattern<G: Gen>(g: &mut G, len: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(len);

    for _ in 0..len {
        let base = gen_base(g);
        result.push(base);
    }

    result
}

#[derive(Clone, Copy, Debug)]
enum AlignErrorKind {
    Mismatch,
    Insertion,
    Deletion,
}

impl Arbitrary for AlignErrorKind {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let kind = u8::arbitrary(g) % 3;
        match kind {
            0 => AlignErrorKind::Mismatch,
            1 => AlignErrorKind::Insertion,
            2 => AlignErrorKind::Deletion,
            _ => unreachable!(),
        }
    }
}

// based off of the WFA generate_dataset.c
fn generate_text<G: Gen>(
    g: &mut G,
    pattern: &[u8],
    error_degree: f64,
) -> Vec<u8> {
    assert!(pattern.len() > 0);
    let mut result = pattern.to_owned();

    let mut cigar = vec![b'M'; pattern.len()];

    let errors = (error_degree * pattern.len() as f64).ceil() as usize;

    for _ in 0..errors {
        match AlignErrorKind::arbitrary(g) {
            AlignErrorKind::Mismatch => {
                let index = usize::arbitrary(g) % result.len();
                let mut base = gen_base(g);
                while base == result[index] {
                    base = gen_base(g);
                }
                result[index] = base;
                cigar[index] = b'X';
            }
            AlignErrorKind::Insertion => {
                let index = usize::arbitrary(g) % result.len();
                let base = gen_base(g);
                result.insert(index, base);
                cigar.insert(index, b'I');
            }
            AlignErrorKind::Deletion => {
                let index = usize::arbitrary(g) % result.len();
                result.remove(index);
                result[index] = b'D';
            }
        }
    }

    result
}
