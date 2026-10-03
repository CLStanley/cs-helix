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

Compatible motions retain their normal spelling but extend the active selection:

```text
Normal mode                 Select/Visual mode
-----------                 ------------------
h    move left              h    extend left
j    move down              j    extend down
k    move up                k    extend up
l    move right             l    extend right
w    word motion            w    extend by word

gsl  go start line          gsl  extend to start line
gel  go end line            gel  extend to end line

gsf  go start prev func     gsf  extend toward start prev func
gSf  go start this func     gSf  extend toward start this func
gsF  go start next func     gsF  extend toward start next func
```

The general rule is:

> **In Select/Visual mode, a compatible movement/navigation sentence extends the selection to the destination described by that sentence.**

The grammar is therefore learned once. Primer should not maintain a second parallel set of extension mnemonics when mode context already supplies the verb.

### Compatibility boundary

Not every Navigation command can sensibly extend a contiguous selection. Structural and local motions generally can; cross-document semantic jumps may not.

Examples such as `gd` (definition), `gi` (implementation), or `gt` (type definition) must be audited against Helix behavior and Primer's selection model before implementation. Primer should preserve useful Helix behavior where possible, but must not pretend a cross-buffer range exists when the underlying editor cannot represent one.

`Esc` exits Select/Visual mode toward the safe normal state according to Primer's general mode rules. It does not rewind all movement performed while Select mode was active.

## Multiple-selection grammar

Multiple selections remain first-class Helix selections. Enzyme supplies a predictable language over the existing machinery.

```text
s a ...    select → add ...
s r ...    select → remove ...
s p ...    select → primary ...
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

Typical workflows:

```text
sw → san → san → m
select word → add next → add next → modify all

sw → saa → m
select word → add all matching occurrences → modify all
```

### Remove selections

```text
srn    select → remove → next selection
srN    select → remove → previous selection
sra    select → remove → all additional/secondary selections
```

`sra` leaves the primary selection intact. The helper should describe it as "remove all additional selections."

### Primary selection

```text
spn    select → primary → next
spN    select → primary → previous
spo    select → primary → only
```

`spn/spN` rotate which existing selection is primary. `spo` keeps only the primary selection.

**Grammar note:** direct `sp` now means **select pair**. The longer `sp...` primary-selection branch therefore has a prefix collision and must be revisited before implementation. Do not silently overload `sp` as both a completed pair command and a primary-selection namespace.

### Keep/filter selections

```text
skm    select → keep → matching...
srm    select → remove → matching...
```

Both prompt for a pattern/filter.

### Split selections

`x` means split in this selection-set branch. The weaker mnemonic is intentional because `ss = select section` is the stronger core-object command.

```text
sx...    select → split ...
sxl      split selection into lines
sxm      split selection by matches...
```

## Advanced Helix selection operations

Primer's goal is parity with useful existing Helix motion/selection capabilities, not merely parity with the commands already remembered during Enzyme design. Alignment, selection-direction flipping, rotating selection contents, syntax-tree selection growth/shrinkage, and other Helix primitives must be included in the upcoming parity audit.

A capability may keep a sensible existing convention or receive Enzyme grammar according to the established rules. It should not be silently dropped because it is uncommon.

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
12. Contextual letter reuse is acceptable when grammatical position makes the branch clear, but a completed command may not simultaneously be an ambiguous namespace (`sp` pair vs `sp...` primary currently needs redesign).
13. Prefer the strongest mnemonic for common/core operations; rarer advanced families may accept a weaker documented mnemonic when necessary (`ss = section`, `sx = split`).
14. Primer must account for existing Helix motion/selection capabilities through an explicit parity audit before Enzyme v1 is declared complete.
15. Do not invent bindings merely to make the grammar exhaustive; preserve sensible conventions when they already satisfy Primer's philosophy.

## Selection v1 remaining decisions

`se...` is intentionally removed. `v` plus compatible motion is the extension model.

`so...` is intentionally removed. Selecting an object directly is the whole/around form; `si...` is the inside modifier.

Before Enzyme v1 is frozen, Primer still needs:

- a new spelling for the primary-selection operations previously proposed under `sp...`, because `sp` now means select pair;
- implementation verification for `sp` / `sip` using true matching-pair semantics;
- implementation verification for `sb` / `sib` using Tree-sitter-defined block semantics;
- Backspace transient-selection cancellation/restoration;
- a **Helix parity audit** covering every normal movement/motion and corresponding Select/Visual extension behavior;
- syntax-tree selection growth/shrink and sibling/parent/child traversal;
- character find/till and repeat-last-motion behavior;
- page/half-page and viewport-relative motions;
- jumplist/history motions and their relationship to `gnh/gNh`;
- advanced multiple-selection operations such as align, flip direction, and rotate contents;
- regex selection/filter/split behavior and its mapping to `sxm`, `skm`, and `srm`;
- commands whose Helix behavior cannot sensibly map to a contiguous selection across buffers/documents.

After that audit, only actual uncovered capabilities should receive new grammar decisions.
