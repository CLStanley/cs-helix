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

Within directional grammar, lowercase `n` means **next** and uppercase `N` means **previous**. This deliberately mirrors the established search-repeat relationship (`n` next, `N` previous) and avoids consuming another object/purpose letter merely to express reverse direction.

Capitalization therefore has two legitimate Enzyme roles when the distinction is already useful and learnable: it may reverse an established directional pair (`n/N`), or distinguish an established object kind (`w/W`). Enzyme should not introduce capitalization arbitrarily, but it also should not reject a compact convention that removes grammar conflicts.

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
| `gsl` | start of current line content (first non-whitespace character) |
| `g0` | absolute beginning of current line (column zero) |
| `gel` | end of current line |
| `gl<number>` | go to absolute line number, e.g. `gl225` |
| `gsd` | start of document |
| `gmd` | middle of document |
| `ged` | end of document |
| `gsw` / `gew` | start / end word boundary |
| `gsW` / `geW` | start / end long-word boundary |
| `gsf` / `gef` | start / end of current function or method |
| `gnf` / `gNf` | next / previous function or method |
| `gsc` / `gec` | start / end of current class/type |
| `gnc` / `gNc` | next / previous class/type |
| `gsb` / `geb` | start / end of current structural block |
| `gnb` / `gNb` | next / previous structural block |
| `gss` / `ges` | start / end of current section |
| `gns` / `gNs` | next / previous section |

A **section** is Primer's user-facing term for the paragraph-like logical text unit traditionally called a paragraph by modal editors.

A **block** is a semantic/syntactic object defined by the current language's Tree-sitter support. Primer must not redefine `block` as merely a brace pair. This means block behavior may legitimately differ between languages as their grammars differ.

Matching **pairs** are a separate, deliberately simpler concept. Pair operations provide a predictable fallback when working in an unfamiliar language: even before the user understands what that language considers a structural block, visually recognizable delimiters can still be addressed through pair navigation/selection.

`gmd` is an absolute document destination. Viewport-relative movement belongs to View mode instead.

### Literal line beginning and line-number input

`gsl` means the start of the **content** of the current line. On an indented line it lands on the first non-whitespace character, which is normally the useful editing destination.

`g0` is the explicit escape hatch for the literal beginning of the line at column zero. This keeps indentation-aware navigation ergonomic without making the absolute boundary inaccessible.

`gl<number>` takes an absolute line number as an argument:

```text
gl1      go to line 1
gl50     go to line 50
gl225    go to line 225
```

The number is an argument, not a repeat count. `gl` should therefore enter a small line-number input state rather than interpreting the digits through Enzyme's general count grammar. The helper/status area should expose the pending line-number input; `Enter` commits it, `Esc` cancels it, and Backspace edits/cancels the pending input sensibly.

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
gp     go to matching counterpart of the current recognized pair
gnp    go to next recognized pair
gNp    go to previous recognized pair
```

`gp` remains a complete, dedicated command for bouncing between the two sides of the currently addressed pair. It is not a directional prefix. The `n/N` directional grammar therefore leaves pair matching conflict-free: `gnp` reads “go next pair,” while `gNp` reads “go previous pair.”

Recognized pairs include syntax-aware braces, brackets, parentheses, quotations/apostrophes where parser context supports them, and other language-aware paired constructs that can be identified reliably.

`gnp` and `gNp` traverse recognized pair starts in deterministic document order; `gp` crosses the currently addressed pair. Primer should prefer a smaller trustworthy parser-aware pair set over false-positive punctuation heuristics.

Pair vocabulary is reused by Selection under the general whole/inside rule:

```text
sp     select surrounding pair including delimiters
sip    select inside surrounding pair excluding delimiters
```

There is no separate outside/around selection family: `sp` already means the whole pair.

## View mode

Primer preserves Helix's `z/Z` **View mode** rather than translating viewport operations into long `g...` sentences.

```text
z / Z    manipulate or inspect the viewport
```

Sensible existing Helix View-mode operations for scrolling, page/half-page movement, and aligning the current line top/center/bottom remain available. `Z` preserves Helix's sticky View-mode concept.

This is intentionally separate from absolute document destinations such as `gsd/gmd/ged`.

## Advanced syntax-tree motions

Primer preserves access to Helix's advanced syntax-tree parent/child/sibling navigation and syntax-aware selection operations for v1 rather than forcing rarely used capabilities into speculative Enzyme vocabulary.

Capability parity matters; renaming every obscure command does not.

> **Enzyme lowers the learning curve; it does not attempt to eliminate learning. Core, frequent operations should follow predictable grammar and be discoverable. Specialized or advanced operations may retain concise established Helix bindings when a grammatical replacement provides little practical benefit.**

The test for an advanced existing binding is not "Can Enzyme invent a sentence for this?" but "Would changing this materially lower the learning curve for normal editing?" If not, preserving the concise existing control is preferable.

Dogfooding Primer in Primer is the mechanism for deciding which preserved advanced operations deserve future Enzyme grammar.

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
v → gNf    extend toward previous function
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
l    line number…
0    column zero
p    matching pair
d    definition
D    declaration
i    implementation
t    type definition
```

Pair navigation should remain discoverable as:

```text
gp     matching pair
gnp    next pair
gNp    previous pair
```

Examples:

```text
gsf    go start function
gef    go end function
gnf    go next function
gNf    go previous function

gnc    go next class
gNc    go previous class
```

A user who learns the relationship vocabulary should be able to predict the class/block/section/pair equivalents.

## Counts and direction

Counts compose with the directional grammar without changing the meaning of the direction slot:

```text
g3nf    third next function
g2Nf    second previous function
g2nc    second next class
g2Nc    second previous class
```

The number is a repeat/count modifier. This is distinct from `gl225`, where `225` is the absolute line-number argument.

## Design constraints

1. Preserve sensible existing conventions instead of renaming them for novelty.
2. `g` means deliberate Go/navigation grammar.
3. `s/e` mean start/end; within directional grammar `n/N` mean next/previous.
4. `gp` remains the dedicated matching-pair command; it is not overloaded as a previous-direction prefix.
5. Capitalization may carry an established compact distinction such as `n/N` direction or `w/W` object kind, but should not be introduced arbitrarily.
6. Prefer Tree-sitter for structural objects and LSP for semantic destinations.
7. `block` is Tree-sitter/language-defined; matching `pair` is a separate fallback concept.
8. `z/Z` own viewport manipulation; `g` owns describable destinations.
9. Advanced Helix capabilities remain reachable even when they do not receive Enzyme-native names in v1.
10. Select/Visual mode reuses compatible navigation rather than duplicating motion vocabulary.
11. Enzyme lowers the learning curve rather than eliminating learning; specialized advanced controls may remain concise native Helix bindings when remapping them adds little practical discoverability.
12. Dogfooding Primer in Primer determines which preserved advanced operations deserve future Enzyme grammar.
13. `gsl` targets the first non-whitespace character; `g0` explicitly targets literal column zero.
14. `gl<number>` accepts an absolute line-number argument and is distinct from repeat/count syntax.
