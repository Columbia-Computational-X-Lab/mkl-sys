#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

extern crate intel_mkl_src;

//---------------------------------------------------------

pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;
pub type max_align_t = u128;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct _MKL_Complex8 {
    pub real: f32,
    pub imag: f32,
}
#[test]
fn bindgen_test_layout__MKL_Complex8() {
    assert_eq!(
        ::core::mem::size_of::<_MKL_Complex8>(),
        8usize,
        concat!("Size of: ", stringify!(_MKL_Complex8))
    );
    assert_eq!(
        ::core::mem::align_of::<_MKL_Complex8>(),
        4usize,
        concat!("Alignment of ", stringify!(_MKL_Complex8))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<_MKL_Complex8>())).real as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_MKL_Complex8),
            "::",
            stringify!(real)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<_MKL_Complex8>())).imag as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_MKL_Complex8),
            "::",
            stringify!(imag)
        )
    );
}
pub type MKL_Complex8 = _MKL_Complex8;
pub type Complex8 = MKL_Complex8;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialOrd, PartialEq)]
pub struct _MKL_Complex16 {
    pub real: f64,
    pub imag: f64,
}
#[test]
fn bindgen_test_layout__MKL_Complex16() {
    assert_eq!(
        ::core::mem::size_of::<_MKL_Complex16>(),
        16usize,
        concat!("Size of: ", stringify!(_MKL_Complex16))
    );
    assert_eq!(
        ::core::mem::align_of::<_MKL_Complex16>(),
        8usize,
        concat!("Alignment of ", stringify!(_MKL_Complex16))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<_MKL_Complex16>())).real as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_MKL_Complex16),
            "::",
            stringify!(real)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<_MKL_Complex16>())).imag as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_MKL_Complex16),
            "::",
            stringify!(imag)
        )
    );
}
pub type MKL_Complex16 = _MKL_Complex16;
pub type Complex16 = MKL_Complex16;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct MKLVersion {
    pub MajorVersion: ::std::os::raw::c_int,
    pub MinorVersion: ::std::os::raw::c_int,
    pub UpdateVersion: ::std::os::raw::c_int,
    pub ProductStatus: *mut ::std::os::raw::c_char,
    pub Build: *mut ::std::os::raw::c_char,
    pub Processor: *mut ::std::os::raw::c_char,
    pub Platform: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_MKLVersion() {
    assert_eq!(
        ::core::mem::size_of::<MKLVersion>(),
        48usize,
        concat!("Size of: ", stringify!(MKLVersion))
    );
    assert_eq!(
        ::core::mem::align_of::<MKLVersion>(),
        8usize,
        concat!("Alignment of ", stringify!(MKLVersion))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MKLVersion>())).MajorVersion as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MKLVersion),
            "::",
            stringify!(MajorVersion)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MKLVersion>())).MinorVersion as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MKLVersion),
            "::",
            stringify!(MinorVersion)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MKLVersion>())).UpdateVersion as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MKLVersion),
            "::",
            stringify!(UpdateVersion)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MKLVersion>())).ProductStatus as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MKLVersion),
            "::",
            stringify!(ProductStatus)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MKLVersion>())).Build as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MKLVersion),
            "::",
            stringify!(Build)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MKLVersion>())).Processor as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(MKLVersion),
            "::",
            stringify!(Processor)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<MKLVersion>())).Platform as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(MKLVersion),
            "::",
            stringify!(Platform)
        )
    );
}
impl Default for MKLVersion {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const MKL_LAYOUT_MKL_ROW_MAJOR: MKL_LAYOUT = 101;
pub const MKL_LAYOUT_MKL_COL_MAJOR: MKL_LAYOUT = 102;
pub type MKL_LAYOUT = ::std::os::raw::c_uint;
pub const MKL_TRANSPOSE_MKL_NOTRANS: MKL_TRANSPOSE = 111;
pub const MKL_TRANSPOSE_MKL_TRANS: MKL_TRANSPOSE = 112;
pub const MKL_TRANSPOSE_MKL_CONJTRANS: MKL_TRANSPOSE = 113;
pub type MKL_TRANSPOSE = ::std::os::raw::c_uint;
pub const MKL_UPLO_MKL_UPPER: MKL_UPLO = 121;
pub const MKL_UPLO_MKL_LOWER: MKL_UPLO = 122;
pub type MKL_UPLO = ::std::os::raw::c_uint;
pub const MKL_DIAG_MKL_NONUNIT: MKL_DIAG = 131;
pub const MKL_DIAG_MKL_UNIT: MKL_DIAG = 132;
pub type MKL_DIAG = ::std::os::raw::c_uint;
pub const MKL_SIDE_MKL_LEFT: MKL_SIDE = 141;
pub const MKL_SIDE_MKL_RIGHT: MKL_SIDE = 142;
pub type MKL_SIDE = ::std::os::raw::c_uint;
pub const MKL_COMPACT_PACK_MKL_COMPACT_SSE: MKL_COMPACT_PACK = 181;
pub const MKL_COMPACT_PACK_MKL_COMPACT_AVX: MKL_COMPACT_PACK = 182;
pub const MKL_COMPACT_PACK_MKL_COMPACT_AVX512: MKL_COMPACT_PACK = 183;
pub type MKL_COMPACT_PACK = ::std::os::raw::c_uint;
pub type sgemm_jit_kernel_t = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::core::ffi::c_void,
        arg2: *mut f32,
        arg3: *mut f32,
        arg4: *mut f32,
    ),
>;
pub type dgemm_jit_kernel_t = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::core::ffi::c_void,
        arg2: *mut f64,
        arg3: *mut f64,
        arg4: *mut f64,
    ),
>;
pub type cgemm_jit_kernel_t = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::core::ffi::c_void,
        arg2: *mut MKL_Complex8,
        arg3: *mut MKL_Complex8,
        arg4: *mut MKL_Complex8,
    ),
>;
pub type zgemm_jit_kernel_t = ::core::option::Option<
    unsafe extern "C" fn(
        arg1: *mut ::core::ffi::c_void,
        arg2: *mut MKL_Complex16,
        arg3: *mut MKL_Complex16,
        arg4: *mut MKL_Complex16,
    ),
>;
pub const mkl_jit_status_t_MKL_JIT_SUCCESS: mkl_jit_status_t = 0;
pub const mkl_jit_status_t_MKL_NO_JIT: mkl_jit_status_t = 1;
pub const mkl_jit_status_t_MKL_JIT_ERROR: mkl_jit_status_t = 2;
pub type mkl_jit_status_t = ::std::os::raw::c_uint;

//---------------------------------------------------------

pub mod blas;
pub mod spblas;

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::*;

    #[test]
    fn complex8() {
        use std::mem::size_of;
        assert_eq!(size_of::<Complex32>(), size_of::<Complex8>());
        let v = Complex8 { real: 1.2_f32, imag: 2.4_f32 };
        let b = unsafe { std::ptr::read(&v as *const _ as *const Complex32) };
        
        assert_eq!(v.real, b.re);
        assert_eq!(v.imag, b.im);
    }

    #[test]
    fn complex16() {
        use std::mem::size_of;
        assert_eq!(size_of::<Complex64>(), size_of::<Complex16>());
        let v = Complex16 { real: 1.2, imag: 2.4 };
        let b = unsafe { std::ptr::read(&v as *const _ as *const Complex64) };
        
        assert_eq!(v.real, b.re);
        assert_eq!(v.imag, b.im);
    }
}
