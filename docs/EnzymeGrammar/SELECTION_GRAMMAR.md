# Primer Text Editor — Enzyme Selection Grammar

> **Status:** Proposed v1 — ready for parity audit/dogfooding
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

## Core object vocabulary

| Key | Object | Notes |
|---|---|---|
| `l` | line | Whole current line. |
| `w` | word | Word under/at cursor. |
| `f` | function/method | Language-neutral callable concept. |
| `c` | class/type | Structural type/class declaration. |
| `b` | structural block | Syntax-aware block. |
| `s` | section | Paragraph/logical text section containing the cursor. |
| `p` | pair | Paired delimiters/syntax object when used in inside/outside grammar. |
| `d` | document | Whole document. |

Structural objects should use Tree-sitter where practical. Section is Primer's user-facing term for the paragraph-like text unit traditionally called a paragraph by modal editors.

## Direct object selection

| Key | Meaning |
|---|---|
| `sl` | select line |
| `sw` | select word |
| `sf` | select current function/method |
| `sc` | select current class/type |
| `sb` | select current structural block |
| `ss` | select current section |
| `sd` | select whole document |

These acquire the containing/current object directly. They are for the thought "select this thing," rather than "start selecting while I move."

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

`Esc` exits/cancels back toward the safe normal state according to Primer's general mode rules.

## Inside / outside relationships

For paired or nested syntax, explicit object selection uses a relationship before the object:

```text
s i [object]    select inside object
s o [object]    select outside/around object
```

Initial forms:

| Key | Meaning |
|---|---|
| `sip` | select inside pair |
| `sop` | select outside/around pair |

The same relationship may be reused for syntax objects where Tree-sitter can define reliable inside/around ranges. Primer should not advertise theoretical combinations with inconsistent semantics.

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
b    block
s    section
d    document
i    inside…
o    outside/around…
a    add selection…
r    remove selection…
p    primary selection…
k    keep/filter selections…
x    split selection…
```

Select/Visual mode should instead surface compatible motions/navigation because `v` already supplies the Select/Extend intent.

## Design constraints

1. `s` means deliberate object-selection grammar.
2. `v` preserves interactive Select/Visual mode.
3. In Select/Visual mode, compatible navigation extends the selection; do not require a redundant `se` prefix.
4. Preserve Helix's first-class and multiple-selection model.
5. Reuse Navigation vocabulary rather than creating a parallel extension vocabulary.
6. Use Tree-sitter for structural selections where practical.
7. Relationships such as inside/outside precede the object: `sip`, `sop`.
8. `n/N` retain next/previous meaning inside selection-set operations.
9. Contextual letter reuse is acceptable when grammatical position makes the branch clear.
10. Prefer the strongest mnemonic for common/core operations; rarer advanced families may accept a weaker documented mnemonic when necessary (`ss = section`, `sx = split`).
11. Primer must account for existing Helix motion/selection capabilities through an explicit parity audit before Enzyme v1 is declared complete.
12. Do not invent bindings merely to make the grammar exhaustive; preserve sensible conventions when they already satisfy Primer's philosophy.

## Selection v1 remaining decisions

`se...` is no longer unresolved; it is intentionally removed. `v` plus compatible motion is the extension model.

Before Enzyme v1 is frozen, Primer still needs a **Helix parity audit** covering:

- every normal movement/motion and whether it is already represented by Enzyme;
- the corresponding Select/Visual extension behavior for compatible motions;
- syntax-tree selection growth/shrink and sibling/parent/child traversal;
- character find/till and repeat-last-motion behavior;
- page/half-page and viewport-relative motions;
- jumplist/history motions and their relationship to `gnh/gNh`;
- advanced multiple-selection operations such as align, flip direction, and rotate contents;
- regex selection/filter/split behavior and its mapping to `sxm`, `skm`, and `srm`;
- commands whose Helix behavior cannot sensibly map to a contiguous selection across buffers/documents.

After that audit, only actual uncovered capabilities should receive new grammar decisions.
