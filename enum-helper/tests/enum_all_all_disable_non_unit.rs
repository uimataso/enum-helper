use assert2::check;
use enum_helper::EnumAll;

// Per PLAN, `all(disable)` suppresses the non-unit-enum error so that `COUNT`
// (which only counts variants and never constructs them) can still be derived
// for enums that contain non-unit variants.
#[derive(EnumAll, Clone)]
#[enum_all(all(disable))]
#[allow(dead_code)]
enum Mixed {
    Unit,
    Tuple(usize),
    Struct { x: usize },
}

#[test]
fn all_disable_allows_non_unit_count() {
    // COUNT includes every non-`skip` variant, unit or not.
    check!(Mixed::COUNT == 3);
}
