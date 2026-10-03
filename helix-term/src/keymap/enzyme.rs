//! Primer-specific Enzyme motion adapters.
//!
//! Enzyme should reuse Helix behavior whenever Helix already knows how to do
//! the operation. The commands in this module exist only where Primer wants a
//! different *resulting interaction* from the same Helix operation.
//!
//! For structural navigation, Helix already knows what a function or class is,
//! how Tree-sitter represents it for each language, and how to find the next or
//! previous one. Enzyme therefore delegates that work to Helix and only changes
//! the resulting structural selection into a navigation destination.
//!
//! Rust learning note: `//!` is an inner documentation comment. It documents
//! the module itself rather than the item immediately following the comment.

use helix_core::Range;
use helix_view::document::Mode;

use crate::commands::{Context, MappableCommand};

/// Run one of Helix's structural-navigation commands, then apply Enzyme's
/// navigation semantics to the selection Helix produced.
///
/// The important boundary here is intentional:
///
/// 1. `helix_command.execute(cx)` lets Helix determine the structural object.
/// 2. Enzyme only post-processes Helix's resulting selection.
///
/// That keeps language and Tree-sitter knowledge in Helix instead of duplicating
/// it in Primer.
fn navigate_without_selecting(cx: &mut Context, helix_command: MappableCommand) {
    // Rust learning note: `MappableCommand` is an enum. Calling `execute` here
    // uses Helix's public command abstraction instead of reaching into private
    // functions such as `goto_next_function` directly.
    helix_command.execute(cx);

    // In Select mode Helix already has the semantics we want: preserve the
    // original anchor and extend to the structural destination. Selection-mode
    // grammar can therefore continue to use Helix behavior unchanged.
    if cx.editor.mode == Mode::Select {
        return;
    }

    // Rust learning note: `current!` is a Helix macro. A macro can expand to
    // code that would be repetitive to write by hand; here it retrieves the
    // active view and document while handling their borrowing correctly.
    let (view, doc) = current!(cx.editor);

    // Helix has already selected the exact object it considers next/previous.
    // We do not inspect syntax or try to identify that object ourselves. We
    // simply collapse each resulting range to its lexical start.
    //
    // `Range::from()` returns the lower document position regardless of whether
    // Helix oriented the range forward or backward. `Range::point()` then turns
    // that position into a cursor-sized range.
    let selection = doc
        .selection(view.id)
        .clone()
        .transform(|range| Range::point(range.from()));

    doc.set_selection(view.id, selection);
}

// These functions are deliberately tiny adapters. Notice that there is no
// Tree-sitter query, object-name string, or direction calculation here: Helix's
// existing commands remain responsible for all of those decisions.
fn enzyme_goto_next_function(cx: &mut Context) {
    navigate_without_selecting(cx, MappableCommand::goto_next_function);
}

fn enzyme_goto_previous_function(cx: &mut Context) {
    navigate_without_selecting(cx, MappableCommand::goto_prev_function);
}

fn enzyme_goto_next_class(cx: &mut Context) {
    navigate_without_selecting(cx, MappableCommand::goto_next_class);
}

fn enzyme_goto_previous_class(cx: &mut Context) {
    navigate_without_selecting(cx, MappableCommand::goto_prev_class);
}

// Rust learning note: this is an inherent `impl` block. It adds associated
// constants to the existing `MappableCommand` type. Each constant packages a
// function pointer and help text in the same form Helix's keymap already uses.
//
// Rust convention normally requires associated constants to be UPPER_CASE.
// These names intentionally follow Helix's lowercase command vocabulary, so we
// suppress that lint only for this block.
#[allow(non_upper_case_globals)]
impl MappableCommand {
    pub const enzyme_goto_next_function: Self = Self::Static {
        name: "enzyme_goto_next_function",
        fun: enzyme_goto_next_function,
        doc: "Go to start of next function",
    };

    pub const enzyme_goto_previous_function: Self = Self::Static {
        name: "enzyme_goto_previous_function",
        fun: enzyme_goto_previous_function,
        doc: "Go to start of previous function",
    };

    pub const enzyme_goto_next_class: Self = Self::Static {
        name: "enzyme_goto_next_class",
        fun: enzyme_goto_next_class,
        doc: "Go to start of next class/type",
    };

    pub const enzyme_goto_previous_class: Self = Self::Static {
        name: "enzyme_goto_previous_class",
        fun: enzyme_goto_previous_class,
        doc: "Go to start of previous class/type",
    };
}
