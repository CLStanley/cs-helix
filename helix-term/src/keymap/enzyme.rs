//! Primer-specific Enzyme motion commands.
//!
//! Keep Primer's modal grammar implementation isolated from Helix's upstream
//! command implementations. Enzyme navigation deliberately treats a
//! structural object as a destination, not as an implicit selection.

use helix_core::{movement, Range};
use helix_view::document::Mode;

use crate::commands::{Context, MappableCommand};

#[derive(Clone, Copy)]
enum StructuralObject {
    Function,
    Class,
}

impl StructuralObject {
    const fn query_name(self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Class => "class",
        }
    }
}

#[derive(Clone, Copy)]
enum StructuralDirection {
    Next,
    Previous,
}

impl StructuralDirection {
    const fn helix_direction(self) -> movement::Direction {
        match self {
            Self::Next => movement::Direction::Forward,
            Self::Previous => movement::Direction::Backward,
        }
    }
}

/// Navigate to the start of a Tree-sitter structural object.
///
/// Helix's structural-object motions intentionally return the whole object as
/// a selection. Enzyme separates navigation from selection: in Normal mode
/// the resulting object range is normalized to a single cursor destination at
/// the object's start. In Select mode the existing anchor is extended to the
/// exact same destination.
fn goto_structural_object(
    cx: &mut Context,
    object: StructuralObject,
    direction: StructuralDirection,
) {
    let count = cx.count();
    let direction = direction.helix_direction();

    let motion = move |editor: &mut helix_view::Editor| {
        let (view, doc) = current!(editor);
        let loader = editor.syn_loader.load();

        let Some(syntax) = doc.syntax() else {
            editor.set_status("Syntax tree is not available in current buffer");
            return;
        };

        let text = doc.text().slice(..);
        let selection = doc.selection(view.id).clone().transform(|range| {
            let object_range = movement::goto_treesitter_object(
                text,
                range,
                object.query_name(),
                direction,
                syntax,
                &loader,
                count,
            );

            // Tree-sitter motions return a range describing the object. The
            // destination of an Enzyme `go` command is always the object's
            // lexical start, independent of traversal direction.
            let destination = object_range.from();

            if editor.mode == Mode::Select {
                Range::new(range.anchor, destination)
            } else {
                Range::point(destination)
            }
        });

        doc.set_selection(view.id, selection);
    };

    cx.editor.apply_motion(motion);
}

fn enzyme_goto_next_function(cx: &mut Context) {
    goto_structural_object(cx, StructuralObject::Function, StructuralDirection::Next);
}

fn enzyme_goto_previous_function(cx: &mut Context) {
    goto_structural_object(
        cx,
        StructuralObject::Function,
        StructuralDirection::Previous,
    );
}

fn enzyme_goto_next_class(cx: &mut Context) {
    goto_structural_object(cx, StructuralObject::Class, StructuralDirection::Next);
}

fn enzyme_goto_previous_class(cx: &mut Context) {
    goto_structural_object(cx, StructuralObject::Class, StructuralDirection::Previous);
}

// Enzyme-specific commands are associated with Helix's mappable-command type
// so the existing keymap/which-key infrastructure can describe them normally
// instead of exposing anonymous "Multiple commands" sequences.
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
