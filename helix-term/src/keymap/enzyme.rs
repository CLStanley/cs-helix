//! Primer-specific Enzyme motion commands.
//!
//! Keep Primer's modal grammar implementation isolated from Helix's upstream
//! command implementations. Enzyme navigation deliberately treats a
//! structural object as a destination, not as an implicit selection.
//!
//! Rust learning note: `//!` is an inner documentation comment. Unlike `//`, it
//! documents the module containing this file and can appear in generated docs.

use helix_core::{movement, Range};
use helix_view::document::Mode;

use crate::commands::{Context, MappableCommand};

// Rust learning note: enums are useful when only a fixed set of values is
// valid. Using variants here is safer than passing arbitrary strings around.
// `Copy` lets these small values be copied rather than moved; `Clone` provides
// explicit duplication too.
#[derive(Clone, Copy)]
enum StructuralObject {
    Function,
    Class,
}

impl StructuralObject {
    // `const fn` may be evaluated at compile time when its input is known.
    // `&'static str` is a borrowed string slice that lives for the program's
    // entire lifetime, which is true for string literals such as these.
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
    // `&mut Context` is a mutable borrow: this function may change the editor
    // through `cx`, but it does not take ownership of the Context.
    let count = cx.count();
    let direction = direction.helix_direction();

    // `move` transfers captured values into the closure. Here the values are
    // Copy types, so this is inexpensive and gives the closure clear ownership.
    let motion = move |editor: &mut helix_view::Editor| {
        // `current!` is an existing Helix macro. We reuse Helix infrastructure
        // whenever its semantics already match what Primer needs.
        let (view, doc) = current!(editor);
        let loader = editor.syn_loader.load();

        // `let ... else` is a Rust early-return pattern. `doc.syntax()` returns
        // an Option; `Some` gives us the syntax tree and `None` takes this path.
        let Some(syntax) = doc.syntax() else {
            editor.set_status("Syntax tree is not available in current buffer");
            return;
        };

        let text = doc.text().slice(..);

        // Helix represents even a normal cursor as a Range. `transform` maps
        // each existing range through this closure and builds a new Selection.
        let selection = doc.selection(view.id).clone().transform(|range| {
            // This is deliberately an upstream Helix operation. Primer only
            // adds custom behavior after Helix has found the structural object.
            let object_range = movement::goto_treesitter_object(
                text,
                range,
                object.query_name(),
                direction,
                syntax,
                &loader,
                count,
            );

            // Range orientation can differ when traversing backward. `from()`
            // returns the lexical minimum, so the Enzyme destination is always
            // the object's actual start regardless of traversal direction.
            let destination = object_range.from();

            if editor.mode == Mode::Select {
                // Preserve the original anchor and move the head. That extends
                // the selection to the same destination Normal mode uses.
                Range::new(range.anchor, destination)
            } else {
                // A point Range is Helix's cursor-like one-position range. This
                // is what separates Enzyme navigation from object selection.
                Range::point(destination)
            }
        });

        doc.set_selection(view.id, selection);
    };

    cx.editor.apply_motion(motion);
}

// These tiny adapter functions make the shared implementation available to
// Helix's mappable-command system without duplicating navigation logic.
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

// This is an inherent `impl`: it adds associated items to MappableCommand.
// We use Helix's existing command registry rather than inventing a Primer one.
// Rust normally wants associated constants in UPPER_CASE, but lowercase names
// intentionally match Helix's command vocabulary, so the lint is scoped here.
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
