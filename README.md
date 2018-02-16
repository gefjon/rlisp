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

Rlisp is (I hope) lexically scoped.

    lisp> (defvar func (let ((x 3)) (lambda () x)))
    ...
    lisp> (func)

will print `3`, and

    lisp> (defvar x 3)
    ...
    lisp> (defvar func (let ((x 3)) (lambda () x)))
    ...
    lisp> (let ((x 4)) (func))
    
will also print `3`.

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


#### Namespaces

Namespaces are created with `make-namespace`. `make-namespace` takes
an optional argument `name`, which is the name of the
namespace. `name` is not evaluated. If `name` is a symbol,
`make-namespace` sets `name` in the global namespace to the newly
created namespace. `make-namespace` returns the created namespace, so
a function like:

    (defun returns-a-namespace ()
        (let ((my-namespace (make-namespace "I made this!")))
            (set my-namespace three 3)
            (set my-namespace foo 'bar)
            my-namespace))

will create a namespace containing the pairs `three` = `3` and `foo` =
`bar`.

Namespace symbols can be accessed using `get` and `set`, which have
the signatures

    (get NAMESPACE SYMBOL)
    (set NAMESPACE SYMBOL VALUE)

`NAMESPACE` and `VALUE` are evaluated but `SYMBOL` is not.

The global namespace can be accessed using `(global-namespace)`. Therefore,

    lisp> (get (global-namespace) x)

is the same as just doing

    lisp> x

and

    lisp> (set (global-namespace) x 12)

is the same as doing

    lisp> (setq x 3)
    
#### Errors

    (catch-error (something-that-may-fail)
      (error-name CATCH-FORM)
      (other-error-name CATCH-FORM)
      (t CATCH-FORM))
  
will try `(something-that-may-fail)` and if it is successful, return
its value. If it fails, `catch-error` will continue through its
`error-name`/`catch-form` pairs. It will evaluate the first one for
which `error-name` `eq`s the name of the error returned by
`(something-that-may-fail)` and evaluate and return the associated
`catch-form`. The `error-name` `t` will match any error, and if no
matching `error-name` is found, `catch-error` will return the error.

The names of errors that Rlisp will generate itself are
`wrong-type-error`, `wrong-arg-count-error`, `improper-list-error`,
`unbound-symbol-error` and `internal-error`. All of these except
`internal-error` can be created by a function of the same name defined
in `builtins/mod.rs`.

`(wrong-type-error WANTED FOUND)` takes two type descriptors, the
desired type and the type passed.

`(wrong-arg-count-error FOUND MIN &optional MAX)` takes 2-3 natnums,
the number of arguments passed, the minimum number desired, and
optionally the maximum number allowed.

`(improper-list-error)` takes no parameters and signals that an
improperly terminated list was found where a `nil`-terminated one was
expected.

`(unbound-symbol-error SYM)` takes a single argument, the name of a
symbol (which is evaluated and thus will be quoted in most cases)
which was found to be unbound when it should have had a value.

The general function `(error KIND &rest INFO)` creates an error with
the `error-name` `KIND`. I recommend using symbols for `KIND` rather
than strings, as `eq`-comparing strings is undefined behavior. Any objects can be `INFO`s, and currently they are just printed in the REPL and otherwise unused.

### Current functions and special forms:

#### Special forms:

+ `cond`
+ `let`
+ `setq`
+ `quote`
+ `if`
+ `defun`
+ `defvar`
+ `catch-error` - 
+ `lambda`
+ `check-type`
+ `get`
+ `set`
+ `make-namespace`

#### Functions defined in `builtins/mod.rs`:

+ `numberp` - the logical union of `integerp` and `floatp`
+ `integerp`
+ `floatp`
+ `consp`
+ `symbolp`
+ `stringp`
+ `functionp`
+ `boolp`
+ `cons`
+ `list`
+ `debug` - prints debug information on the object passed
+ `print` - pretty-prints all arguments passed to it
+ `eq` - pointer/numeric equality
+ `wrong-type-error`
+ `wrong-arg-count-error`
+ `improper-list-error`
+ `unbound-symbol-error`
+ `error`
+ `global-namespace`

#### Variables defined in `builtins/mod.rs`:

+ `+pi+`
+ `+e+`
+ `+sqrt2+`
+ `+ln2+`
+ `+ln10+`
+ `+log2-e+` - the log base 2 of e
+ `+lge+` - the log base 10 of e
+ `+1/pi+`
+ `+2/pi+`
+ `+pi/2+`
+ `+pi/3+`
+ `+pi/4+`
+ `+pi/6+`
+ `+pi/8+`
+ `+1/sqrt2+`
+ `+infinity+`
+ `+-infinity+`
+ `+epsilon+`
+ `+min-num+`
+ `+max-num+`
+ `+nan+`
+ `+min-integer+`
+ `+max-integer+`


#### Functions defined in `math/mod.rs`:

+ `=`
+ `*`
+ `+`
+ `-`
+ `/`
+ `<`
+ `<=`
+ `>`
+ `>=`
+ `rem`
+ `mod`
+ `trunc`
+ `floor`
+ `ceil`
+ `round`
+ `flatten` - tries to coerce floats into ints, but only if they already are ints at heart
