# Kuon programming language

---

This document is like a description/list of ideas for this language. It's a hard link to a file from my Obsidian vault where I keep my project notes.
I'll probably add a proper TODO list at some point, but most things mentioned here aren't implemented yet (and may be implemented a little differently? sometimes I forget to update my notes if I change my mind on something as I'm implementing it).

---

Kuon is an embedded programming language.
It's also statically typed, and interpreted (implemented as a bytecode VM).

Some additional ideas
- make something like php on top of it, so a way to write kuon code in any file within some special delimeters like `<?php` and `?>` and then embed the output of the code inside those delimeters into the file
- also a normal interpreter/repl executable
- build tools, formatters, LSP, etc.
- also an extension for VSCode which sort of mimics the common lisp repl which you get in emacs with slime (kuon doesn't really have something like the common lisp condition system, but it's also fully expression based so I think it could work? maybe... would need to think about the whole image thing)

Usage ideas:
- use like php for a blog
- use for macros in other languages
- use to generate docs (embed as comments in a file, which don't expand to anything but update some database which is then used to generate an automatic docs page)
- just overall use it as a general purpose language for scripts, web backends, etc.
- embed it into godot maybe? (and other programs I guess, but I really want to try making games in godot with kuon)

## Expression based

Kuon is expression based, so if-else is an expression, loops are expressions, etc.
Also, curly braces are expressions which contain multiple different expressions inside them (delimited by semicolons). They evaluate to the value of the last expression inside of them.
If the last expression ends with a semicolon, there's an implied `null` after it, so the whole expression evaluates to `null` (this is pretty much the same as with Rust, except it's evaluated to `()` there, it makes it easier to write functions which don't return anything).

### Variable assignment

```
val a = 3; // const
var a = 3; // variable

var a = {
	var b = 2;
	b + 1
};
```

You can also manually specify a type (which might be necessary in some situations?):
```
val a int = 3;
```

## Type system

### Primitive types

```
any, null, int, float, bool, char, string
```

### Type keyword

The `type` keyword is used to define type aliases:
```
type b = bool;
```

### Structs

```
struct Person {
	name string,
	age int,
};

val p = Person {
	name: "Kuon",
	age: 20
}
```

Structs also support default values for their fields:
```
struct Person {
	name string = "Kuon",
	age int = 20,
};
```

You can also use struct types without defining them with the `mkstruct` keyword:
```
fun makePerson(name string, age uint) struct { name string, age int } {
	mkstruct {
		name: "Kuon",
		age: 20
	}
}

val person = getPerson();
// person.name = "Kuon", person.age = 20
```

### Enums

```
enum Element {
	Empty,
	Text {
		value string
	},
	Number {
		value float
	},
}

val e1 = Element:Empty;
val e2 = Element:Text { value: "lorem ipsum" };
```

### Tuples

```
<int, string>

val a = <1, "one">;
```

### Arrays, Hashmaps

```
array[string]
map[string, any]

var a = ["a", "b", "c"];
var m = map {
	"a": 1,
	"b": "...",
	"c": false,
};
```

### Result

This is just an enum definition in the stdlib, same as with Rust.
```
Result[int, string]
```

### Nullability

```
val a nullable string = null;

if a? {
	val b = a!;
	...
} else {
	...
}
```
`a?` returns true in case the value is not null, false otherwise.
`a!` returns the value of `a` as a `string` instead of a `nullable string`. If `a` was null, the program panics.

### Type casts

The `as` operators lets you convert between types. This is especially useful for the `any` type.
Type casts happen at runtime and, if they fail, the program panics.
Some type casts happen implicitly, like float to int, etc. (need to figure out which ones exactly).

I'm also thinking to have a special syntax for adding your own conversion functions. Like for example, you could make a function that parses a string and converts it into one of your structs, and use it with `as`. Something like this:
```
// define a function that converts from string to Person here

val person = "name: Kuon, age: 20" as Person
```

## Destructuring

Destructuring is built into the `val` and `var` blocks and lets you destructure tuples and structs.
```
// TUPLES
// ======

val <key, value> = <1, "one">;
// key = 1, value = "one"

// STRUCTS
// =======

val { name: name, age: age } = Person { name: "Kuon", age: 20 };
val { name, age } = Person { name: "Kuon", age: 20 };
// name = "Kuon", age = 20

val { name, ... } = Person { name: "Kuon", age: 20 };
// name = "Kuon"
```

## Pattern matching

Pattern matching is similar to destructuring, but there's a special `match` block for it and it allows you to also compare values:
```
match person {
	Person { name: "Kuon", age } -> {}
	Person { name: "Eruruu", age } -> {}
	else -> {}
}

match <1, "one"> {
	<1, name> -> {}
	else -> {}
}
```
In the struct case, `age` is expanded to `age: age`, which binds the value of the `age` field to a variable called `age` in that scope. Same thing for the tuple case, `name` is bound to `"one"`.

In the case of tuples, you can also match like this:
```
match tuple {
	<a, b, a> -> {}
	else -> {}
}
```
This would match the pattern if the first and third element of the tuple are equal.

Pattern matching also works on enums and arrays:
```
match [1, 2, 3, 4, 5] {
	[1, 2, ...] -> {}
	[..., 3, ...] -> {}
	[..., 4, 5] -> {}
	else -> {}
}

match element {
	Element::Empty -> {}
	
	Element::Text { value: "idk" } -> {}
	Element::Text { value } -> {}
	
	Element::Number { value } -> {}
}
```
The `...` means that you don't care about the rest of the values, it also works on struct fields.

## Functions

```
fun add(a int, b int) int {
	a + b
}
```

### Optional, keyword, rest parameters

Optional parameter:
```
fun foo(a int, b string = "default value") {}

foo(a)
foo(a, "some other value")
```

...

### Closures

```
val a = 5;
val f = fun (n int) int {
	n + a
}

f(3) // 8
```

### First-class functions

```
val twice = fun (f fun (int) int, n int) int {
	f(f(n))
}
val increment = fun (n int) int {
	n + 1
}

twice(increment, 1) // 3
```

### Pass by value/reference

Kuon is pass-by-value by default, i.e. arguments you pass to a function are copied in memory (this includes structs as well as primitive types).
The exceptions are:
- array
- map
- string
- fun
They contain pointers internally and those pointers are what's passed by value, so the entire type is effectively pass-by-reference. They're also garbage collected, while other types of values are just stored on the function's stack and freed when the function returns.

However, this does not mean you mean you can modify them in a function without explicitly saying that in the function definition with `var`!!

### Var function parameters

You can specify that you want a function to be able to modify a parameter it's passed in with the `var` keyword.
This passes primitive types and structs/enums by reference, and arrays, maps, strings and functions are already pass-by-reference implicitly.

```
fun increment(var n int) {
	n += 1;
}

var n = 0;
increment(n);
// n == 1
```

In case you don't use a `var` for a parameter, there's an implicit `val` in there, but you can also type it in yourself:
```
fun increment(val n int) {
	n += 1; // ERROR!!
}

fun increment(n int) {
	n += 1; // ALSO ERROR!!
}
```

### Value and type functions

Value functions are pretty much just methods:
```
fun (val n int):abs() int {
	if n > 0 {
		n
	} else {
		-n
	}
}

fun (var n int):increment() {
	n += 1;
}

val n = -4;
// n:abs() == 4
n:increment();
// n:abs() == 3
```

Type functions are like static methods. They're useful for things like constructors:
```
fun Person#new(name string, age int) Person {
	Person { name: name, age: age }
}

let p = Person#new("Kuon", 20)
```

## Interfaces

```
interface SupportsArithmetic {
	fun (val n this):negate() this
	fun add(a this, b this) this
	fun multiply(a this, b this) this
	fun divide(a this, b this) this
}

fun (val n CustomNumber):negate() CustomNumber { ... }
fun add(a CustomNumber, b CustomNumber) CustomNumber { ... }
fun multiply(a CustomNumber, b CustomNumber) CustomNumber { ... }
fun divide(a CustomNumber, b CustomNumber) CustomNumber { ... }
```
Now the type `CustomNumber` implements the `SupportsArithmetic` interface (btw it's pretty nice how you can read this as *CustomNumber now supports arithmetic*).
*SupportsArithmetic* will probably be a built-in interface that the +, -, etc. operators use.

## Generics

```
fun [Type] foo(a Type) Type {
	...
}
```

This also applies to value and type functions.
They can be combined with interfaces:
```
fun [Type SupportsArithmetic] sum(a Type, b Type) Type {
	a + b
}
```

## Quantities and Units

Kuon deals with converting between different units of measure for you!!
This feature is inspired by the [Frink language](https://www.hillelwayne.com/post/frink/). F# also has a feature for expressing units of measure in the type system, but it's much, much more limited than Frink.

As an example, here are some quantity definitions:
```
quantity distance {
	millimeters = 1/1000 meters,
	centimeters = 1/100 meters,
	meters = 1,
	kilometers = 1000 meters,
}

quantity time {
	milliseconds = 1/1000 seconds,
	seconds = 1,
	minutes = 60 seconds,
	hours = 60 minutes,
}

quantity weight {
	grams = 1,
	kilograms = 1000 grams,
	tons = 1000 kilograms,
}

quantity velocity [distance/time] { }

quantity acceleration [velocity/time] { }

quantity force [weight * acceleration] {
	newtons = 1 kilograms * meters/seconds^2
	kilonewtons = 1000 newtons
}
```

You can then write a function such as:
```
fun secondsInHours(h: int) int {
	h<hours> into seconds
}

// secondsInHours(2) == 120
```
The `into` operators takes a quantity and returns its value as a `float` in the requested unit, such as `seconds` in the example above. Floats are auto-converted into integers when required, so that's why the function can return an `int`.

Another example:
```
val w: weight = 30<grams>;
val a: acceleration = 5<centimeters/minutes^2>;
val f: float = (w/a) into newtons;

// f == 1.6666666666666667e-8
```
(those type annotations in the `val` expressions are not necessary, I added them for clarity)

## More ideas

- something like the common lisp `loop` macro
- lots of functions for string formatting and parsing
	- maybe a built-in templating engine? So a function which takes in a string that could have Kuon code embedded between curly braces which is then evaluated
- pre and post conditions for functions
	- just conditions checked at runtime at the beginning of a function and just before it returns, they mainly serve as documentation but they're checked whenever the function is called and the program panics if they're not satisfied (this can be disabled)
	- also something like another kind of condition that's like unit tests? Like, you write a function and you can specify that as a precondition it can't take in 0, as a post condition the returned string will never be empty, and you provide examples (so calling it with 1 returns "abc", calling it with 2 returns "def", etc.). These examples serve as documentation, same as pre and post conditions do, but they can also be run with a test command.
- `defer` expression
- `clone` expression
	- Lets you clone a value, because arrays, maps, strings and functions are passed by reference (their internal pointers are copied). Prefixing a value with `clone` creates a new allocation and does a full copy of the data.
	- If called on a struct, it clones all of its members recursively
- `swap` expression
	- Just swaps 2 variables
- random modules in the standard library for opening a window, making http requests, etc. - there's a lot of useful high-level things which should be in language stdlibs in CURRENT_YEAR but usually aren't for whatever reason
- linear algebra math and complex number math built into the language (so uhh... a matrix type and complex number type and lots of operations that work on them)
- something like matplotlib built into the stdlib? just like how it's super easy to do that kinda stuff in python