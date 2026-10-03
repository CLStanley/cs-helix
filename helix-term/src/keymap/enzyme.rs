//! Primer-specific Enzyme motion and selection adapters.
//!
//! Enzyme should reuse Helix behavior whenever Helix already knows how to do
//! the operation. The commands in this module exist only where Primer wants a
//! different *resulting interaction* or spelling from the same Helix operation.
//!
//! The rule is intentionally small:
//!
//! - Helix decides what syntax objects mean and how they are selected/moved to.
//! - Enzyme decides how those operations compose into Primer's grammar.
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

// Navigation adapters -------------------------------------------------------

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

// Selection adapters --------------------------------------------------------

/// Ask Helix to select a text object, but supply the object key on Enzyme's
/// behalf instead of waiting for another physical key press.
///
/// Helix's `select_textobject_around` / `select_textobject_inner` commands are
/// deliberately interactive: they install an `on_next_key` callback and then
/// wait for a key such as `f`, `t`, `p`, or `m`. Enzyme already learned the
/// user's intent from its own grammar (`sf`, `sc`, `ss`, `sip`, ...), so these
/// adapters execute the normal Helix command and immediately feed that pending
/// callback the exact key Helix itself expects.
///
/// This is an adapter rather than a second text-object implementation: all
/// Tree-sitter queries, pair matching, counts, and selection behavior remain in
/// Helix.
fn select_helix_textobject(cx: &mut Context, around: bool, helix_object_key: char) {
    let command = if around {
        MappableCommand::select_textobject_around
    } else {
        MappableCommand::select_textobject_inner
    };

    command.execute(cx);

    // Rust learning note: `Option::take()` replaces the field with `None` and
    // gives us ownership of the callback. That matters because `FnOnce` may be
    // called exactly once, and calling it also needs `&mut Context` again.
    let Some((callback, _kind)) = cx.on_next_key_callback.take() else {
        cx.editor
            .set_error("Enzyme expected Helix to request a text-object key");
        return;
    };

    // KeyEvent implements FromStr, so parsing a one-character string gives the
    // same unmodified key event Helix would have received from the keyboard.
    let key = helix_object_key.to_string();
    let Ok(event) = key.parse() else {
        cx.editor
            .set_error("Enzyme could not construct a text-object key");
        return;
    };

    callback(cx, event);
}

fn enzyme_select_line(cx: &mut Context) {
    MappableCommand::extend_line.execute(cx);
}

fn enzyme_select_word(cx: &mut Context) {
    select_helix_textobject(cx, true, 'w');
}

fn enzyme_select_function(cx: &mut Context) {
    select_helix_textobject(cx, true, 'f');
}

fn enzyme_select_class(cx: &mut Context) {
    // Helix names the class/type text object `t` (type definition).
    select_helix_textobject(cx, true, 't');
}

fn enzyme_select_section(cx: &mut Context) {
    // Enzyme calls prose/blank-line-delimited paragraphs "sections"; Helix's
    // underlying text-object key for that concept is `p`.
    select_helix_textobject(cx, true, 'p');
}

fn enzyme_select_block(cx: &mut Context) {
    // `m` is Helix's closest surrounding pair text object.
    select_helix_textobject(cx, true, 'm');
}

fn enzyme_select_inside_pair(cx: &mut Context) {
    select_helix_textobject(cx, false, 'm');
}

fn enzyme_select_outside_pair(cx: &mut Context) {
    select_helix_textobject(cx, true, 'm');
}

fn enzyme_select_document(cx: &mut Context) {
    MappableCommand::select_all.execute(cx);
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

    pub const enzyme_select_line: Self = Self::Static {
        name: "enzyme_select_line",
        fun: enzyme_select_line,
        doc: "Select current line",
    };

    pub const enzyme_select_word: Self = Self::Static {
        name: "enzyme_select_word",
        fun: enzyme_select_word,
        doc: "Select current word",
    };

    pub const enzyme_select_function: Self = Self::Static {
        name: "enzyme_select_function",
        fun: enzyme_select_function,
        doc: "Select current function",
    };

    pub const enzyme_select_class: Self = Self::Static {
        name: "enzyme_select_class",
        fun: enzyme_select_class,
        doc: "Select current class/type",
    };

    pub const enzyme_select_section: Self = Self::Static {
        name: "enzyme_select_section",
        fun: enzyme_select_section,
        doc: "Select current section",
    };

    pub const enzyme_select_block: Self = Self::Static {
        name: "enzyme_select_block",
        fun: enzyme_select_block,
        doc: "Select current block/pair",
    };

    pub const enzyme_select_inside_pair: Self = Self::Static {
        name: "enzyme_select_inside_pair",
        fun: enzyme_select_inside_pair,
        doc: "Select inside closest pair",
    };

    pub const enzyme_select_outside_pair: Self = Self::Static {
        name: "enzyme_select_outside_pair",
        fun: enzyme_select_outside_pair,
        doc: "Select outside/around closest pair",
    };

    pub const enzyme_select_document: Self = Self::Static {
        name: "enzyme_select_document",
        fun: enzyme_select_document,
        doc: "Select whole document",
    };
}
