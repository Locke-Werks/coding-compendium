---
id: php
title: PHP
type: language
verified: 2026-08-02
volatility: low
verify: php --version

name: PHP
aka: [php8, php7, hypertext preprocessor]
family: interpreted
likelihood: possible
extensions: ['.php', '.phtml', '.blade.php']

# PHP is the easiest language in the deck to identify and the hardest to date.
# The tells split into two groups: `<?php` and `$` say it is PHP, and `elseif`
# plus the backslash namespaces say which decade it was written in.
tells:
  - pattern: '<\?php'
    kind: regex
    weight: 10
    note: >
      The opening tag, usually the first five characters of the file. No other
      language on this deck has an opening tag at all. Seeing it ends the
      question immediately.
  - pattern: '\$\w+'
    kind: sigil
    weight: 8
    note: >
      Every variable starts with a dollar sign, including plain local ones.
      PowerShell does this too, but PowerShell has no semicolons at the ends of
      lines. Python, Ruby, and JavaScript never mark variables at all.
  - pattern: 'elseif'
    kind: token
    weight: 8
    note: >
      One word, no space. Python writes `elif`, Ruby writes `elsif`, JavaScript
      writes `else if`. Lua also uses `elseif`, but Lua has no `$` on its
      variables and closes blocks with `end`.
  - pattern: 'use \w+\\'
    kind: regex
    weight: 9
    note: >
      A backslash separating namespace parts, as in `use App\Models\User;`, is
      PHP alone. Java and Python separate with a dot, Rust and C++ with `::`.
  - pattern: 'public function'
    kind: regex
    weight: 8
    note: >
      PHP puts the visibility word before `function` and never states a return
      type there. Java and C# write `public void name()` with the type in the
      middle. JavaScript has no `public` keyword in a plain function.
  - pattern: 'echo'
    kind: token
    weight: 7
    note: >
      PHP prints with `echo` and no parentheses. Bash also has `echo`, but a Bash
      line does not end in a semicolon and its variables are bare on assignment.
  - pattern: '->'
    kind: operator
    weight: 6
    note: >
      Member access, as in `$user->name`. Rust puts `->` before a return type and
      C++ uses it on pointers. In PHP it always follows a `$` variable.
  - pattern: '::'
    kind: operator
    weight: 4
    note: >
      Static access, as in `User::find(1)`. Rust and C++ use `::` for module and
      namespace paths instead, and there is no `$` anywhere near theirs.

rules_out:
  - pattern: 'def'
    because: Python or Ruby.
  - pattern: 'console\.log'
    kind: regex
    because: JavaScript or TypeScript.
  - pattern: '^\s*end\s*$'
    kind: regex
    because: Ruby, Lua, or Elixir. PHP closes with a brace.
  - pattern: 'let '
    kind: regex
    because: JavaScript, TypeScript, or Rust. PHP has `const` but never `let`.
  - pattern: '^import '
    kind: line_start
    because: Python, JavaScript, or Java. PHP says `use` for namespaces and `require` for files.
  - pattern: 'elif'
    because: Python or Bash. PHP writes `elseif`.
  - pattern: '#include'
    kind: line_start
    because: C or C++.

project_fingerprint:
  manifests:
    - file: composer.json
      decisive: true
      note: >
        Composer is PHP's package manager and this is its manifest. The file is
        JSON (JavaScript Object Notation), which is why it looks like a Node
        project at a glance. The giveaway is inside: `require` keys naming
        vendor and package pairs like `laravel/framework`, and a `psr-4` block
        under `autoload`.
    - file: artisan
      decisive: true
      note: >
        A file called `artisan` with no extension at the repo root means Laravel,
        which means PHP. It is a PHP script; the missing extension is a habit
        borrowed from Unix command names.
    - file: wp-config.php
      decisive: true
      note: >
        WordPress. It holds database credentials in plain text, so it belongs in
        .gitignore and never in a commit.
    - file: index.php
      decisive: true
      note: >
        The traditional entry point. In an old-style project there is no router
        at all: the web address maps straight onto the folder path, and every
        page is its own `.php` file.
  lockfiles: [composer.lock]
  build_dirs: ['vendor/', 'bootstrap/cache/', 'storage/framework/']
  entry_points: [index.php, 'public/index.php', artisan]

shape:
  blocks: braces
  statement_end: semicolon
  comment_line: '// for a line. # also works and marks older code.'
  comment_block: '/* */'
  string_quotes: >
    Double quotes substitute `$variables` inside them. Single quotes do not and
    are faster to read for that reason. Heredoc syntax with `<<<EOT` handles long
    blocks.
  naming: camelCase for methods, PascalCase for classes, SCREAMING_SNAKE_CASE for constants, and $snake_case variables in anything written before about 2015
  import_keyword: use for namespaces, require and include for files

tooling:
  package_manager: Composer
  registry: Packagist
  runtime: the php binary, usually sitting behind a web server such as Apache or nginx
  install_command: composer require <vendor>/<package>
  run_command: php <file>.php, or php -S localhost:8000 for a throwaway local server
  test_command: ./vendor/bin/phpunit

confusable_with:
  - language: javascript
    settle_it: >
      Both use braces, semicolons, and the word `function`, and both throw a `$`
      around. PHP marks every variable with `$` and opens the file with `<?php`.
      JavaScript marks nothing, declares with `const` or `let`, and reaches into
      objects with a dot where PHP uses `->`.
    tiebreak: { pattern: '<\?php', kind: regex, favors: php }
  - language: powershell
    settle_it: >
      Both write `$name = "value"`. PHP ends the statement with a semicolon and
      lives inside `<?php`. PowerShell ends at the newline and its commands are
      hyphenated verb-noun pairs like `Get-ChildItem`.
    tiebreak: { pattern: '\b(Get|Set|New|Remove)-\w+', kind: regex, favors: powershell }
  - language: html
    settle_it: >
      A `.php` file is often mostly tags with small islands of code in it, so it
      reads as a web page. HTML on its own can never contain `<?php`. If the tag
      is anywhere in the file, the file is PHP.
    tiebreak: { pattern: '<\?php', kind: regex, favors: php }

errors_look_like:
  sample: |
    PHP Fatal error:  Uncaught TypeError: Unsupported operand types: string + int
    in C:\Users\<yourname>\site\app.php:12
    Stack trace:
    #0 C:\Users\<yourname>\site\app.php(20): total()
    #1 {main}
      thrown in C:\Users\<yourname>\site\app.php on line 12
  recognize_by: >
    The line opens with the literal words `PHP Fatal error:`, `PHP Warning:`, or
    `PHP Parse error:`. Underneath it, a `Stack trace:` header and frames
    numbered `#0`, `#1`, ending with `{main}`. The location is appended to the
    message as `in <path>:<line>` and then repeated at the bottom as `thrown in
    ... on line 12`. Python numbers nothing and Ruby numbers nothing; the `#0`
    frames are PHP alone.
  patterns:
    - '^PHP (Fatal error|Warning|Notice|Parse error|Deprecated):'
    - '^#\d+ .*\(\d+\):'
    - '^Stack trace:'

meet_it_when: >
  Someone asks you to fix a site that already exists. WordPress runs a large
  share of the web and is written in PHP, so any theme or plugin edit lands you
  here. You also meet it on cheap shared hosting, in a Laravel project someone
  started, and in a folder full of `.php` files with no build step anywhere.

what_agents_get_wrong: >
  The dangerous failure is old code, because PHP has more bad examples in public
  than any other language and an agent has read all of them. Watch for `mysql_`
  function names such as `mysql_query` and `mysql_connect`: those were removed in
  PHP 7, so they no longer run, and the reason they were removed is that they
  invite you to build a query by gluing strings together. Anything shaped like
  `"SELECT * FROM users WHERE id = " . $_GET['id']` is a SQL injection hole, and
  the fix is a prepared statement through PDO (PHP Data Objects) with the value
  passed as a parameter instead of pasted into the text. The second habit is
  echoing user input straight into the page without `htmlspecialchars`, which
  lets someone else's script run in your visitors' browsers. Third, `==` in PHP
  compares loosely and converts types while it does so, which produces true for
  pairs that are not equal in any human sense; generated code should use `===`.
  Fourth, agents write PHP 5 idioms out of habit: `array()` instead of `[]`, no
  type declarations on parameters, no `declare(strict_types=1);` at the top. None
  of those break, they just date the file the moment it is written.

version_landscape: >
  PHP is the deck's worst case for stale answers. PHP 5 to PHP 7 in 2015 removed
  whole families of functions, and PHP 8 in 2020 turned many quiet warnings into
  real errors and added typed properties and named arguments. Code you find from
  2012 will use `mysql_query`, which has not existed for a decade. Treat `mysql_`
  as a date stamp: if a snippet contains it, everything else in that snippet is
  from the same era and should be thrown out with it. Check what you actually
  have with `php --version` before copying anything.

see_also:
  - javascript
  - html
  - sql
  - powershell
  - g2-package-managers
  - g6-secrets-and-what-never-to-commit
  - j1-how-to-recognize-a-language

keywords: [composer, packagist, laravel, wordpress, wp, symfony, phpunit, xampp, blade, artisan]
---

PHP (PHP: Hypertext Preprocessor) is the language most of the older web is
written in. The name contains itself, which tells you roughly which decade it
came from.

It is a template language that grew into a general one. That history explains its
shape: a `.php` file is a web page with code mixed into it, and everything
outside the code tags gets sent to the browser untouched.

## The shape

Blocks use curly braces. Statements end with a semicolon. Indentation means
nothing to the interpreter, unlike Python, where it means everything.

Every variable starts with `$`, including ordinary local ones. That single
character is the fastest way to tell PHP from JavaScript, which uses the same
braces and the same word `function` and marks nothing.

```php
<?php
$count = 3;                      // the $ is part of the variable name
const MAX_ITEMS = 100;

function add(int $a, int $b): int {
    return $a + $b;              // types are optional and worth having
}

if ($count > 2) {
    echo "$count is plenty";     // double quotes substitute the variable
} elseif ($count === 2) {        // one word, and three equals signs
    echo "two";
} else {
    echo "not enough";
}
```

Comments are `//` for a line and `/* */` for a block. A `#` comment also works
and usually means the file is old.

Member access is `->`, so `$user->name` reads a property and `$user->save()`
calls a method. Static access is `::`. Both are strong tells when they appear
next to a `$`.

## Six lines of it

```php
<?php
$scores = ['ada' => 10];

foreach ($scores as $name => $points) {
    echo "$name: $points\n";
}

echo json_encode($scores);
```

The `=>` inside the brackets pairs a key with a value. JavaScript uses the same
two characters for an arrow function, which is a different thing entirely.

## What it is for

Websites, and almost nothing else. WordPress, Laravel, Drupal, and an enormous
volume of hand-written pages from before frameworks were normal.

A PHP file runs on the server every time someone loads the page, and what reaches
the browser is the HTML (HyperText Markup Language) it printed. There is no build
step to run and no bundle to ship, which is why it took over cheap hosting and
why it is still there.

## Reading its errors

Errors arrive with the word PHP in front of them.

```text
PHP Fatal error:  Uncaught TypeError: Unsupported operand types: string + int
in C:\Users\<yourname>\site\app.php:12
Stack trace:
#0 C:\Users\<yourname>\site\app.php(20): total()
#1 {main}
```

Read the first line for what broke, then the path and line number that follow the
word `in`. The numbered frames underneath are who called whom, newest first, and
`{main}` is the bottom of the stack.

One warning about the web version. If a page loads blank with no message at all,
the error went to a log file rather than the screen, because a production server
hides them on purpose. See [f4-logs](#f4-logs) for where to look.
