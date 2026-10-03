//! Primer-specific Enzyme motion and selection adapters.
//!
//! Enzyme reuses Helix behavior whenever Helix already knows how to do the
//! operation. Primer-specific code belongs here only when Enzyme needs a
//! different interaction or spelling.

use helix_core::{textobject, Range};
use helix_view::document::Mode;

use crate::commands::{Context, MappableCommand};

/// Run an existing Helix motion and collapse each resulting range with
/// `collapse` when Enzyme is in Normal mode.
fn navigate_without_selecting(
    cx: &mut Context,
    helix_command: MappableCommand,
    collapse: fn(Range) -> usize,
) {
    helix_command.execute(cx);

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

/// Tree-sitter structural motions produce a range whose lexical start is the
/// structural destination Enzyme wants in Normal mode.
fn structural_start(range: Range) -> usize {
    range.from()
}

/// Directional paragraph/section motions use their active head as destination.
fn motion_head(range: Range) -> usize {
    range.head
}

fn enzyme_goto_next_function(cx: &mut Context) {
    navigate_without_selecting(cx, MappableCommand::goto_next_function, structural_start);
}

fn enzyme_goto_previous_function(cx: &mut Context) {
    navigate_without_selecting(cx, MappableCommand::goto_prev_function, structural_start);
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

/// Ask Helix's interactive text-object command to select one known object.
///
/// Rust learning note: `Option::take()` moves the one-shot callback out of the
/// Context while replacing the field with `None`, which lets us invoke it with
/// `&mut Context` without violating Rust's borrowing rules.
fn select_helix_textobject(cx: &mut Context, around: bool, helix_object_key: char) {
    let command = if around {
        MappableCommand::select_textobject_around
    } else {
        MappableCommand::select_textobject_inner
    };
    command.execute(cx);

    let Some((callback, _kind)) = cx.on_next_key_callback.take() else {
        cx.editor
            .set_error("Enzyme expected Helix to request a text-object key");
        return;
    };

    let key = helix_object_key.to_string();
    let Ok(event) = key.parse() else {
        cx.editor
            .set_error("Enzyme could not construct a text-object key");
        return;
    };
    callback(cx, event);
}

/// Select the closest matching delimiter pair directly.
///
/// Helix's generic text-object command is normally executed as a motion. That
/// path is useful for Helix's modal semantics, but during Primer dogfooding it
/// caused the closest-pair result to collapse to one boundary in Normal mode.
/// Enzyme's `sp`/`sip` are explicit selection commands, so we call Helix-core's
/// pair text-object primitive directly and install the resulting ranges.
///
/// Helix still owns pair recognition (including syntax-aware matching); Enzyme
/// only decides whether the requested object is Around or Inside.
fn select_pair(cx: &mut Context, object: textobject::TextObject) {
    let (view, doc) = current!(cx.editor);
    let text = doc.text().slice(..);
    let syntax = doc.syntax();
    let selection = doc.selection(view.id).clone().transform(|range| {
        textobject::textobject_pair_surround_closest(syntax, text, range, object, 1)
    });
    doc.set_selection(view.id, selection);
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
    select_helix_textobject(cx, true, 't');
}

fn enzyme_select_section(cx: &mut Context) {
    select_helix_textobject(cx, true, 'p');
}

fn enzyme_select_block(cx: &mut Context) {
    // Temporary compatibility behavior. Helix does not currently expose a
    // language-neutral `block` text-object capture. Keep this command isolated
    // here while Primer adds a real Tree-sitter-defined block object rather than
    // baking brace/pair semantics into the Enzyme grammar.
    select_helix_textobject(cx, true, 'm');
}

fn enzyme_select_pair(cx: &mut Context) {
    select_pair(cx, textobject::TextObject::Around);
}

fn enzyme_select_inside_pair(cx: &mut Context) {
    select_pair(cx, textobject::TextObject::Inside);
}

fn enzyme_select_document(cx: &mut Context) {
    MappableCommand::select_all.execute(cx);
}

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
        doc: "Select current structural block",
    };
    pub const enzyme_select_pair: Self = Self::Static {
        name: "enzyme_select_pair",
        fun: enzyme_select_pair,
        doc: "Select closest matching pair",
    };
    pub const enzyme_select_inside_pair: Self = Self::Static {
        name: "enzyme_select_inside_pair",
        fun: enzyme_select_inside_pair,
        doc: "Select inside closest matching pair",
    };
    pub const enzyme_select_document: Self = Self::Static {
        name: "enzyme_select_document",
        fun: enzyme_select_document,
        doc: "Select whole document",
    };
}
