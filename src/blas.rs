//! This is based on `bindgen` produced FFI binding for MKL's cblas
//! (mkl_cblas.h) interface.
use super::*;

pub const CBLAS_LAYOUT_CblasRowMajor: CBLAS_LAYOUT = 101;
pub const CBLAS_LAYOUT_CblasColMajor: CBLAS_LAYOUT = 102;
pub type CBLAS_LAYOUT = ::std::os::raw::c_uint;
pub const CBLAS_TRANSPOSE_CblasNoTrans: CBLAS_TRANSPOSE = 111;
pub const CBLAS_TRANSPOSE_CblasTrans: CBLAS_TRANSPOSE = 112;
pub const CBLAS_TRANSPOSE_CblasConjTrans: CBLAS_TRANSPOSE = 113;
pub type CBLAS_TRANSPOSE = ::std::os::raw::c_uint;
pub const CBLAS_UPLO_CblasUpper: CBLAS_UPLO = 121;
pub const CBLAS_UPLO_CblasLower: CBLAS_UPLO = 122;
pub type CBLAS_UPLO = ::std::os::raw::c_uint;
pub const CBLAS_DIAG_CblasNonUnit: CBLAS_DIAG = 131;
pub const CBLAS_DIAG_CblasUnit: CBLAS_DIAG = 132;
pub type CBLAS_DIAG = ::std::os::raw::c_uint;
pub const CBLAS_SIDE_CblasLeft: CBLAS_SIDE = 141;
pub const CBLAS_SIDE_CblasRight: CBLAS_SIDE = 142;
pub type CBLAS_SIDE = ::std::os::raw::c_uint;
pub const CBLAS_STORAGE_CblasPacked: CBLAS_STORAGE = 151;
pub type CBLAS_STORAGE = ::std::os::raw::c_uint;
pub const CBLAS_IDENTIFIER_CblasAMatrix: CBLAS_IDENTIFIER = 161;
pub const CBLAS_IDENTIFIER_CblasBMatrix: CBLAS_IDENTIFIER = 162;
pub type CBLAS_IDENTIFIER = ::std::os::raw::c_uint;
pub const CBLAS_OFFSET_CblasRowOffset: CBLAS_OFFSET = 171;
pub const CBLAS_OFFSET_CblasColOffset: CBLAS_OFFSET = 172;
pub const CBLAS_OFFSET_CblasFixOffset: CBLAS_OFFSET = 173;
pub type CBLAS_OFFSET = ::std::os::raw::c_uint;
pub use self::CBLAS_LAYOUT as CBLAS_ORDER;
extern "C" {
    pub fn cblas_dcabs1(z: *const ::core::ffi::c_void) -> f64;
}
extern "C" {
    pub fn cblas_scabs1(c: *const ::core::ffi::c_void) -> f32;
}
extern "C" {
    pub fn cblas_sdot(
        N: ::std::os::raw::c_int,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        Y: *const f32,
        incY: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn cblas_sdoti(
        N: ::std::os::raw::c_int,
        X: *const f32,
        indx: *const ::std::os::raw::c_int,
        Y: *const f32,
    ) -> f32;
}
extern "C" {
    pub fn cblas_ddot(
        N: ::std::os::raw::c_int,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        Y: *const f64,
        incY: ::std::os::raw::c_int,
    ) -> f64;
}
extern "C" {
    pub fn cblas_ddoti(
        N: ::std::os::raw::c_int,
        X: *const f64,
        indx: *const ::std::os::raw::c_int,
        Y: *const f64,
    ) -> f64;
}
extern "C" {
    pub fn cblas_dsdot(
        N: ::std::os::raw::c_int,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        Y: *const f32,
        incY: ::std::os::raw::c_int,
    ) -> f64;
}
extern "C" {
    pub fn cblas_sdsdot(
        N: ::std::os::raw::c_int,
        sb: f32,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        Y: *const f32,
        incY: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn cblas_cdotu_sub(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        dotu: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_cdotui_sub(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        indx: *const ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        dotui: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_cdotc_sub(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        dotc: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_cdotci_sub(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        indx: *const ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        dotui: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_zdotu_sub(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        dotu: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_zdotui_sub(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        indx: *const ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        dotui: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_zdotc_sub(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        dotc: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_zdotci_sub(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        indx: *const ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        dotui: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_snrm2(N: ::std::os::raw::c_int, X: *const f32, incX: ::std::os::raw::c_int)
        -> f32;
}
extern "C" {
    pub fn cblas_sasum(N: ::std::os::raw::c_int, X: *const f32, incX: ::std::os::raw::c_int)
        -> f32;
}
extern "C" {
    pub fn cblas_dnrm2(N: ::std::os::raw::c_int, X: *const f64, incX: ::std::os::raw::c_int)
        -> f64;
}
extern "C" {
    pub fn cblas_dasum(N: ::std::os::raw::c_int, X: *const f64, incX: ::std::os::raw::c_int)
        -> f64;
}
extern "C" {
    pub fn cblas_scnrm2(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn cblas_scasum(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn cblas_dznrm2(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    ) -> f64;
}
extern "C" {
    pub fn cblas_dzasum(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    ) -> f64;
}
extern "C" {
    pub fn cblas_isamax(
        N: ::std::os::raw::c_int,
        X: *const f32,
        incX: ::std::os::raw::c_int,
    ) -> size_t;
}
extern "C" {
    pub fn cblas_idamax(
        N: ::std::os::raw::c_int,
        X: *const f64,
        incX: ::std::os::raw::c_int,
    ) -> size_t;
}
extern "C" {
    pub fn cblas_icamax(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    ) -> size_t;
}
extern "C" {
    pub fn cblas_izamax(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    ) -> size_t;
}
extern "C" {
    pub fn cblas_isamin(
        N: ::std::os::raw::c_int,
        X: *const f32,
        incX: ::std::os::raw::c_int,
    ) -> size_t;
}
extern "C" {
    pub fn cblas_idamin(
        N: ::std::os::raw::c_int,
        X: *const f64,
        incX: ::std::os::raw::c_int,
    ) -> size_t;
}
extern "C" {
    pub fn cblas_icamin(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    ) -> size_t;
}
extern "C" {
    pub fn cblas_izamin(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    ) -> size_t;
}
extern "C" {
    pub fn cblas_sswap(
        N: ::std::os::raw::c_int,
        X: *mut f32,
        incX: ::std::os::raw::c_int,
        Y: *mut f32,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_scopy(
        N: ::std::os::raw::c_int,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        Y: *mut f32,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_saxpy(
        N: ::std::os::raw::c_int,
        alpha: f32,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        Y: *mut f32,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_saxpby(
        N: ::std::os::raw::c_int,
        alpha: f32,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        beta: f32,
        Y: *mut f32,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_saxpyi(
        N: ::std::os::raw::c_int,
        alpha: f32,
        X: *const f32,
        indx: *const ::std::os::raw::c_int,
        Y: *mut f32,
    );
}
extern "C" {
    pub fn cblas_sgthr(
        N: ::std::os::raw::c_int,
        Y: *const f32,
        X: *mut f32,
        indx: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_sgthrz(
        N: ::std::os::raw::c_int,
        Y: *mut f32,
        X: *mut f32,
        indx: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ssctr(
        N: ::std::os::raw::c_int,
        X: *const f32,
        indx: *const ::std::os::raw::c_int,
        Y: *mut f32,
    );
}
extern "C" {
    pub fn cblas_srotg(a: *mut f32, b: *mut f32, c: *mut f32, s: *mut f32);
}
extern "C" {
    pub fn cblas_dswap(
        N: ::std::os::raw::c_int,
        X: *mut f64,
        incX: ::std::os::raw::c_int,
        Y: *mut f64,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dcopy(
        N: ::std::os::raw::c_int,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        Y: *mut f64,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_daxpy(
        N: ::std::os::raw::c_int,
        alpha: f64,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        Y: *mut f64,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_daxpby(
        N: ::std::os::raw::c_int,
        alpha: f64,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        beta: f64,
        Y: *mut f64,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_daxpyi(
        N: ::std::os::raw::c_int,
        alpha: f64,
        X: *const f64,
        indx: *const ::std::os::raw::c_int,
        Y: *mut f64,
    );
}
extern "C" {
    pub fn cblas_dgthr(
        N: ::std::os::raw::c_int,
        Y: *const f64,
        X: *mut f64,
        indx: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dgthrz(
        N: ::std::os::raw::c_int,
        Y: *mut f64,
        X: *mut f64,
        indx: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dsctr(
        N: ::std::os::raw::c_int,
        X: *const f64,
        indx: *const ::std::os::raw::c_int,
        Y: *mut f64,
    );
}
extern "C" {
    pub fn cblas_drotg(a: *mut f64, b: *mut f64, c: *mut f64, s: *mut f64);
}
extern "C" {
    pub fn cblas_cswap(
        N: ::std::os::raw::c_int,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ccopy(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_caxpy(
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_caxpby(
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_caxpyi(
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        indx: *const ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_cgthr(
        N: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        X: *mut ::core::ffi::c_void,
        indx: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cgthrz(
        N: ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
        X: *mut ::core::ffi::c_void,
        indx: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_csctr(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        indx: *const ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_crotg(
        a: *mut ::core::ffi::c_void,
        b: *const ::core::ffi::c_void,
        c: *mut f32,
        s: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_zswap(
        N: ::std::os::raw::c_int,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zcopy(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zaxpy(
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zaxpby(
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zaxpyi(
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        indx: *const ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_zgthr(
        N: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        X: *mut ::core::ffi::c_void,
        indx: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zgthrz(
        N: ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
        X: *mut ::core::ffi::c_void,
        indx: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zsctr(
        N: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        indx: *const ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_zrotg(
        a: *mut ::core::ffi::c_void,
        b: *const ::core::ffi::c_void,
        c: *mut f64,
        s: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_srotmg(d1: *mut f32, d2: *mut f32, b1: *mut f32, b2: f32, P: *mut f32);
}
extern "C" {
    pub fn cblas_srot(
        N: ::std::os::raw::c_int,
        X: *mut f32,
        incX: ::std::os::raw::c_int,
        Y: *mut f32,
        incY: ::std::os::raw::c_int,
        c: f32,
        s: f32,
    );
}
extern "C" {
    pub fn cblas_sroti(
        N: ::std::os::raw::c_int,
        X: *mut f32,
        indx: *const ::std::os::raw::c_int,
        Y: *mut f32,
        c: f32,
        s: f32,
    );
}
extern "C" {
    pub fn cblas_srotm(
        N: ::std::os::raw::c_int,
        X: *mut f32,
        incX: ::std::os::raw::c_int,
        Y: *mut f32,
        incY: ::std::os::raw::c_int,
        P: *const f32,
    );
}
extern "C" {
    pub fn cblas_drotmg(d1: *mut f64, d2: *mut f64, b1: *mut f64, b2: f64, P: *mut f64);
}
extern "C" {
    pub fn cblas_drot(
        N: ::std::os::raw::c_int,
        X: *mut f64,
        incX: ::std::os::raw::c_int,
        Y: *mut f64,
        incY: ::std::os::raw::c_int,
        c: f64,
        s: f64,
    );
}
extern "C" {
    pub fn cblas_drotm(
        N: ::std::os::raw::c_int,
        X: *mut f64,
        incX: ::std::os::raw::c_int,
        Y: *mut f64,
        incY: ::std::os::raw::c_int,
        P: *const f64,
    );
}
extern "C" {
    pub fn cblas_droti(
        N: ::std::os::raw::c_int,
        X: *mut f64,
        indx: *const ::std::os::raw::c_int,
        Y: *mut f64,
        c: f64,
        s: f64,
    );
}
extern "C" {
    pub fn cblas_csrot(
        N: ::std::os::raw::c_int,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        c: f32,
        s: f32,
    );
}
extern "C" {
    pub fn cblas_zdrot(
        N: ::std::os::raw::c_int,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        c: f64,
        s: f64,
    );
}
extern "C" {
    pub fn cblas_sscal(
        N: ::std::os::raw::c_int,
        alpha: f32,
        X: *mut f32,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dscal(
        N: ::std::os::raw::c_int,
        alpha: f64,
        X: *mut f64,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cscal(
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zscal(
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_csscal(
        N: ::std::os::raw::c_int,
        alpha: f32,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zdscal(
        N: ::std::os::raw::c_int,
        alpha: f64,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_sgemv(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        beta: f32,
        Y: *mut f32,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_sgbmv(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        KL: ::std::os::raw::c_int,
        KU: ::std::os::raw::c_int,
        alpha: f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        beta: f32,
        Y: *mut f32,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_strmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        X: *mut f32,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_stbmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        X: *mut f32,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_stpmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        Ap: *const f32,
        X: *mut f32,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_strsv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        X: *mut f32,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_stbsv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        X: *mut f32,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_stpsv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        Ap: *const f32,
        X: *mut f32,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dgemv(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        beta: f64,
        Y: *mut f64,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dgbmv(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        KL: ::std::os::raw::c_int,
        KU: ::std::os::raw::c_int,
        alpha: f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        beta: f64,
        Y: *mut f64,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dtrmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        X: *mut f64,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dtbmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        X: *mut f64,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dtpmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        Ap: *const f64,
        X: *mut f64,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dtrsv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        X: *mut f64,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dtbsv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        X: *mut f64,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dtpsv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        Ap: *const f64,
        X: *mut f64,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cgemv(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cgbmv(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        KL: ::std::os::raw::c_int,
        KU: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ctrmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ctbmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ctpmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        Ap: *const ::core::ffi::c_void,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ctrsv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ctbsv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ctpsv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        Ap: *const ::core::ffi::c_void,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zgemv(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zgbmv(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        KL: ::std::os::raw::c_int,
        KU: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ztrmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ztbmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ztpmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        Ap: *const ::core::ffi::c_void,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ztrsv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ztbsv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ztpsv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        N: ::std::os::raw::c_int,
        Ap: *const ::core::ffi::c_void,
        X: *mut ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ssymv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        beta: f32,
        Y: *mut f32,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ssbmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        beta: f32,
        Y: *mut f32,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_sspmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f32,
        Ap: *const f32,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        beta: f32,
        Y: *mut f32,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_sger(
        Layout: CBLAS_LAYOUT,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: f32,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        Y: *const f32,
        incY: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ssyr(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f32,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_sspr(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f32,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        Ap: *mut f32,
    );
}
extern "C" {
    pub fn cblas_ssyr2(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f32,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        Y: *const f32,
        incY: ::std::os::raw::c_int,
        A: *mut f32,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_sspr2(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f32,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        Y: *const f32,
        incY: ::std::os::raw::c_int,
        A: *mut f32,
    );
}
extern "C" {
    pub fn cblas_dsymv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        beta: f64,
        Y: *mut f64,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dsbmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        beta: f64,
        Y: *mut f64,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dspmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f64,
        Ap: *const f64,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        beta: f64,
        Y: *mut f64,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dger(
        Layout: CBLAS_LAYOUT,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: f64,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        Y: *const f64,
        incY: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dsyr(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f64,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dspr(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f64,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        Ap: *mut f64,
    );
}
extern "C" {
    pub fn cblas_dsyr2(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f64,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        Y: *const f64,
        incY: ::std::os::raw::c_int,
        A: *mut f64,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dspr2(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f64,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        Y: *const f64,
        incY: ::std::os::raw::c_int,
        A: *mut f64,
    );
}
extern "C" {
    pub fn cblas_chemv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_chbmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_chpmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        Ap: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cgeru(
        Layout: CBLAS_LAYOUT,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        A: *mut ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cgerc(
        Layout: CBLAS_LAYOUT,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        A: *mut ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cher(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f32,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        A: *mut ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_chpr(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f32,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        A: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_cher2(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        A: *mut ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_chpr2(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        Ap: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_zhemv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zhbmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zhpmv(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        Ap: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zgeru(
        Layout: CBLAS_LAYOUT,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        A: *mut ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zgerc(
        Layout: CBLAS_LAYOUT,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        A: *mut ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zher(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f64,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        A: *mut ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zhpr(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: f64,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        A: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_zher2(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        A: *mut ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zhpr2(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        Y: *const ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        Ap: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_sgemm(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        beta: f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_sgemm_batch(
        Layout: CBLAS_LAYOUT,
        TransA_Array: *const CBLAS_TRANSPOSE,
        TransB_Array: *const CBLAS_TRANSPOSE,
        M_Array: *const ::std::os::raw::c_int,
        N_Array: *const ::std::os::raw::c_int,
        K_Array: *const ::std::os::raw::c_int,
        alpha_Array: *const f32,
        A_Array: *mut *const f32,
        lda_Array: *const ::std::os::raw::c_int,
        B_Array: *mut *const f32,
        ldb_Array: *const ::std::os::raw::c_int,
        beta_Array: *const f32,
        C_Array: *mut *mut f32,
        ldc_Array: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_sgemm_batch_strided(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        stridea: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        strideb: ::std::os::raw::c_int,
        beta: f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
        stridec: ::std::os::raw::c_int,
        batch_size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_sgemmt(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        beta: f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ssymm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        beta: f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ssyrk(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        beta: f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ssyr2k(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        beta: f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_strmm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *mut f32,
        ldb: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_strsm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: f32,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *mut f32,
        ldb: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_strsm_batch(
        Layout: CBLAS_LAYOUT,
        Side_Array: *const CBLAS_SIDE,
        Uplo_Array: *const CBLAS_UPLO,
        TransA_Array: *const CBLAS_TRANSPOSE,
        Diag_Array: *const CBLAS_DIAG,
        M_Array: *const ::std::os::raw::c_int,
        N_Array: *const ::std::os::raw::c_int,
        alpha_Array: *const f32,
        A_Array: *mut *const f32,
        lda_Array: *const ::std::os::raw::c_int,
        B_Array: *mut *mut f32,
        ldb_Array: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dgemm(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        beta: f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dgemm_batch(
        Layout: CBLAS_LAYOUT,
        TransA_Array: *const CBLAS_TRANSPOSE,
        TransB_Array: *const CBLAS_TRANSPOSE,
        M_Array: *const ::std::os::raw::c_int,
        N_Array: *const ::std::os::raw::c_int,
        K_Array: *const ::std::os::raw::c_int,
        alpha_Array: *const f64,
        A_Array: *mut *const f64,
        lda_Array: *const ::std::os::raw::c_int,
        B_Array: *mut *const f64,
        ldb_Array: *const ::std::os::raw::c_int,
        beta_Array: *const f64,
        C_Array: *mut *mut f64,
        ldc_Array: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dgemm_batch_strided(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        stridea: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        strideb: ::std::os::raw::c_int,
        beta: f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
        stridec: ::std::os::raw::c_int,
        batch_size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dgemmt(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        beta: f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dsymm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        beta: f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dsyrk(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        beta: f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dsyr2k(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        beta: f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dtrmm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *mut f64,
        ldb: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dtrsm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: f64,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *mut f64,
        ldb: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dtrsm_batch(
        Layout: CBLAS_LAYOUT,
        Side_Array: *const CBLAS_SIDE,
        Uplo_Array: *const CBLAS_UPLO,
        Transa_Array: *const CBLAS_TRANSPOSE,
        Diag_Array: *const CBLAS_DIAG,
        M_Array: *const ::std::os::raw::c_int,
        N_Array: *const ::std::os::raw::c_int,
        alpha_Array: *const f64,
        A_Array: *mut *const f64,
        lda_Array: *const ::std::os::raw::c_int,
        B_Array: *mut *mut f64,
        ldb_Array: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cgemm(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cgemm3m(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cgemm_batch(
        Layout: CBLAS_LAYOUT,
        TransA_Array: *const CBLAS_TRANSPOSE,
        TransB_Array: *const CBLAS_TRANSPOSE,
        M_Array: *const ::std::os::raw::c_int,
        N_Array: *const ::std::os::raw::c_int,
        K_Array: *const ::std::os::raw::c_int,
        alpha_Array: *const ::core::ffi::c_void,
        A_Array: *mut *const ::core::ffi::c_void,
        lda_Array: *const ::std::os::raw::c_int,
        B_Array: *mut *const ::core::ffi::c_void,
        ldb_Array: *const ::std::os::raw::c_int,
        beta_Array: *const ::core::ffi::c_void,
        C_Array: *mut *mut ::core::ffi::c_void,
        ldc_Array: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cgemm_batch_strided(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        stridea: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        strideb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
        stridec: ::std::os::raw::c_int,
        batch_size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cgemm3m_batch(
        Layout: CBLAS_LAYOUT,
        TransA_Array: *const CBLAS_TRANSPOSE,
        TransB_Array: *const CBLAS_TRANSPOSE,
        M_Array: *const ::std::os::raw::c_int,
        N_Array: *const ::std::os::raw::c_int,
        K_Array: *const ::std::os::raw::c_int,
        alpha_Array: *const ::core::ffi::c_void,
        A_Array: *mut *const ::core::ffi::c_void,
        lda_Array: *const ::std::os::raw::c_int,
        B_Array: *mut *const ::core::ffi::c_void,
        ldb_Array: *const ::std::os::raw::c_int,
        beta_Array: *const ::core::ffi::c_void,
        C_Array: *mut *mut ::core::ffi::c_void,
        ldc_Array: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cgemmt(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_csymm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_csyrk(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_csyr2k(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ctrmm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *mut ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ctrsm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *mut ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ctrsm_batch(
        Layout: CBLAS_LAYOUT,
        Side_Array: *const CBLAS_SIDE,
        Uplo_Array: *const CBLAS_UPLO,
        Transa_Array: *const CBLAS_TRANSPOSE,
        Diag_Array: *const CBLAS_DIAG,
        M_Array: *const ::std::os::raw::c_int,
        N_Array: *const ::std::os::raw::c_int,
        alpha_Array: *const ::core::ffi::c_void,
        A_Array: *mut *const ::core::ffi::c_void,
        lda_Array: *const ::std::os::raw::c_int,
        B_Array: *mut *mut ::core::ffi::c_void,
        ldb_Array: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zgemm(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zgemm3m(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zgemm_batch(
        Layout: CBLAS_LAYOUT,
        TransA_Array: *const CBLAS_TRANSPOSE,
        TransB_Array: *const CBLAS_TRANSPOSE,
        M_Array: *const ::std::os::raw::c_int,
        N_Array: *const ::std::os::raw::c_int,
        K_Array: *const ::std::os::raw::c_int,
        alpha_Array: *const ::core::ffi::c_void,
        A_Array: *mut *const ::core::ffi::c_void,
        lda_Array: *const ::std::os::raw::c_int,
        B_Array: *mut *const ::core::ffi::c_void,
        ldb_Array: *const ::std::os::raw::c_int,
        beta_Array: *const ::core::ffi::c_void,
        C_Array: *mut *mut ::core::ffi::c_void,
        ldc_Array: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zgemm_batch_strided(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        stridea: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        strideb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
        stridec: ::std::os::raw::c_int,
        batch_size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zgemm3m_batch(
        Layout: CBLAS_LAYOUT,
        TransA_Array: *const CBLAS_TRANSPOSE,
        TransB_Array: *const CBLAS_TRANSPOSE,
        M_Array: *const ::std::os::raw::c_int,
        N_Array: *const ::std::os::raw::c_int,
        K_Array: *const ::std::os::raw::c_int,
        alpha_Array: *const ::core::ffi::c_void,
        A_Array: *mut *const ::core::ffi::c_void,
        lda_Array: *const ::std::os::raw::c_int,
        B_Array: *mut *const ::core::ffi::c_void,
        ldb_Array: *const ::std::os::raw::c_int,
        beta_Array: *const ::core::ffi::c_void,
        C_Array: *mut *mut ::core::ffi::c_void,
        ldc_Array: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zgemmt(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zsymm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zsyrk(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zsyr2k(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ztrmm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *mut ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ztrsm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        TransA: CBLAS_TRANSPOSE,
        Diag: CBLAS_DIAG,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *mut ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_ztrsm_batch(
        Layout: CBLAS_LAYOUT,
        Side_Array: *const CBLAS_SIDE,
        Uplo_Array: *const CBLAS_UPLO,
        Transa_Array: *const CBLAS_TRANSPOSE,
        Diag_Array: *const CBLAS_DIAG,
        M_Array: *const ::std::os::raw::c_int,
        N_Array: *const ::std::os::raw::c_int,
        alpha_Array: *const ::core::ffi::c_void,
        A_Array: *mut *const ::core::ffi::c_void,
        lda_Array: *const ::std::os::raw::c_int,
        B_Array: *mut *mut ::core::ffi::c_void,
        ldb_Array: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_chemm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cherk(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        beta: f32,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_cher2k(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: f32,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zhemm(
        Layout: CBLAS_LAYOUT,
        Side: CBLAS_SIDE,
        Uplo: CBLAS_UPLO,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zherk(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f64,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        beta: f64,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zher2k(
        Layout: CBLAS_LAYOUT,
        Uplo: CBLAS_UPLO,
        Trans: CBLAS_TRANSPOSE,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        beta: f64,
        C: *mut ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_sgemm_alloc(
        identifier: CBLAS_IDENTIFIER,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
    ) -> *mut f32;
}
extern "C" {
    pub fn cblas_sgemm_pack_get_size(
        identifier: CBLAS_IDENTIFIER,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
    ) -> size_t;
}
extern "C" {
    pub fn cblas_sgemm_pack(
        Layout: CBLAS_LAYOUT,
        identifier: CBLAS_IDENTIFIER,
        Trans: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        src: *const f32,
        ld: ::std::os::raw::c_int,
        dest: *mut f32,
    );
}
extern "C" {
    pub fn cblas_sgemm_compute(
        Layout: CBLAS_LAYOUT,
        TransA: ::std::os::raw::c_int,
        TransB: ::std::os::raw::c_int,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        A: *const f32,
        lda: ::std::os::raw::c_int,
        B: *const f32,
        ldb: ::std::os::raw::c_int,
        beta: f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_sgemm_free(dest: *mut f32);
}
extern "C" {
    pub fn cblas_dgemm_alloc(
        identifier: CBLAS_IDENTIFIER,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
    ) -> *mut f64;
}
extern "C" {
    pub fn cblas_dgemm_pack_get_size(
        identifier: CBLAS_IDENTIFIER,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
    ) -> size_t;
}
extern "C" {
    pub fn cblas_dgemm_pack(
        Layout: CBLAS_LAYOUT,
        identifier: CBLAS_IDENTIFIER,
        Trans: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f64,
        src: *const f64,
        ld: ::std::os::raw::c_int,
        dest: *mut f64,
    );
}
extern "C" {
    pub fn cblas_dgemm_compute(
        Layout: CBLAS_LAYOUT,
        TransA: ::std::os::raw::c_int,
        TransB: ::std::os::raw::c_int,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        A: *const f64,
        lda: ::std::os::raw::c_int,
        B: *const f64,
        ldb: ::std::os::raw::c_int,
        beta: f64,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_dgemm_free(dest: *mut f64);
}
extern "C" {
    pub fn cblas_gemm_s16s16s32(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        OffsetC: CBLAS_OFFSET,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        A: *const ::std::os::raw::c_short,
        lda: ::std::os::raw::c_int,
        ao: ::std::os::raw::c_short,
        B: *const ::std::os::raw::c_short,
        ldb: ::std::os::raw::c_int,
        bo: ::std::os::raw::c_short,
        beta: f32,
        C: *mut ::std::os::raw::c_int,
        ldc: ::std::os::raw::c_int,
        cb: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_gemm_s8u8s32(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        OffsetC: CBLAS_OFFSET,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        ao: ::std::os::raw::c_char,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        bo: ::std::os::raw::c_char,
        beta: f32,
        C: *mut ::std::os::raw::c_int,
        ldc: ::std::os::raw::c_int,
        cb: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_gemm_bf16bf16f32(
        Layout: CBLAS_LAYOUT,
        TransA: CBLAS_TRANSPOSE,
        TransB: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        A: *const ::std::os::raw::c_ushort,
        lda: ::std::os::raw::c_int,
        B: *const ::std::os::raw::c_ushort,
        ldb: ::std::os::raw::c_int,
        beta: f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_gemm_s8u8s32_pack_get_size(
        identifier: CBLAS_IDENTIFIER,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
    ) -> size_t;
}
extern "C" {
    pub fn cblas_gemm_s16s16s32_pack_get_size(
        identifier: CBLAS_IDENTIFIER,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
    ) -> size_t;
}
extern "C" {
    pub fn cblas_gemm_bf16bf16f32_pack_get_size(
        identifier: CBLAS_IDENTIFIER,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
    ) -> size_t;
}
extern "C" {
    pub fn cblas_gemm_s8u8s32_pack(
        Layout: CBLAS_LAYOUT,
        identifier: CBLAS_IDENTIFIER,
        Trans: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        src: *const ::core::ffi::c_void,
        ld: ::std::os::raw::c_int,
        dest: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn cblas_gemm_s16s16s32_pack(
        Layout: CBLAS_LAYOUT,
        identifier: CBLAS_IDENTIFIER,
        Trans: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        src: *const ::std::os::raw::c_short,
        ld: ::std::os::raw::c_int,
        dest: *mut ::std::os::raw::c_short,
    );
}
extern "C" {
    pub fn cblas_gemm_bf16bf16f32_pack(
        Layout: CBLAS_LAYOUT,
        identifier: CBLAS_IDENTIFIER,
        Trans: CBLAS_TRANSPOSE,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        src: *const ::std::os::raw::c_ushort,
        ld: ::std::os::raw::c_int,
        dest: *mut ::std::os::raw::c_ushort,
    );
}
extern "C" {
    pub fn cblas_gemm_s8u8s32_compute(
        Layout: CBLAS_LAYOUT,
        TransA: ::std::os::raw::c_int,
        TransB: ::std::os::raw::c_int,
        offsetc: CBLAS_OFFSET,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        A: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        ao: ::std::os::raw::c_char,
        B: *const ::core::ffi::c_void,
        ldb: ::std::os::raw::c_int,
        bo: ::std::os::raw::c_char,
        beta: f32,
        C: *mut ::std::os::raw::c_int,
        ldc: ::std::os::raw::c_int,
        co: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_gemm_s16s16s32_compute(
        Layout: CBLAS_LAYOUT,
        TransA: ::std::os::raw::c_int,
        TransB: ::std::os::raw::c_int,
        offsetc: CBLAS_OFFSET,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        A: *const ::std::os::raw::c_short,
        lda: ::std::os::raw::c_int,
        ao: ::std::os::raw::c_short,
        B: *const ::std::os::raw::c_short,
        ldb: ::std::os::raw::c_int,
        bo: ::std::os::raw::c_short,
        beta: f32,
        C: *mut ::std::os::raw::c_int,
        ldc: ::std::os::raw::c_int,
        co: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_gemm_bf16bf16f32_compute(
        Layout: CBLAS_LAYOUT,
        TransA: ::std::os::raw::c_int,
        TransB: ::std::os::raw::c_int,
        M: ::std::os::raw::c_int,
        N: ::std::os::raw::c_int,
        K: ::std::os::raw::c_int,
        alpha: f32,
        A: *const ::std::os::raw::c_ushort,
        lda: ::std::os::raw::c_int,
        B: *const ::std::os::raw::c_ushort,
        ldb: ::std::os::raw::c_int,
        beta: f32,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_cblas_jit_create_dgemm(
        jitter: *mut *mut ::core::ffi::c_void,
        layout: MKL_LAYOUT,
        transa: MKL_TRANSPOSE,
        transb: MKL_TRANSPOSE,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: f64,
        lda: ::std::os::raw::c_int,
        ldb: ::std::os::raw::c_int,
        beta: f64,
        ldc: ::std::os::raw::c_int,
    ) -> mkl_jit_status_t;
}
extern "C" {
    pub fn mkl_cblas_jit_create_sgemm(
        jitter: *mut *mut ::core::ffi::c_void,
        layout: MKL_LAYOUT,
        transa: MKL_TRANSPOSE,
        transb: MKL_TRANSPOSE,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: f32,
        lda: ::std::os::raw::c_int,
        ldb: ::std::os::raw::c_int,
        beta: f32,
        ldc: ::std::os::raw::c_int,
    ) -> mkl_jit_status_t;
}
extern "C" {
    pub fn mkl_cblas_jit_create_cgemm(
        jitter: *mut *mut ::core::ffi::c_void,
        layout: MKL_LAYOUT,
        transa: MKL_TRANSPOSE,
        transb: MKL_TRANSPOSE,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    ) -> mkl_jit_status_t;
}
extern "C" {
    pub fn mkl_cblas_jit_create_zgemm(
        jitter: *mut *mut ::core::ffi::c_void,
        layout: MKL_LAYOUT,
        transa: MKL_TRANSPOSE,
        transb: MKL_TRANSPOSE,
        m: ::std::os::raw::c_int,
        n: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        lda: ::std::os::raw::c_int,
        ldb: ::std::os::raw::c_int,
        beta: *const ::core::ffi::c_void,
        ldc: ::std::os::raw::c_int,
    ) -> mkl_jit_status_t;
}
extern "C" {
    pub fn mkl_jit_get_dgemm_ptr(jitter: *const ::core::ffi::c_void) -> dgemm_jit_kernel_t;
}
extern "C" {
    pub fn mkl_jit_get_sgemm_ptr(jitter: *const ::core::ffi::c_void) -> sgemm_jit_kernel_t;
}
extern "C" {
    pub fn mkl_jit_get_cgemm_ptr(jitter: *const ::core::ffi::c_void) -> cgemm_jit_kernel_t;
}
extern "C" {
    pub fn mkl_jit_get_zgemm_ptr(jitter: *const ::core::ffi::c_void) -> zgemm_jit_kernel_t;
}
extern "C" {
    pub fn mkl_jit_destroy(jitter: *mut ::core::ffi::c_void) -> mkl_jit_status_t;
}
extern "C" {
    pub fn cblas_saxpy_batch(
        n: *const ::std::os::raw::c_int,
        alpha: *const f32,
        x: *mut *const f32,
        incx: *const ::std::os::raw::c_int,
        y: *mut *mut f32,
        incy: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_daxpy_batch(
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        x: *mut *const f64,
        incx: *const ::std::os::raw::c_int,
        y: *mut *mut f64,
        incy: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_caxpy_batch(
        n: *const ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        x: *mut *const ::core::ffi::c_void,
        incx: *const ::std::os::raw::c_int,
        y: *mut *mut ::core::ffi::c_void,
        incy: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zaxpy_batch(
        n: *const ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        x: *mut *const ::core::ffi::c_void,
        incx: *const ::std::os::raw::c_int,
        y: *mut *mut ::core::ffi::c_void,
        incy: *const ::std::os::raw::c_int,
        group_count: ::std::os::raw::c_int,
        group_size: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_saxpy_batch_strided(
        N: ::std::os::raw::c_int,
        alpha: f32,
        X: *const f32,
        incX: ::std::os::raw::c_int,
        stridex: ::std::os::raw::c_int,
        Y: *mut f32,
        incY: ::std::os::raw::c_int,
        stridey: ::std::os::raw::c_int,
        batch_size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_daxpy_batch_strided(
        N: ::std::os::raw::c_int,
        alpha: f64,
        X: *const f64,
        incX: ::std::os::raw::c_int,
        stridex: ::std::os::raw::c_int,
        Y: *mut f64,
        incY: ::std::os::raw::c_int,
        stridey: ::std::os::raw::c_int,
        batch_size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_caxpy_batch_strided(
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        stridex: ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        stridey: ::std::os::raw::c_int,
        batch_size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn cblas_zaxpy_batch_strided(
        N: ::std::os::raw::c_int,
        alpha: *const ::core::ffi::c_void,
        X: *const ::core::ffi::c_void,
        incX: ::std::os::raw::c_int,
        stridex: ::std::os::raw::c_int,
        Y: *mut ::core::ffi::c_void,
        incY: ::std::os::raw::c_int,
        stridey: ::std::os::raw::c_int,
        batch_size: ::std::os::raw::c_int,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ddot() {
        let a = vec![1_f64; 100];
        let b = vec![2_f64; 100];
        let v = unsafe { cblas_ddot(a.len() as i32, a.as_ptr(), 1, b.as_ptr(), 1) };
        approx::assert_ulps_eq!(200., v, max_ulps = 4, epsilon = std::f64::EPSILON);
    }

    #[test]
    fn zdotc() {
        use core::ffi::c_void;

        let mut a: Vec<Complex16> = Vec::with_capacity(60);
        let mut b: Vec<Complex16> = Vec::with_capacity(60);
        for _ in 0..60 {
            a.push(Complex16 {
                real: 1_f64,
                imag: 0_f64,
            });
            b.push(Complex16 {
                real: 0_f64,
                imag: 1_f64,
            });
        }
        assert_eq!(a.len(), 60);
        let mut ret = Complex16 { real: 0., imag: 0. };
        unsafe {
            cblas_zdotc_sub(
                a.len() as i32,
                b.as_ptr() as *const c_void,
                1,
                a.as_ptr() as *const c_void,
                1,
                &mut ret as *mut Complex16 as *mut c_void,
            )
        };
        approx::assert_ulps_eq!(-60., ret.imag, max_ulps = 4, epsilon = std::f64::EPSILON);
    }
}
