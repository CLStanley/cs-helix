# Primer Text Editor — Enzyme Navigation Grammar

> **Status:** Enzyme v1 implementation target
>
> **Parent specification:** `MODAL_GRAMMAR.md`

## Purpose

Navigation is Primer's language for moving through text and code. Preserve short conventions when they already make sense; use compositional `g = go` grammar for deliberate destinations.

```text
h  left
j  down
k  up
l  right

g  go / explicit navigation grammar
```

Enzyme should improve discoverability without turning movement into full sentences.

## Core rules

```text
g s [object]    go to start of current object
g e [object]    go to end of current object
g n [object]    go to next object
g N [object]    go to previous object
```

`n/N` retain their established next/previous relationship. This replaces the earlier middle-capital forms such as `gSf`/`gsF`; direction is expressed explicitly and consistently.

## Immediate and conventional navigation

| Key | Meaning |
|---|---|
| `h` | left |
| `j` | down |
| `k` | up |
| `l` | right |
| `w` | ordinary word fast path |
| `f<char>` | find next character |
| `F<char>` | find previous character |
| `t<char>` | till next character |
| `T<char>` | till previous character |
| `/` | search current document forward |
| `?` | search current document backward |
| `n` | next search result |
| `N` | previous search result |
| `Alt-.` | repeat last motion |

These remain because they are compact conventions and their behavior can be taught directly. Primer does not replace a sensible Helix convention merely to be different.

## Structural and document navigation

| Pattern | Meaning |
|---|---|
| `gsl` | start of current line |
| `gel` | end of current line |
| `gsd` | start of document |
| `gmd` | middle of document |
| `ged` | end of document |
| `gsw` / `gew` | start / end word boundary |
| `gsf` / `gef` | start / end of current function or method |
| `gnf` / `gNf` | next / previous function or method |
| `gsc` / `gec` | start / end of current class/type |
| `gnc` / `gNc` | next / previous class/type |
| `gsb` / `geb` | start / end of current structural block |
| `gnb` / `gNb` | next / previous structural block |
| `gss` / `ges` | start / end of current section |
| `gns` / `gNs` | next / previous section |

A **section** is Primer's user-facing term for the paragraph-like logical text unit traditionally called a paragraph by modal editors. Structural objects should use Tree-sitter where practical.

`gmd` is an absolute document destination. Viewport-relative movement belongs to View mode instead.

## Semantic/LSP destinations

```text
gd    go to definition
gD    go to declaration
gi    go to implementation
gt    go to type definition
```

These are short, mnemonic semantic destinations. Primer preserves strong Helix/Vim-family conventions where they already communicate intent well.

## Diagnostics and history

```text
gnd    next diagnostic
gNd    previous diagnostic

gnm    next editor modification/change
gNm    previous editor modification/change

gnh    next/forward location in navigation history
gNh    previous/back location in navigation history
```

Repository/VCS changes remain in the Leader/Git family rather than being conflated with editor modification history.

## Pair navigation

```text
gp    go to matching counterpart of current recognized pair
gP    go to next recognized pair
```

Recognized pairs include syntax-aware braces, brackets, parentheses, quotations/apostrophes where parser context supports them, and other language-aware paired constructs that can be identified reliably.

`gP` advances through recognized pair starts in deterministic document order; `gp` crosses the addressed pair. Primer should prefer a smaller trustworthy parser-aware pair set over false-positive punctuation heuristics.

Pair vocabulary is reused by Selection:

```text
sip    select inside pair
sop    select outside/around pair
```

## View mode

Primer preserves Helix's `z/Z` **View mode** rather than translating viewport operations into long `g...` sentences.

```text
z / Z    manipulate or inspect the viewport
```

Sensible existing Helix View-mode operations for scrolling, page/half-page movement, and aligning the current line top/center/bottom remain available. `Z` preserves Helix's sticky View-mode concept.

This is intentionally separate from absolute document destinations such as `gsd/gmd/ged`.

## Advanced syntax-tree motions

Primer preserves access to Helix's advanced syntax-tree parent/child/sibling navigation and syntax-aware selection operations for v1 rather than forcing rarely used capabilities into speculative Enzyme vocabulary.

Capability parity matters; renaming every obscure command does not. If these operations become common while developing Primer in Primer, they can receive Enzyme-native grammar later.

## Interactive visible-target Jump

```text
Space j    interactive visible-target Jump
```

Jump is an editor feature rather than ordinary `g` navigation. It enters a transient labelled-target state over useful visible destinations; typing a displayed label moves there and `Esc` cancels.

## Select/Visual interaction

`v` enters Select/Visual mode. Compatible navigation keeps the same spelling and extends the selection rather than merely moving:

```text
v → l      extend right
v → w      extend by word
v → gsf    extend toward start of current function
v → gnf    extend toward next function
```

Cross-buffer semantic jumps need not pretend to create impossible contiguous selections; preserve sensible Helix behavior in those cases.

## Teaching hierarchy

After `g`, the helper should expose:

```text
GO…

s    start of…
e    end of…
m    middle of…
n    next…
N    previous…
d    definition
D    declaration
i    implementation
t    type definition
p    matching pair
P    next pair
```

Examples:

```text
gsf    go start function
gef    go end function
gnf    go next function
gNf    go previous function
```

A user who learns the relationship vocabulary should be able to predict the class/block/section equivalents.

## Design constraints

1. Preserve sensible existing conventions instead of renaming them for novelty.
2. `g` means deliberate Go/navigation grammar.
3. `s/e` mean start/end; `n/N` mean next/previous.
4. Do not encode previous/current/next by changing capitalization in the middle of a sentence.
5. Prefer Tree-sitter for structural objects and LSP for semantic destinations.
6. Grammatical position may disambiguate reused letters.
7. `z/Z` own viewport manipulation; `g` owns describable destinations.
8. Advanced Helix capabilities remain reachable even when they do not receive Enzyme-native names in v1.
9. Select/Visual mode reuses compatible navigation rather than duplicating motion vocabulary.
10. Dogfooding Primer in Primer is the mechanism for deciding which preserved advanced operations deserve future Enzyme grammar.
