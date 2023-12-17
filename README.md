# Basis (math language)
A math console language with a bunch of useful functions and constants

## Interpreter Progress

![image](https://github.com/JakeRoggenbuck/basis/assets/35516367/3599b6c6-31d0-42f9-9c5f-a9293093f091)

## Syntax

Note: The `>` at the start of the line is the prompt. This prompt will evaluate the line and then display the value of the `console` attribute.

### Expressions

Basis supports simple expressions like addition through the summation function (See more about functions below), which takes exactly two arguments on the left.
```
> 1 1 +
int: 2
```

Note: `1 1 +` is called an anonymous expression because it is not given a name.

Basis supports fraction representation as the default for rational numbers. Rationals stay as fractions until you convert to a float (See more below).
```
> 2 3 /
ratio: 2 / 3
```

Basis allows you to convert fractions to decimal with the decimal `.` function which operates on one rational to the left.
```
> 2 5 / .
dec: 0.4
```

The precision of rations can be specified, they default to 32 bit max, but can be set with the tilde function. The shorthand `prec` will show the precision of the ratio in the console.
```
> 1 3 / ~
ratio (prec 1): 1 / 3

> 1 3 / ~ .
dec: 0.3
```

### Variables

#### Number types
There are many types along with the `number` type which cannot be instantiated but which gets derived from by every type listed below called the "number types"
- int
- ratio
- real
- dec
- complex
- imaginary
- size

```
> a int 5 =
a = int: 5

> b ratio 3 4 / =
b = ratio: 3 / 4
```

Zero in any `number` `type`, when cast to a `bool` is `false`

#### Methods on number types
Every number type, has a method to turn each number type into any other number type. If this method is called, is saves the value of the newly casted value into it's own data so that it can be called again with no expense. This is a speed vs memory tradeoff.

#### Literal
Literal can wrap the `number` `type` and other types. It essentially is like a normal variable, but instead of being included in the expression execution, it stays a literal.

```
> mynum literal 3 4 / =
literal: mynum

> mynum unwrap
ratio: 3 / 4
```

Why have a literal like this? Sometimes you may want to apply functions to an expression but not get a specific decimal or other numeric output, you may want another expression with the literals still persisting. Say if you wanted to take an expression `0 cos` and multiply it by `pi`, instead of getting `3.14...` you would get `pi` (because `0 cos` is `1`), the literal is not evaluated until you unwrap it by doing `0 cos pi * unwrap` to get some decimal representation of `pi`.

#### Strings
```
> "hello"
string: "hello" -> "size: 6"
```

#### Size
The `size` type is a special `type` around the `real` type that denotes that it is a `size` of another `type`
```
> 4 size
size: 4
```

Why is the size in double quotes? The `tuple` syntax and it's `console` attribute has its `size` in the `tuple` parentheses, so it seems only natural that the `console` attribute for `string` would also be have its size in the characters that surround its instantiation.

#### Late initialization
```
> a int =
a = late: int
```

#### The late type
The `late` `type` is a wrapper around the `option` `type`, making any late initialization not of the `type` of the vairable, but of `type` `late`, which is just a wrapper for the `option` `type`, so late initialized types are all basically options (but technically of `type` `late`).

After adding a value to a variable defined as a `late` `type`, it just becomes the `type` that `late` was given as a `genetic`.
E.g. `a int =` is a `late` type passed `int` as a generic.

### Provided Constants (of type literal)
A few constants are provided such as g (Gravity of Earth), e (Euler's number), pi (Pi), h (Planck constant)
```
> e
literal: e

> pi
literal: pi

> pi 0 ~
literal (prec 0): 3

> pi 0 ~ .
dec: 3

> pi 1 ~ .
dec: 3.1

> pi desc
"Ratio of a circle's circumference to it's radius"

> e .
dec: 2.718281828459045
```

#### Bool
The `bool` type is either `true` or `false`

```
> a bool true =
a = bool: true
```

#### Nil type
The `nil` `type` is a `type` of `type` `nil`

When cast to a `bool`, it becomes `false` later referred to as a `fasle` `type`

### More math functions

#### Multiplication
Used with `*`.

Multiplication can be applied to a `vector` as a scalar)

### I/O functions

#### print
The `print` function operated on the `type` `type`

```
> a int 1 =
a = int: 1
> a print
1
```

#### input
```
> a int =
a = nil

> a input =
a = <value provided by input>

~ or more concisely ~

> a int input =
```

### Comments
Comments are done like the following with a space before and after any text inside the `comment` as an homage to [Cruz Lang](https://github.com/JakeRoggenbuck/cruz-lang) and later [Planck Lang](https://github.com/plancklang/) and [Z-Flat](https://github.com/adamhutchings/zflat) along with many more.

```
> ~ hello world ~
comment: ~ hello world ~
```
Comments are the `comment` `type` and can be converted into the `string` `type`

### Functions

Functions in basis can be created with the following syntax
```
> f (x int) =: x 1 +
function: f (int) -> int
> 1 f
2
```

### Option
The `option` `type` is a wrapper for any `type` which has an attribute `exists` which is a `bool`.

There are two methods specific to `option`, `some` and `none` inspired by Rust. The `some` method is equivalent to comparing the `exists` attribute to `true` and the `none` method is evaluated to comparing the `exists` to `false`.

The `option` `type` has an attribute called `value` which is of `type` `type` and is defined generically as seen in the section about generic types.

The `option` `type` will return the `exists` `bool` when cast into a `bool`

### Closures

The `function` syntax may seem weird at first, but when given the closure syntax, it makes slightly more sense.

#### Single line closure
The following is a new closure (scope) that is evaluated as an expression

```
> : 1 1 +
closure: 1 1 +

> : 1 1 + unwrap
int: 2
```

#### Multi-line closure
The following is a new multi-line closure (scope) that is evaluated as an expression as well
```
> {
    1 1 + }
closure: 1 1 +

> {
    1 1 + } unwrap
int: 2
```

### The unwrap function
Many types are wrappers, like literal, closure, option. The unwrap function takes a `type` of `type` (i.e. any type) and unwraps the type to its inner type.

### Generic Types
Since number cannot be initialized, if we want to write a function that works for any number, we must write a function using the generic syntax that allows any of the types specified by the syntax to operate on. Generic types are only allowed in function and can only be initialized as parameters.

```
> f (x <number>) x <1> +
function: f (<number>) -> <number>
```

### Do types have attributes or method or both?
All attributes are actually functions (lazily loaded when specified to be so)

```
> 3 4 / . ~ create a ratio of 3 / 4 and then call the . function to cast it into a decimal type ~
dec: 0.75
```

```
> a 3 4 / =
a = ratio: 3 / 4

> a . ~ cast a into a decimal type, also stores the decimal representation of a into a ~
dec: 0.75

> a . ~ load the existing decimal cast of value a ~
dec: 0.75   
```

Note: `a .` would be called an anonymous expression because it is not given a name.

Any use of the `generic` `type` in the `type` declaration must be wrapped in `< >` and any actual creation of data must also be wrapped in `< >`, which when evaluated will be cast into the `type` provided to the function. (E.g. `2 f` where `2` is an `int`, will return `3` because internally the `<1>` was cast to an `int` with `1 int`)

### Making custom types
- TODO

### Deriving from types
- TODO

### Vectors

Vectors in basis are like mathematical vectors by default but can also be used as containers
```
> [1 2 3]
vector[int]: [1 2 3] -> [size: 1x3]
```

Multidimensional vectors
```
> [1 2 3; 2 3 4; 3 4 5]
vector[int]: [[1 2 3]  [2 3 4]  [3 4 5]] -> [size: 3x3]
```

Vector operations `+` and `-`
```
> [1 2] [2 3] +
vector[int]: [3 5] -> [size: 1x2]

> [1 2] [2 3] -
vector[int]: [-1 -1] -> [size: 1x2]
```

Dot product in basis
```
> [1 2 3] [2 3 4] .
vector[int]: [2 6 12] -> [size: 1x3]
```

### Containers

The `vector` type is a type of `container`, but basis also has support for `tuple` and `set`, both of which are derived from `container`.

```
> [2 3 4 5 5]
vector[int]: [2 3 4 5 5] -> [size: 1x5]

> (2 3 4 5 5)
tuple[int]: (2 3 4 5 5) -> (size: 5)

> {2 3 4 5 5}
set[int]: (2 3 4 5) -> {size: 4}
```

There are more containers like `sample` and `population` for statistics described below.

### Statistics

Mean of a vector, tuple, or set. For simplicity, this function is shown described with only the `ratio` type, but the real definition would be written for the `number` type with the `generic` `type` syntax.
```
> mean (c container[ratio]) =:
    sum: ratio = 0
    count: int = 0
    for x: ratio in c {
        sum x +=
        count 1 + count +=
    }
```

```
> [1 2 3] mean
ratio: 2

> (1 2 3) mean
ration: 2
```

#### Sample and Population
The `sample` and `population` types are very similar in that they wrap the `tuple` type. They are useful because the statistical functions (as described in this section) apply to both of them differently.

Statistical tests
```
> a (1 2 3) sample = ~ creates a tuple, casts it to a sample type, assign it to a ~
a = sample: (1 2 3) -> (size: 3)

> b (1 2 4) sample = ~ creates a tuple, casts it to a sample type, assign it to b ~
b = sample: (1 2 4) -> (size: 3)

> test a b ttest =
test = ttestresult: (pvalue: 0.768) (t: -0.316)

> test pvalue
dec: 0.768
```

### Scripting in basis
Entire scripts can be written outside of the [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) to be saved and run.

minecraft_coord_convert.basis
```
overworld_to_neither(coords vector[int]) =: 8 coords *

neither_to_overworld(coords vector[int]) =: coords 8 /

out_coords [100, 40, 10] overworld_to_neither =

out_coords print
```

### Magic
You may have noticed that the previous examples all have a console. How does the console know what to display below each line? Well, it is[1] lazily computed when any type is created. Any type derived from `type` have a few common attributes that are either lazily loaded if they are specific to the instance (e.g. like the representation for the console (`console` representation), or the `printable` representation). Other attributes like `desc` are pointers to a constant shared description that is the same for in instance of any type. This might be something like the `desc` for `int`, "Integer numeric type that stores whole positive and negative numbers". The `console` attribute is actually a function and may use the `printable` attribute (which is also a function). For example, `printable` for an int might be "1", and the `console` will be `"int: {}" printable`, which in full form will look like `console (): "int: {}" printable`
 
[1] Note: This is just the document outlining the implementation, the implementation is not yet written, so technically it isn't lazily loaded yet, but will be when the implementation is written in it's entirety. Other instances of "is" statements may be the same case.
