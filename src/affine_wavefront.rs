use libc::{c_int, size_t};
use std::ffi::CString;

use crate::bindings::*;

use crate::mm_allocator::MMAllocator;

use crate::penalties::AffinePenalties;

pub struct AffineWavefronts<'a> {
    ptr: *mut affine_wavefronts_t,
    allocator: &'a MMAllocator,
}

impl<'a> AffineWavefronts<'a> {
    pub fn new_complete(
        pattern_len: usize,
        text_len: usize,
        penalties: &mut AffinePenalties,
        alloc: &'a MMAllocator,
    ) -> Self {
        let pat_len = pattern_len as c_int;
        let text_len = text_len as c_int;
        let penalties_ptr = penalties.as_ptr();
        let stats_ptr = std::ptr::null_mut() as *mut wavefronts_stats_t;
        let ptr = unsafe {
            affine_wavefronts_new_complete(
                pat_len,
                text_len,
                penalties_ptr,
                stats_ptr,
                alloc.alloc_ptr(),
            )
        };
        AffineWavefronts {
            ptr,
            allocator: alloc,
        }
    }

    pub fn new_reduced(
        pattern_len: usize,
        text_len: usize,
        penalties: &mut AffinePenalties,
        min_wavefront_len: i32,
        min_dist_threshold: i32,
        alloc: &'a MMAllocator,
    ) -> Self {
        let pat_len = pattern_len as c_int;
        let text_len = text_len as c_int;
        let stats_ptr = std::ptr::null_mut() as *mut wavefronts_stats_t;

        let ptr = unsafe {
            affine_wavefronts_new_reduced(
                pat_len,
                text_len,
                penalties.as_ptr(),
                min_wavefront_len as c_int,
                min_dist_threshold as c_int,
                stats_ptr,
                alloc.alloc_ptr(),
            )
        };

        Self {
            ptr,
            allocator: alloc,
        }
    }

    pub fn align(&mut self, pattern: &[u8], text: &[u8]) {
        let pat_len = pattern.len() as c_int;
        let text_len = text.len() as c_int;
        let pattern = CString::new(pattern).unwrap();
        let text = CString::new(text).unwrap();

        unsafe {
            affine_wavefronts_align(self.ptr, pattern.as_ptr(), pat_len, text.as_ptr(), text_len);
        }
    }

    pub fn edit_cigar(&self) -> &edit_cigar_t {
        unsafe {
            let wf_ref = self.ptr.as_ref().unwrap();
            &wf_ref.edit_cigar
        }
    }

    pub fn edit_cigar_score(&mut self, penalties: &mut AffinePenalties) -> c_int {
        let penalties = penalties as *mut AffinePenalties;
        let penalties_ptr: *mut affine_penalties_t = penalties.cast();
        unsafe {
            let wf_ref = self.ptr.as_mut().unwrap();
            let cigar = &mut wf_ref.edit_cigar as *mut edit_cigar_t;
            edit_cigar_score_gap_affine(cigar, penalties_ptr)
        }
    }

    pub fn print_cigar(&mut self, pattern: &[u8], text: &[u8]) {
        let pat_len = pattern.len() as c_int;
        let text_len = text.len() as c_int;
        let pattern = CString::new(pattern).unwrap();
        let text = CString::new(text).unwrap();

        unsafe {
            let wf_ref = self.ptr.as_mut().unwrap();
            let cg_mut = &mut wf_ref.edit_cigar as *mut edit_cigar_t;
            edit_cigar_print_pretty(
                stderr,
                pattern.as_ptr(),
                pat_len,
                text.as_ptr(),
                text_len,
                cg_mut,
                self.allocator.alloc_ptr(),
            );
        }
    }
}

impl Drop for AffineWavefronts<'_> {
    fn drop(&mut self) {
        unsafe { affine_wavefronts_delete(self.ptr) }
    }
}
