# Rlisp
## A simple interpreted Lisp built in Rust

I'm building this as an educational project in my free time. It's in
its incredibly early stages right now, but it runs and several
functions and special forms work.

If you want to give it a try, just run it with `cargo run`. I use
Rust nightly out of habit, and I really have no clue whether or not
this will compile on stable.
 
### Syntax

Rlisp reads a lot like any other Lisp-1: lists are
whitespace-separated elements wrapped in parentheses, strings are
wrapped in double-quotes (`"`) (Rlisp recognizes the escape sequences
`\t` and `\n` to create tabs and newlines, plus `\"` for the character
`"`), numbers are numbers, and symbols are anything else. Semicolons
`;` start comments, and anything from the semicolon to the end of a
line is ignored.

As a note, Rlisp **is case-sensitive**. `foo` and `FOO` and `Foo` and `fOO` are all distinct symbols.

Rlisp has a boolean type, and the symbols `t` and `nil` evaluate
to `true` and `false`. In situations where a boolean value is
expected, any non-`nil` value is treated as `true`.

Rlisp's numbers are all 64-bit floats. During read, any token that
doesn't start with `(` or `"` is treated as potentially being a
number, and if Rust's `f64::from_str` errors, it is used as a symbol
instead. I'm not entirely sure what `f64::from_str` allows and doesn't
allow, but I know that: `1+` is a symbol, `+1` is a number, and in
every case I have tested it works the way I expect.

### Variables and binding

NOTE: local bindings (ie `let` and function calls) **are visible to
functions called during their scopes**.

    (defvar 1+ (n) (+ n 1))
    (defvar 1- (n) (- n 1))
    
    (defvar inefficent-add (n m)
        (cond ((= m 0) n)
              (t (inefficent-add (1+ n) (1- m)))))
    
    (let ((1+ nil))
        (inefficent-add 5 5))

will cause an error because at the point of calling `inefficent-add`,
`1+` is not a function.

#### `defvar`

Global variables are declared with `defvar`. A call in the form of:

    (defvar SYMBOL VALUE)

will overwrite the binding of `SYMBOL` to the result of evaluating
`VALUE`. Overwriting a symbol inside of a local binding, such as a
function call or a `let` body, is undefined behavior; `defvar`s should
only occur at the top level.

#### `let`

Local bindings are created either within function calls for the arguments or with the special form `let`. A call in the form of:

    (let ((SYMBOL1 VALUE1)
          (SYMBOL2 VALUE2)
          ...)
       BODY1
       BODY2
       ...
       RETURN_VALUE)
       
will locally bind `SYMBOL1` and `SYMBOL2` to the result of evaluating
`VALUE1` and `VALUE2`, evaluate each body clause in order, and return
the result of evaluating the last body clause.

#### `setq`

Assignment is done with the special form `setq`. A call in the form of:

    (setq SYMBOL1 VALUE1
          SYMBOL2 VALUE2
          ...)

will set each of the symbols to the result of evaluating the values passed.


#### `defun`

`defun` is used for function definition. A call in the form of:

    (defun SYMBOL (ARG1 ARG2 ...)
        BODY1
        BODY2
        ...
        RETURN_VALUE)

will create a function named `SYMBOL` which takes the args `(ARG1 ARG2
...)` evaluates all of its body clauses in order and returns the
result of the last, bind the value of `SYMBOL` to that function, and
then return the function. None of these are evaluated during the call to `defun`.

Rlisp supports both optional and rest arguments. Any arguments after
the symbol `&optional` are optional, and will be `nil` if not
supplied. The first argument after `&rest` will be a proper
`nil`-terminated list containing the rest of the arguments passed. Any
`&rest` arguments after the first, and any `&optional` arguments after
a `&rest` argument will be ignored. (note: some special forms
e.g. `setq` take multiple `&rest` arguments. Do not be fooled; regular
functions don't get to do that!)

### Current functions and special forms:

#### Special forms:

+ `cond`
+ `let`
+ `setq`
+ `quote`
+ `if`
+ `defun`
+ `defvar`

#### Functions defined in `builtins/mod.rs`:

+ `numberp`
+ `consp`
+ `cons`
+ `list`
+ `debug` - prints debug information on the object passed
+ `print` - pretty-prints all arguments passed to it

#### Functions defined in `math/mod.rs`:

+ `=`
+ `*`
+ `+`
+ `-`
+ `/`
+ `<`
+ `<=`
+ `rem`
+ `mod`
+ `trunc`
+ `floor`
+ `ceil`
+ `round`
+ `integerp`
