#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub data: TokenData,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenData {
    LeftParenCurly,  // {
    LeftParenSquare, // [
    LeftParenNormal, // (

    RightParenCurly,  // }
    RightParenSquare, // ]
    RightParenNormal, // )

    Semicolon,   // ;
    Comma,       // ,
    Dot,         // .
    ThreeDots,   // ...
    Colon,       // :
    DoubleColon, // ::

    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /

    PlusEquals,  // +=
    MinusEquals, // -=
    StarEquals,  // *=
    SlashEquals, // /=

    Val, // val
    Var, // var

    Any,    // any
    Null,   // null
    Int,    // int
    Float,  // float
    Bool,   // bool
    Char,   // char
    String, // string

    Array, // array
    Map,   // map

    Nullable,        // nullable
    QuestionMark,    // ?
    ExclamationMark, // !

    As, // as

    True,  // true
    False, // false

    Struct,   // struct
    MkStruct, // mkstruct
    Enum,     // enum

    Interface, // interface

    Ref, // ref

    ThisCapital,    // This
    ThisNoncapital, // this

    Fun, // fun

    If,   // if
    Else, // else

    Match,      // match
    MatchArrow, // ->

    And, // and
    Or,  // or
    Not, // not

    Equals,            // =
    EqualsEquals,      // ==
    ExclamationEquals, // !=

    LessThan,           // <
    LessThanOrEqual,    // <=
    GreaterThan,        // >
    GreaterThanOrEqual, // >=

    ValueString(String),     // "abc"
    ValueChar(char),         // 'a'
    ValueInt(i64),           // 42
    ValueFloat(f64),         // 3.14
    ValueIdentifier(String), // foo

    Error(String),
}
