# rpnc

reverse polish notation calculator, partly based on forth

## usage

if you already know how rpn works, you can skip to the "invoking rpnc" section.
if you don't, it's time for a maths lesson.

most people are familiar with infix notation, the "standard" way of notating maths.
`1 + 1` gives the result `2`. the operator goes in between the operands, hence
**in**fix. rpn is postfix, which means the operands go before the operator.
`1 1 +` gives 2. this maps better onto stack-based computing, because when you
see a number, you can push it onto the stack, and when you see an operator,
you can run it on the top elements of the stack. 

## invoking rpnc

`rpnc [OPTIONS] [FILE]`

when invoked with no arguments, rpnc will run in interactive mode. it prints a prompt
and takes input a line at a time. stack state is preserved between lines.
the program can be exited with ctrl-c, ctrl-d (eof) or "bye".

when invoked with the `-e LINE` option, rpnc will run the line given and exit. if `-r`
is also given, rpnc will not exit and instead go into interactive mode, preserving
the stack from the init line. `-r` also works for files.

files can be piped directly into rpnc or it can load them with the positionl arguments.
if the file argument is not given, stdin is used.

rpnc will execute the `~/.rpncrc` file on init, if it exists. this can be disabled with `-c`.

## operators

any decimal literal will push that number to the stack. hex and binary aren't supported.

- `+`: add the top two values on the stack (`1 2 +` gives 3)
- `-`: pop a and b, push b-a (`2 1 -` gives 1)
- `*`: pop a and b, push a*b
- `/`: pop a and b, push b/a (`4 2 /` gives 2)
- `**`: pop a and b, push b to the power of a (`8 2 **` gives 64)
- `%`: pop a and b, push b mod a
- `log`: pop a and b, push log a of b
- `ln`: pop x, push ln x (equivalent to `e log`)
- `sin`, `cos`, `tan`, `asin` etc: pop x, do the relevant trig function, push the result. `rpnc` uses radians, so watch out!
- `.`: print the top value of the stack, but don't remove it (unlike FORTH)
- `.stdf`: print the top value of the stack, in standard form
- `.s`: print the entire stack
- `swp`: swap the top two values of the stack
- `pop`: remove the top value of the stack
- `dup`: duplicate the top value of the stack
- `over`: duplicate the second value from the top (`a b -- a b a`)
- `rot`: swap the second and third values on the stack (`a b c -- b a c`)
- `pi`: push pi to the stack
- `e`: push e to the stack
- `dice`: pop x. push a random number in range [0, x)
- `bye`: exit rpnc (EOF or ctrl-d also works)  
this is where it gets real spicy. that's right, it's forth time  
- `:`: start "compiling" a user-defined word. the word directly after the `:` is the name, and all other words until the first `;` will be added to the definition
- `(` and `)`: any words following a `(` will be ignored until the next `)` (it's comments basically)

`rpnc` also ignores any line starting with `#`. hence, you can start rpnc files with `#!/bin/env rpnc` or whatever else and they can be executed right from your shell of choice. `-e` can (kinda) be used to give arguments in the form of starting stack values

yeah
