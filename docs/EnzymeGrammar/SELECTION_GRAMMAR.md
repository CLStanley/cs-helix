# Primer Text Editor — Enzyme Selection Grammar

> **Status:** Proposed v1 — active implementation/dogfooding
>
> **Parent specification:** `MODAL_GRAMMAR.md`

## Purpose

Primer has two complementary ways to select text:

```text
s...    deliberately select a known object
v       enter interactive Select/Visual mode
```

`s` is the explicit object-selection grammar. `v` preserves Helix's interactive selection-extension concept: once Select/Visual mode is active, compatible motions extend the current selection instead of merely moving it.

Primer preserves Helix's first-class selection model. Enzyme changes the vocabulary used to express intent, not the underlying range/multiple-selection machinery.

## Core selection rule

Enzyme selection has one general relationship rule:

> **`s + object` selects the whole object. `s + i + object` selects the contents inside that object.**

The whole-object form includes the object's boundaries when the object has meaningful boundaries. The `i` modifier removes those boundaries and selects the contents.

This replaces the earlier `inside` / `outside` split. There is no `so...` family: selecting the object itself already means selecting the whole/around object, so an additional `outside` spelling would be redundant.

Primer should only advertise `si<object>` combinations for which the underlying editor can define a useful and reliable inside range.

## Core object vocabulary

| Key | Object | Notes |
|---|---|---|
| `l` | line | Whole current line. |
| `w` | word | Word under/at cursor. |
| `f` | function/method | Language-neutral callable concept. |
| `c` | class/type | Structural type/class declaration. |
| `b` | structural block | **Tree-sitter/language-defined** block. |
| `s` | section | Paragraph/logical text section containing the cursor. |
| `p` | pair | Matching delimiters such as `()`, `[]`, `{}`, quotes, etc. |
| `d` | document | Whole document. |

Structural objects should use Tree-sitter where practical. Section is Primer's user-facing term for the paragraph-like text unit traditionally called a paragraph by modal editors.

### Block and pair are intentionally different

A **block** is a semantic/syntactic object defined by the language's Tree-sitter support. Primer should not redefine a block as "whatever is inside braces." Different languages may recognize different structures as blocks.

A **pair** is a matching-delimiter relationship that is useful even when the user does not yet understand a language's structural grammar:

```text
( ... )
[ ... ]
{ ... }
" ... "
' ... '
```

A brace-delimited construct may happen to be both a Tree-sitter block and a pair, but those are separate concepts.

This distinction provides a deliberate fallback for unfamiliar languages. A user who does not yet know what that language considers a block can still operate on visually recognizable matching pairs until they learn the language's structural objects.

## Direct object selection

| Key | Meaning |
|---|---|
| `sl` | select line |
| `sw` | select word |
| `sf` | select current function/method |
| `sc` | select current class/type |
| `sb` | select current Tree-sitter structural block |
| `ss` | select current section |
| `sp` | select surrounding pair, including delimiters |
| `sd` | select whole document |

These acquire the containing/current object directly. They are for the thought "select this thing," rather than "start selecting while I move."

`sd` is Enzyme's select-all spelling. `d` already means document in Navigation (`gsd`, `ged`, etc.), so `sd` follows the grammar without introducing a separate `all` concept. The `s` namespace also makes it less likely to trigger accidentally than a single immediate select-all key.

## Inside selection

`i` means **inside** and appears between the Select verb and the object:

```text
s i [object]
```

Established examples:

| Key | Meaning |
|---|---|
| `sip` | select inside surrounding pair, excluding delimiters |
| `sib` | select inside current Tree-sitter block, excluding its structural boundaries when Helix/Tree-sitter can define that range reliably |

The same rule can extend naturally to other objects (`sif`, `sic`, `sis`, etc.) only when Helix/Tree-sitter supplies a meaningful distinction between the whole object and its contents. Primer should not invent arbitrary inside semantics merely to make the grammar exhaustive.

For pairs, the intended behavior is explicit:

```text
foo(alpha, beta)
   ^-----------^   sp  -> (alpha, beta)
    ^---------^    sip -> alpha, beta
```

Pair selection must use matching-pair semantics rather than treating a pair as a Tree-sitter block.

## Canceling an accidental/transient selection

Primer distinguishes **leaving a mode**, **canceling a transient operation**, and **undoing an edit**:

```text
Esc         exit/cancel the current mode or incomplete grammar sentence
Backspace   "oops, never mind" — cancel the current transient Enzyme operation
u           undo an actual edit
;           collapse the current selection at its active position
```

For an Enzyme object selection, Backspace should restore the selection/cursor state that existed immediately before the transient selection operation when that state is available.

Example:

```text
cursor inside a function
sf
Backspace
```

The function selection is abandoned and the cursor returns to its pre-`sf` position.

Backspace is **not** an editing undo mechanism. Once an action has changed the document, `u` remains the way to undo that edit.

Esc must not acquire hidden "rewind all Select-mode navigation" semantics. In Select/Visual mode, Esc continues to mean leave the mode according to Primer's modal rules.

## Interactive Select/Visual mode

```text
v    enter Select/Visual mode
```

Select/Visual mode supplies the selection intent implicitly. Therefore Enzyme does **not** add a redundant `se... = select → extend...` family.

Compatible motions retain their normal spelling but extend the active selection. The general rule is:

> **In Select/Visual mode, a compatible movement/navigation sentence extends the selection to the destination described by that sentence.**

The grammar is therefore learned once. Primer should not maintain a second parallel set of extension mnemonics when mode context already supplies the verb.

Not every Navigation command can sensibly extend a contiguous selection. Structural and local motions generally can; cross-document semantic jumps may not. Primer should preserve useful Helix behavior where possible, but must not pretend a cross-buffer range exists when the underlying editor cannot represent one.

`Esc` exits Select/Visual mode toward the safe normal state. It does not rewind all movement performed while Select mode was active.

## Multiple-selection grammar

Multiple selections remain first-class Helix selections. Enzyme supplies predictable grammar for common selection-building operations where doing so materially improves normal editing.

```text
s a ...    select → add ...
s r ...    select → remove ...
s k ...    select → keep/filter ...
s x ...    select → split ...
```

### Add matching selections

```text
san    select → add → next matching occurrence
saN    select → add → previous matching occurrence
saa    select → add → all matching occurrences
```

The match source is the current selection text/range, not necessarily a lexical word.

### Remove selections

```text
srn    select → remove → next selection
srN    select → remove → previous selection
sra    select → remove → all additional/secondary selections
```

`sra` leaves the primary selection intact.

### Keep/filter and split

```text
skm    select → keep → matching...
srm    select → remove → matching...

sx...  select → split ...
sxl    split selection into lines
sxm    split selection by matches...
```

## Advanced selection-set controls

Enzyme lowers the learning curve; it does not attempt to eliminate learning. Core navigation and selection should be predictable from a small vocabulary. Specialized controls may still require learning and reference, just as advanced features in a conventional editor do.

Primer therefore intentionally preserves these concise Helix controls rather than inventing a longer Enzyme namespace for them:

```text
(       rotate selections backward / change primary backward
)       rotate selections forward / change primary forward
,       keep only the primary selection
Alt+,   remove the primary selection
```

These are niche selection-set management operations, not part of the core object-selection/navigation philosophy. Their existence does not justify complicating the everyday `s` grammar.

The same standard applies during the broader parity audit: **do not remap an advanced Helix capability merely because Enzyme can invent a grammatical spelling for it. Change it when doing so materially lowers the learning curve or resolves a real inconsistency.**

Other advanced Helix selection operations—alignment, selection-direction flipping, rotating selection contents, syntax-tree growth/shrinkage, sibling/parent/child operations, and similar tools—must remain reachable. They may retain sensible Helix bindings unless dogfooding demonstrates a meaningful reason to promote them into Enzyme grammar.

## Actions after selection

Selection hands off naturally to direct editing actions:

```text
s l d    select line → delete
s l c    select line → comment
s w m    select word → modify
s f y    select function → yank
s s c    select section → comment
s i p d  select inside pair → delete
```

Editing actions operate across all active selections where supported.

## Helper examples

After `s`:

```text
SELECT…

l    line
w    word
f    function/method
c    class/type
b    block (Tree-sitter/language-defined)
s    section
p    pair
d    document
i    inside…
a    add selection…
r    remove selection…
k    keep/filter selections…
x    split selection…
```

Select/Visual mode should instead surface compatible motions/navigation because `v` already supplies the Select/Extend intent.

## Design constraints

1. `s` means deliberate object-selection grammar.
2. `v` preserves interactive Select/Visual mode.
3. `s + object` selects the whole object; `s + i + object` selects inside it when a meaningful inside range exists.
4. There is no redundant `so...` outside/around family; whole-object selection already supplies that meaning.
5. `block` is Tree-sitter/language-defined; `pair` is a separate matching-delimiter concept and acts as a predictable fallback in unfamiliar languages.
6. In Select/Visual mode, compatible navigation extends the selection; do not require a redundant `se` prefix.
7. Preserve Helix's first-class and multiple-selection model.
8. Reuse Navigation vocabulary rather than creating a parallel extension vocabulary.
9. `sd` is select document/select all.
10. Backspace means cancel the current transient Enzyme operation and restore its pre-operation state when available; Esc remains mode exit/cancel, and `u` remains editing undo.
11. `n/N` retain next/previous meaning inside selection-set operations.
12. Prefer the strongest mnemonic for common/core operations; rarer advanced controls may retain concise established bindings.
13. Primer must account for existing Helix motion/selection capabilities through an explicit parity audit before Enzyme v1 is declared complete.
14. Enzyme lowers the learning curve rather than eliminating learning. Advanced operations may require learning/reference even when the core grammar is highly predictable.
15. Do not invent bindings merely to make the grammar exhaustive; preserve sensible conventions when they already satisfy Primer's philosophy.

## Selection v1 implementation checklist

`se...` is intentionally removed. `v` plus compatible motion is the extension model.

`so...` is intentionally removed. Selecting an object directly is the whole/around form; `si...` is the inside modifier.

The old proposed `spn/spN/spo` primary-selection namespace is intentionally removed. `sp` means select pair; Primer preserves Helix's concise `(`/`)`/`,`/`Alt+,` controls for primary-selection management.

Before Enzyme v1 is frozen, Primer still needs:

- implementation verification for `sp` / `sip` using true matching-pair semantics;
- implementation verification for `sb` / `sib` using Tree-sitter-defined block semantics;
- Backspace transient-selection cancellation/restoration;
- Select/Visual-mode navigation parity for compatible Enzyme motions;
- a Helix parity audit to ensure advanced capabilities remain reachable without automatically remapping them;
- verification of regex selection/filter/split behavior for the common Enzyme multiple-selection grammar;
- verification of commands whose Helix behavior cannot sensibly map to a contiguous selection across buffers/documents.

After that audit, only actual uncovered capabilities that materially benefit from Enzyme grammar should receive new grammar decisions.
