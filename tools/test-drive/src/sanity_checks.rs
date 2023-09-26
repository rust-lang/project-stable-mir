//! Module that contains sanity checks that Stable MIR APIs don't crash and that
//! their result is coherent.
//!
//! These checks should only depend on StableMIR APIs. See other modules for tests that compare
//! the result between StableMIR and internal APIs.
use crate::TestResult;
use rustc_smir::stable_mir;
use std::fmt::Debug;
use std::hint::black_box;

fn check_equal<T>(val: T, expected: T, msg: &str) -> TestResult
where
    T: Debug + PartialEq,
{
    if val != expected {
        Err(format!(
            "{}: \n Expected: {:?}\n Found: {:?}",
            msg, expected, val
        ))
    } else {
        Ok(())
    }
}

pub fn check(val: bool, msg: String) -> TestResult {
    if !val {
        Err(msg)
    } else {
        Ok(())
    }
}

// Test that if there is an entry point, the function is part of `all_local_items`.
pub fn test_entry_fn() -> TestResult {
    let entry_fn = stable_mir::entry_fn();
    entry_fn.map_or(Ok(()), |entry_fn| {
        check_body(entry_fn.body());
        let all_items = stable_mir::all_local_items();
        check(
            all_items.contains(&entry_fn),
            format!("Failed to find entry_fn `{:?}`", entry_fn),
        )
    })
}

/// Iterate over local function bodies.
pub fn test_all_fns() -> TestResult {
    let all_items = stable_mir::all_local_items();
    for item in all_items {
        // Get body and iterate over items
        let body = item.body();
        check_body(body);
    }
    Ok(())
}

/// Using these structures will always follow calls to get more details about those structures.
/// Unless user is trying to find a specific type, this will get repetitive.
pub fn test_traits() -> TestResult {
    // FIXME: All trait declarations only return local traits.
    // See https://github.com/rust-lang/project-stable-mir/issues/37
    let all_traits = stable_mir::all_trait_decls();
    for trait_decl in all_traits.iter().map(stable_mir::trait_decl) {
        // Can't compare trait_decl, so just compare a field for now.
        check_equal(
            stable_mir::trait_decl(&trait_decl.def_id).specialization_kind,
            trait_decl.specialization_kind,
            "external crate mismatch",
        )?;
    }

    for trait_impl in stable_mir::all_trait_impls()
        .iter()
        .map(stable_mir::trait_impl)
    {
        check(
            all_traits.contains(&trait_impl.value.def_id),
            format!("Failed to find trait definition {trait_impl:?}"),
        )?;
    }
    Ok(())
}

pub fn test_crates() -> TestResult {
    for krate in stable_mir::external_crates() {
        check_equal(
            stable_mir::find_crate(&krate.name.as_str()),
            Some(krate),
            "external crate mismatch",
        )?;
    }

    let local = stable_mir::local_crate();
    check_equal(
        stable_mir::find_crate(&local.name.as_str()),
        Some(local),
        "local crate mismatch",
    )
}

/// Visit all local types, statements and terminator to ensure nothing crashes.
fn check_body(body: stable_mir::mir::Body) {
    for bb in body.blocks {
        for stmt in bb.statements {
            black_box(matches!(stmt, stable_mir::mir::Statement::Assign(..)));
        }
        black_box(matches!(
            bb.terminator,
            stable_mir::mir::Terminator::Goto { .. }
        ));
    }

    for local in body.locals {
        black_box(matches!(local.kind(), stable_mir::ty::TyKind::Alias(..)));
    }
}
