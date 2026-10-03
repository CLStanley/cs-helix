//! Primer-specific Enzyme motion and selection adapters.
//!
//! Helix owns language/text-object semantics wherever possible. Enzyme owns the
//! predictable grammar layered over those primitives.

use helix_core::{textobject, Range};
use helix_view::document::Mode;
use crate::commands::{Context, MappableCommand};

fn navigate_without_selecting(cx: &mut Context, command: MappableCommand, collapse: fn(Range) -> usize) {
    command.execute(cx);
    if cx.editor.mode == Mode::Select { return; }
    let (view, doc) = current!(cx.editor);
    let selection = doc.selection(view.id).clone().transform(|range| Range::point(collapse(range)));
    doc.set_selection(view.id, selection);
}

fn structural_start(range: Range) -> usize { range.from() }
fn motion_head(range: Range) -> usize { range.head }

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

/// Reuse one of Helix's interactive text objects without asking the user for a
/// second object key. `Option::take` moves the one-shot callback out so Rust
/// allows us to invoke it with the mutable command context.
fn select_helix_textobject(cx: &mut Context, around: bool, object_key: char) {
    let command = if around { MappableCommand::select_textobject_around } else { MappableCommand::select_textobject_inner };
    command.execute(cx);
    let Some((callback, _kind)) = cx.on_next_key_callback.take() else {
        cx.editor.set_error("Enzyme expected Helix to request a text-object key");
        return;
    };
    let Ok(event) = object_key.to_string().parse() else {
        cx.editor.set_error("Enzyme could not construct a text-object key");
        return;
    };
    callback(cx, event);
}

/// Select Helix's closest matching pair directly.
///
/// The generic text-object command passes pair ranges through Helix's motion
/// machinery, which collapsed the result during Primer's Normal-mode testing.
/// `sp`/`sip` are explicit selection operations, so Enzyme installs the range
/// returned by Helix-core's pair matcher directly. Pair recognition remains a
/// Helix responsibility; Enzyme only chooses Around versus Inside.
fn select_pair(cx: &mut Context, object: textobject::TextObject) {
    let (view, doc) = current!(cx.editor);
    let text = doc.text().slice(..);
    let syntax = doc.syntax();
    let selection = doc.selection(view.id).clone().transform(|range| {
        textobject::textobject_pair_surround_closest(syntax, text, range, object, 1)
    });
    doc.set_selection(view.id, selection);
}

fn enzyme_select_line(cx: &mut Context) { MappableCommand::extend_line.execute(cx); }
fn enzyme_select_word(cx: &mut Context) { select_helix_textobject(cx, true, 'w'); }
fn enzyme_select_function(cx: &mut Context) { select_helix_textobject(cx, true, 'f'); }
fn enzyme_select_class(cx: &mut Context) { select_helix_textobject(cx, true, 't'); }
fn enzyme_select_section(cx: &mut Context) { select_helix_textobject(cx, true, 'p'); }

fn enzyme_select_block(cx: &mut Context) {
    // Compatibility only: upstream Helix has no language-neutral `block`
    // text-object capture. Primer must replace this pair fallback with a true
    // Tree-sitter-defined block object rather than defining block as braces.
    select_helix_textobject(cx, true, 'm');
}

fn enzyme_select_pair(cx: &mut Context) { select_pair(cx, textobject::TextObject::Around); }
fn enzyme_select_inside_pair(cx: &mut Context) { select_pair(cx, textobject::TextObject::Inside); }
fn enzyme_select_document(cx: &mut Context) { MappableCommand::select_all.execute(cx); }

#[allow(non_upper_case_globals)]
impl MappableCommand {
    pub const enzyme_goto_next_function: Self = Self::Static { name: "enzyme_goto_next_function", fun: enzyme_goto_next_function, doc: "Go to next function" };
    pub const enzyme_goto_previous_function: Self = Self::Static { name: "enzyme_goto_previous_function", fun: enzyme_goto_previous_function, doc: "Go to previous function" };
    pub const enzyme_goto_next_class: Self = Self::Static { name: "enzyme_goto_next_class", fun: enzyme_goto_next_class, doc: "Go to next class/type" };
    pub const enzyme_goto_previous_class: Self = Self::Static { name: "enzyme_goto_previous_class", fun: enzyme_goto_previous_class, doc: "Go to previous class/type" };
    pub const enzyme_goto_next_section: Self = Self::Static { name: "enzyme_goto_next_section", fun: enzyme_goto_next_section, doc: "Go to next section" };
    pub const enzyme_goto_previous_section: Self = Self::Static { name: "enzyme_goto_previous_section", fun: enzyme_goto_previous_section, doc: "Go to previous section" };
    pub const enzyme_select_line: Self = Self::Static { name: "enzyme_select_line", fun: enzyme_select_line, doc: "Select current line" };
    pub const enzyme_select_word: Self = Self::Static { name: "enzyme_select_word", fun: enzyme_select_word, doc: "Select current word" };
    pub const enzyme_select_function: Self = Self::Static { name: "enzyme_select_function", fun: enzyme_select_function, doc: "Select current function" };
    pub const enzyme_select_class: Self = Self::Static { name: "enzyme_select_class", fun: enzyme_select_class, doc: "Select current class/type" };
    pub const enzyme_select_section: Self = Self::Static { name: "enzyme_select_section", fun: enzyme_select_section, doc: "Select current section" };
    pub const enzyme_select_block: Self = Self::Static { name: "enzyme_select_block", fun: enzyme_select_block, doc: "Select current structural block" };
    pub const enzyme_select_pair: Self = Self::Static { name: "enzyme_select_pair", fun: enzyme_select_pair, doc: "Select closest matching pair" };
    pub const enzyme_select_inside_pair: Self = Self::Static { name: "enzyme_select_inside_pair", fun: enzyme_select_inside_pair, doc: "Select inside closest matching pair" };
    // Temporary alias so the branch remains buildable until default.rs is moved
    // from the obsolete `sop` spelling to `sp`. It deliberately performs the
    // same whole-pair operation that `sp` will own.
    pub const enzyme_select_outside_pair: Self = Self::Static { name: "enzyme_select_outside_pair", fun: enzyme_select_pair, doc: "Select closest matching pair" };
    pub const enzyme_select_document: Self = Self::Static { name: "enzyme_select_document", fun: enzyme_select_document, doc: "Select whole document" };
}
