# Specification for the Moses Programming Language

Moses is a functional programming language named after Moses Isajewitsch Schönfinkel (Моисей Исаевич Шейнфинкель). A Moses program consists of one valid Moses expression whose evaluation is the execution of the program.

In this specification _emphasised_ words and sentences are comments of the designer regarding things he is not really sure about.

## Eager vs. Lazy

It is still not clear whether Moses is evaluating its expressions eagerly or lazily. It is quite appealing to have blocks be evaluated by only evaluating its last expression, using the earlier expressions of the block only as it becomes necessary. This would go hand in hand with a pure, non-mutable paradigm. On the other hand, I’d like to allow for occasional side effects, if the programmer wants to use them, which would suggest an eager style.

## Comments and Whitespace

Moses has support for line comments which are initiated by a `#` that is not part of a string or character literal and end at the end of the line. Comments are ignored by the Moses interpreter/compiler.

A line consisting of Whitespace only is ignored. Whitespace between token is also ignored.

## Literals

```EBNF
Literal = StringLiteral | CharacterLiteral | NumberLiteral;

StringLiteral = '"', {UnicodeCharacter}, '"';

CharacterLiteral = "'", {UnicodeCharacter}, "'";

NumberLiteral = IntegerLiteral | FractionLiteral | DecFractionLiteral;
IntegerLiteral = ["-"], NaturalLiteral;
NaturalLiteral = NonZeroDigit, {Digit | "_"} | "0";
NonZeroDigit = "1" - "9";
Digit = "0" | NonZeroDigit;
FractionLiteral = IntegerLiteral, "%", IntegerLiteral;
DecimalLiteral = [IntegerLiteral], ".", NaturalLiteral;
```

## Identifiers

```EBNF
Identifier = TypeIdentifier | FunctionIdentifier
TypeIdentifier = "A" - "Z", {"A" - "Z" | "a" - "z" | Digit | "_"};
VariableIdentifier = "a" - "z", {"A" - "Z" | "a" - "z" | Digit | "_"};
```

## Types and Function Types

Moses supports higher kinded types (of the kind `* -> *`, others via partial application) and higher order functions. In fact, higher kinded types are functions of the type `Type -> Type`, where `Type` is the type of all (small) types. (Due to the structure of the category of (small) Types, type functions are elements of `Type`as well.)

### Type Expressions

A type expression is either a built-in type, an enumeration of values, or the categorical sum or product of types. While products are constructed using the `,` combinator (like it is the case with the respective values), sums and enumerations (which are sums of singleton types) are constructed via the `|` combinator. The precedence is `,` over `|`, left-associative. Parentheses can be used to structure type expressions; `()` describes the unit type with the singleton value unit (`()`).

```EBNF
CustomEnum = 1 | 2 | 3 | 4
CustomSum = FooType | BarType
CustonProd = FooType, BarType
```

### Type Annotations

Every expression can be followed by a type annotation `Expression, ":", TypeExpression`, which tries to convert the expression to the given type after evaluating it, if possible. If such conversion fails, a panic occurs.

## (Value) Expressions

Literals are expressions, as are variable identifiers and function calls. Expressions can be structured by using parentheses `(` and `)` in the usual manner. Parentheses also introduce new scopes. There are empty expressions with the unit value `()`, namely the empty scope.

### Definitions

The definition of a new name to the scope works like this:
```ENBF
Defintion = TypeDefintion | VariableDefinition;
TypeDefinition = TypeIdentifier, "=", TypeExpression;
VariableDefinition = VariableIdentifier, "=", ValueExpression;
```
`ValueExpression`s are expressions that are not `TypeExpression`s. A definition is an expression as well, evaluating to its left hand side, but with the side effect of introducing the name to the scope. _It is not yet clear, if the evaluation of the right hand side should be eager or lazy._

_What to do with multiple definitions like `foo = bar = baz`? I feel they can be difficult to handle in combination with tuple and block combinators._

### Blocks

A block is a list of expressions with the value of the last expression in the block. The block combinator is a `;`, merging the two expressions next to it into a block. The block combinator is left associative and has a very low precedence merging expressions as large as possible. Thus a block should generally be encapsulated in parentheses. Out of comfort, it is allowed to omit block combinators (that is only use newlines) in the top-level scope _and possibly everywhere_ as long as no ambiguity arises _(question being, how to check for that)_.

### Tuples

A tuple is a list of expression whose value is determined by _all_ expressions that are part of it. Members can be accessed via zero-based indexing. The type of a tuple is the type theoretical product of the types of the constituents. The tuple combinator is a `,`, merging the two expressions next to it into a tuple, comparable to the block combinator. Nested tuples are always flattened, to prevent flattening, use special constructors. Indexing is done by
1. brackets indexing: `foo[0]` _preferably, because standard_
2. dot indexing: `foo.0` _cool (mainly because of the name functionalities), but potentially confusing_

There are also named tuples, where one or multiple members get a name additional to the their index. This does not change the type of the tuple, but allows the interpreter/compiler to warn the user about wrong usage. Functions can also “insist” on taking tuples with fitting names. Names of members are introduced via the `=` combinator. _There is a problem with definitions within tuples, if a tuple construction is not encapsulated via parentheses. We have to see, if it works out._

Examples
```Moses
pair = (fst = 13, snd = 42) # _not_ equivalent to pair = fst = 13, snd = 42
pair.fst
pair.2
```

## Function Calls

1. fitting names

Unit is normally ignored in function calls, so that e. g. a function `foo: S, T -> U` can also be called like `foo s, t, ()` or, more importantly, like `foo (s, t,)` and still works as expected. The same is true for functions `bar: S -> T` and calls `bar (s1; s2;)`. This is, so that one doesn’t need to take care of trailing commas and separators in list-like environments.

## Panics

If an error occurs that can’t be recovered from, the Moses program exits with the error message as exit value. The interpreter prompts the error message and the place its occurrence, the compiler provides the option for a wrapper to every program printing the error message to stderr.