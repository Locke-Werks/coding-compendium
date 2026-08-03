---
id: j7-the-ones-you-will-not-meet
title: The ones you will not meet
type: section
track: J
order: 70
verified: 2026-08-02
volatility: low
answer: >
  Fortran, COBOL, Perl, and Visual Basic are all still running somewhere
  important, and none of them will ever be on your screen. Recognize them, note
  that the code is older than the problem you came to solve, and move on.
owns:
  - the legacy language sidebar
see_also:
  - j1-how-to-recognize-a-language
  - j4-reading-a-repo-you-did-not-write
  - c2-compiled-vs-interpreted
keywords:
  - fortran
  - cobol
  - perl
  - visual basic
  - legacy code
  - old languages
  - mainframe
---

## More

Four languages you will hear named, will never be asked to write, and should still be able
to recognize. They are all older than most of the people maintaining them, and every one of
them is holding up something you use.

**Fortran** does the heavy arithmetic. Weather models, climate simulations, physics. The
numerical libraries underneath Python's scientific stack are Fortran, and nobody has
replaced them because nobody has beaten them.

**COBOL (Common Business-Oriented Language)** runs money. Banks, insurers, payroll,
government benefit systems. Estimates of how many billions of lines are still in production
vary wildly, which tells you something about how well anyone has counted.

**Perl** was the glue of the early internet and the tool of choice for anything involving
text. It has been quietly replaced by Python nearly everywhere, and the parts that were not
replaced are still running.

**Visual Basic** put a button on a form and let a person double-click it to write the code
behind it. Its descendant lives inside Microsoft Office as the macro language, which means
it is probably running in your accounting department right now.

The reason none of these will be on your screen: no agent will choose one for a new project,
no tutorial you follow will use one, and the jobs that involve them are held by people who
have held them for thirty years. You would meet one only by inheriting it, and inheriting
one is a career decision rather than an afternoon.

Recognizing them is still worth two minutes, because a file you cannot identify is a file
you cannot reason about.

## Full

### Fortran

Born 1957, and the first programming language anyone would recognize as one today. It exists
to do arithmetic on large arrays of numbers extremely fast, and after nearly seventy years
of competition it is still the best at exactly that.

```fortran
do i = 1, n
   y(i) = a * x(i) + y(i)
end do
```

Recognize it by `end do`, `end if`, and `end program`, by variable names in capitals in
older code, and by `.eq.` and `.lt.` where other languages write `==` and `<`.

A fair amount of the machine learning boom runs on matrix routines written before the moon
landing, and every attempt to replace them has produced something slower.

### COBOL

Born 1959, designed so that a business manager could read the source code without being a
programmer. It succeeded at being readable and failed at being read.

```cobol
MULTIPLY HOURS-WORKED BY HOURLY-RATE GIVING GROSS-PAY.
```

Recognize it by full English sentences in capitals, by statements ending in a period, and by
hyphens inside every name. Nothing else looks remotely like this.

It survives because rewriting the system that moves your paycheck is a project with no upside
and one very visible failure mode. When several governments needed emergency changes to
benefits systems in 2020, the public request that went out was for retired programmers.

### Perl

Born 1987, and for fifteen years the fastest way to get anything done with text. It gave the
early web its first generation of interactive pages and it gave system administrators a tool
that could do in one line what took a page of anything else.

```perl
@lines = grep { /error/i } <STDIN>;
```

Recognize it by the sigils: `$` for one thing, `@` for a list, `%` for a lookup table, and
regular expressions written directly into the syntax with no ceremony.

The line about it that has never been improved on: Perl is the only language that looks the
same before and after encryption.

### Visual Basic

Born 1991, and the reason a very large number of people found out they could program at all.
You dragged a button onto a window, double-clicked it, and typed what should happen. Nothing
before it was that direct and very little since has been either.

```vbnet
Private Sub Button1_Click()
    MsgBox "Hello, " & TextBox1.Text
End Sub
```

Recognize it by `Sub`, `End Sub`, `Dim`, and by the whole thing reading like slightly stiff
English with capitalized keywords.

Its revenge on the industry that abandoned it is Visual Basic for Applications, the macro
language inside Excel, which means a meaningful share of the world's financial modeling runs
on a language Microsoft stopped promoting a quarter of a century ago, in files that nobody
has ever put in version control.

### What to do if you actually meet one

Paste it into the identify box like anything else. Then stop and ask a different question
from the usual one: not how to change this, but who owns it and what happens if it stops.
Code in these four languages is normally load-bearing, normally undocumented, and normally
older than the process that depends on it.

If the answer is that you own it now, the first move is the same as with any inherited
repository ([j4](#j4-reading-a-repo-you-did-not-write)): find out how it is run, how it is
tested, and how it is deployed, before touching a single line. The second move is to write
down what you learn, because whoever comes after you will be starting from exactly where you
started.
