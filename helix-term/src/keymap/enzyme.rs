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

/// Run an existing Helix motion and collapse each resulting range with `collapse`.
///
/// Rust learning note: `fn(Range) -> usize` is a function-pointer type. Passing
/// the collapse rule as a function lets this adapter share all of the command
/// execution/selection plumbing while still respecting different kinds of Helix
/// motions. This is preferable to teaching one generic rule about semantics that
/// are not actually generic.
fn navigate_without_selecting(
    cx: &mut Context,
    helix_command: MappableCommand,
    collapse: fn(Range) -> usize,
) {
    helix_command.execute(cx);

    // Select mode explicitly asks Helix to retain selection behavior.
    if cx.editor.mode == Mode::Select {
        return;
    }

    let (view, doc) = current!(cx.editor);
    let selection = doc
        .selection(view.id)
        .clone()
        .transform(|range| Range::point(collapse(range)));

    doc.set_selection(view.id, selection);
}

/// Structural Tree-sitter objects have a stable lexical start.
///
/// Helix's `goto_treesitter_object` constructs the object as
/// `Range::new(start_char, end_char)`, then `goto_ts_object_impl` changes only
/// its direction. Consequently `Range::from()` remains the object's real start
/// for both next and previous structural navigation.
fn structural_start(range: Range) -> usize {
    range.from()
}

/// Paragraph/section movement is directional rather than a Tree-sitter object
/// lookup. For those motions the active `head` is the destination Helix chose.
fn motion_head(range: Range) -> usize {
    range.head
}

// Thin adapters -------------------------------------------------------------
//
// Functions/classes use the lexical start of the structural object. Sections
// use Helix's directional motion head. Helix still owns all object detection and
// movement logic; Enzyme only removes the implicit Normal-mode selection.

fn enzyme_goto_next_function(cx: &mut Context) {
    navigate_without_selecting(
        cx,
        MappableCommand::goto_next_function,
        structural_start,
    );
}

fn enzyme_goto_previous_function(cx: &mut Context) {
    navigate_without_selecting(
        cx,
        MappableCommand::goto_prev_function,
        structural_start,
    );
}

fn enzyme_goto_next_class(cx: &mut Context) {
    navigate_without_selecting(cx, MappableCommand::goto_next_class, structural_start);
}

fn enzyme_goto_previous_class(cx: &mut Context) {
    navigate_without_selecting(cx, MappableCommand::goto_prev_class, structural_start);
}

fn enzyme_goto_next_section(cx: &mut Context) {
    navigate_without_selecting(cx, MappableCommand::goto_next_paragraph, motion_head);
}

fn enzyme_goto_previous_section(cx: &mut Context) {
    navigate_without_selecting(cx, MappableCommand::goto_prev_paragraph, motion_head);
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
