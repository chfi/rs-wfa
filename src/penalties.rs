use libc::c_int;

#[repr(C)]
pub struct AffinePenalties {
    pub match_: c_int,
    pub mismatch: c_int,
    pub gap_opening: c_int,
    pub gap_extension: c_int,
}
