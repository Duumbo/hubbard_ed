// Mod diagexact
pub mod matrix;

#[cfg(feature = "coefs_real")]
#[cfg(feature = "double_precision")]
#[macro_export]
macro_rules! diag_worker {
    () => (dspevd)
}

#[cfg(feature = "coefs_real")]
#[cfg(feature = "single_precision")]
#[macro_export]
macro_rules! diag_worker {
    () => (sspevd)
}

#[cfg(feature = "coefs_complex")]
#[cfg(feature = "double_precision")]
#[macro_export]
macro_rules! diag_worker {
    () => (zhpevd)
}

#[cfg(feature = "coefs_complex")]
#[cfg(feature = "single_precision")]
#[macro_export]
macro_rules! diag_worker {
    () => (chpevd)
}

#[macro_export]
macro_rules! any_pevd{
    (
        $jobz:expr,
        $uplo:expr,
        $n:expr,
        $ap:expr,
        $w:expr,
        $z:expr,
        $ldz:expr,
        $work:expr,
        $lwork:expr,
        $iwork:expr,
        $liwork:expr,
        $info:expr
        ) => (
            diag_worker!()(
        $jobz,
        $uplo,
        $n,
        $ap,
        $w,
        $z,
        $ldz,
        $work,
        $lwork,
        $iwork,
        $liwork,
        $info
        )
    )
}

