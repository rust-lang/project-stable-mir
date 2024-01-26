//! Module that contains sanity checks that Stable MIR APIs don't crash and that
//! their result is coherent.
//!
//! These checks should only depend on StableMIR APIs. See other modules for tests that compare
//! the result between StableMIR and internal APIs.
use crate::TestResult;
use stable_mir::ty::{ImplDef, TraitDef};
use stable_mir::{self, mir, mir::MirVisitor, ty, CrateDef};
use std::collections::HashSet;
use std::fmt::Debug;
use std::iter::zip;

fn check_equal<T>(val: T, expected: T, msg: &str) -> TestResult
where
    T: Debug + PartialEq,
{
    if val != expected {
        Err(format!(
            "{}: \n Expected: `{:?}`\n Found: `{:?}`",
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
        check_body(&entry_fn.name(), &entry_fn.body())?;
        let all_items = stable_mir::all_local_items();
        check(
            all_items.contains(&entry_fn),
            format!("Failed to find entry_fn: `{:?}`", entry_fn),
        )?;
        check_equal(
            entry_fn.kind(),
            stable_mir::ItemKind::Fn,
            "Entry must be a function",
        )
    })
}

/// Iterate over local function bodies.
pub fn test_all_fns() -> TestResult {
    let all_items = stable_mir::all_local_items();
    for item in all_items {
        // Get body and iterate over items
        let body = item.body();
        check_body(&item.name(), &body)?;
    }
    Ok(())
}

/// Test that we can retrieve information about the trait declaration for every trait implementation.
pub fn test_traits() -> TestResult {
    let all_traits = HashSet::<TraitDef>::from_iter(stable_mir::all_trait_decls().into_iter());
    for trait_impl in stable_mir::all_trait_impls()
        .iter()
        .map(ImplDef::trait_impl)
    {
        check(
            all_traits.contains(&trait_impl.value.def_id),
            format!("Failed to find trait definition: `{trait_impl:?}`"),
        )?;
    }
    Ok(())
}

pub fn test_crates() -> TestResult {
    for krate in stable_mir::external_crates() {
        check(
            stable_mir::find_crates(&krate.name.as_str()).contains(&krate),
            format!("Cannot find `{krate:?}`"),
        )?;
    }

    let local = stable_mir::local_crate();
    check(
        stable_mir::find_crates(&local.name.as_str()).contains(&local),
        format!("Cannot find local: `{local:?}`"),
    )
}

pub fn test_instances() -> TestResult {
    let all_items = stable_mir::all_local_items();
    let mut queue = all_items
        .iter()
        .filter_map(|item| {
            (item.kind() == stable_mir::ItemKind::Fn)
                .then(|| mir::mono::Instance::try_from(*item).ok())
                .flatten()
        })
        .collect::<Vec<_>>();

    let mut visited = HashSet::<mir::mono::Instance>::default();
    while let Some(next_item) = queue.pop() {
        if visited.insert(next_item.clone()) {
            let Some(body) = next_item.body() else {
                continue;
            };
            let visitor = check_body(&next_item.mangled_name(), &body)?;
            for term in visitor.terminators {
                match &term.kind {
                    // We currently don't support Copy / Move `ty()` due to missing Place::ty().
                    // https://github.com/rust-lang/project-stable-mir/issues/49
                    mir::TerminatorKind::Call {
                        func: mir::Operand::Constant(constant),
                        ..
                    } => {
                        match constant.ty().kind().rigid() {
                            Some(ty::RigidTy::FnDef(def, args)) => {
                                queue.push(mir::mono::Instance::resolve(*def, &args).unwrap());
                            }
                            Some(ty::RigidTy::FnPtr(..)) => { /* ignore FnPtr for now */ }
                            ty => check(false, format!("Unexpected call: `{ty:?}`"))?,
                        }
                    }
                    _ => { /* Do nothing */ }
                }
            }
        }
    }
    Ok(())
}

/// Visit all local types, statements and terminator to ensure nothing crashes.
fn check_body(name: &str, body: &mir::Body) -> Result<BodyVisitor, String> {
    let mut visitor = BodyVisitor::default();
    visitor.visit_body(body);

    check_equal(
        body.blocks.len(),
        visitor.statements.len(),
        &format!("Function `{name}`: Unexpected visited BB statements"),
    )?;
    check_equal(
        body.blocks.len(),
        visitor.terminators.len(),
        &format!("Function `{name}`: Visited terminals"),
    )?;
    for (idx, bb) in body.blocks.iter().enumerate() {
        for (stmt, visited_stmt) in zip(&bb.statements, &visitor.statements[idx]) {
            check_equal(
                stmt,
                visited_stmt,
                &format!("Function `{name}`: Visited statement"),
            )?;
        }
        check_equal(
            &bb.terminator,
            &visitor.terminators[idx],
            &format!("Function `{name}`: Terminator"),
        )?;
    }

    for local in body.locals() {
        if !visitor.types.contains(&local.ty) {
            // Format fails due to unsupported CoroutineWitness.
            // See https://github.com/rust-lang/project-stable-mir/issues/50.
            check(
                false,
                format!("Function `{name}`: Missing type `{:?}`", local.ty),
            )?;
        };
    }
    Ok(visitor)
}

#[derive(Debug, Default)]
struct BodyVisitor {
    statements: Vec<Vec<mir::Statement>>,
    terminators: Vec<mir::Terminator>,
    types: HashSet<ty::Ty>,
}

impl mir::MirVisitor for BodyVisitor {
    fn visit_basic_block(&mut self, bb: &mir::BasicBlock) {
        assert_eq!(self.statements.len(), self.terminators.len());
        self.statements.push(vec![]);
        self.super_basic_block(bb)
    }
    fn visit_statement(&mut self, stmt: &mir::Statement, loc: mir::visit::Location) {
        self.statements.last_mut().unwrap().push(stmt.clone());
        self.super_statement(stmt, loc)
    }

    fn visit_terminator(&mut self, term: &mir::Terminator, location: mir::visit::Location) {
        self.terminators.push(term.clone());
        self.super_terminator(term, location);
    }

    fn visit_ty(&mut self, ty: &ty::Ty, _location: mir::visit::Location) {
        self.types.insert(*ty);
        self.super_ty(ty)
    }
}
