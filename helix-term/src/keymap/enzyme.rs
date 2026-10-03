//! Primer-specific Enzyme motion adapters.
//!
//! Enzyme should reuse Helix behavior whenever Helix already knows how to do
//! the operation. The commands in this module exist only where Primer wants a
//! different *resulting interaction* from the same Helix operation.
//!
//! The rule is intentionally small:
//!
//! - Helix decides what the motion means and where it goes.
//! - Enzyme decides whether Normal mode should keep Helix's implicit selection.
//!
//! Rust learning note: `//!` is an inner documentation comment. It documents
//! this module rather than the item immediately following the comment.

use helix_core::Range;
use helix_view::document::Mode;

use crate::commands::{Context, MappableCommand};

/// Run an existing Helix motion and normalize its result for Enzyme navigation.
///
/// Some Helix motions represent movement by returning a non-point `Range`.
/// That is useful for Helix's selection-first editing model, but Enzyme keeps
/// navigation and selection as separate grammar concepts. In Normal mode we
/// therefore collapse the Range Helix produced to a cursor. In Select mode we
/// leave Helix's result alone.
///
/// This function intentionally knows nothing about functions, classes,
/// paragraphs, Tree-sitter, or programming languages. That knowledge remains
/// in the Helix command passed to us.
fn navigate_without_selecting(cx: &mut Context, helix_command: MappableCommand) {
    // Rust learning note: `MappableCommand` is an enum whose `Static` variant
    // stores a function pointer. Calling `execute` lets us reuse the exact same
    // command that an ordinary Helix key binding would invoke.
    helix_command.execute(cx);

    // Select mode is explicitly asking for selection behavior, so Enzyme has
    // nothing to normalize there.
    if cx.editor.mode == Mode::Select {
        return;
    }

    // `current!` is a Helix macro. It expands to the boilerplate needed to get
    // mutable access to the active view and document while satisfying Rust's
    // borrowing rules.
    let (view, doc) = current!(cx.editor);

    // Helix has already performed the motion. At this point we alter only the
    // shape of its resulting selection: `Range::point` represents a cursor.
    //
    // NOTE: `from()` is deliberately isolated here. If a Helix motion's true
    // navigation endpoint is not its lexical range start, we can improve this
    // one adapter without teaching Enzyme how that motion itself works.
    let selection = doc
        .selection(view.id)
        .clone()
        .transform(|range| Range::point(range.from()));

    doc.set_selection(view.id, selection);
}

// Thin adapters -------------------------------------------------------------
//
// Each function below delegates the actual motion to Helix. We only create an
// Enzyme wrapper when the Helix command leaves a selection in Normal mode.

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

fn enzyme_goto_next_section(cx: &mut Context) {
    navigate_without_selecting(cx, MappableCommand::goto_next_paragraph);
}

fn enzyme_goto_previous_section(cx: &mut Context) {
    navigate_without_selecting(cx, MappableCommand::goto_prev_paragraph);
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
        doc: "Go to next function",
    };

    pub const enzyme_goto_previous_function: Self = Self::Static {
        name: "enzyme_goto_previous_function",
        fun: enzyme_goto_previous_function,
        doc: "Go to previous function",
    };

    pub const enzyme_goto_next_class: Self = Self::Static {
        name: "enzyme_goto_next_class",
        fun: enzyme_goto_next_class,
        doc: "Go to next class/type",
    };

    pub const enzyme_goto_previous_class: Self = Self::Static {
        name: "enzyme_goto_previous_class",
        fun: enzyme_goto_previous_class,
        doc: "Go to previous class/type",
    };

    pub const enzyme_goto_next_section: Self = Self::Static {
        name: "enzyme_goto_next_section",
        fun: enzyme_goto_next_section,
        doc: "Go to next section",
    };

    pub const enzyme_goto_previous_section: Self = Self::Static {
        name: "enzyme_goto_previous_section",
        fun: enzyme_goto_previous_section,
        doc: "Go to previous section",
    };
}
