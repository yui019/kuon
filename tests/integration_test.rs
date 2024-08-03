use std::collections::HashMap;

use kuon::{
    analyzer,
    compiler::{self},
    lexer::Lexer,
    parser,
    vm::{self, execution_result::ExecutionResult},
};

fn eval(code: &str) -> ExecutionResult {
    let code_str = code.to_string();
    let mut lexer = Lexer::from_string(&code_str);

    let mut ast = parser::parse_source(&mut lexer).unwrap();
    analyzer::validate(&mut ast).unwrap();

    let chunk = compiler::compile_source(&ast).unwrap();

    vm::execute(&chunk)
}

#[test]
fn test1() {
    let source = "1 + 2";
    assert_eq!(eval(source), ExecutionResult::Int(3));
}

#[test]
fn test2() {
    let source = r#"
    1 + 2;
    2 + 3
    "#;
    assert_eq!(eval(source), ExecutionResult::Int(5));
}

#[test]
fn test3() {
    let source = r#"
    val a = 3;
    val b = 4;
    a + b
    "#;
    assert_eq!(eval(source), ExecutionResult::Int(7));
}

#[test]
fn test4() {
    let source = r#"
    val a = 3;
    val b = 4;
    if a > b {
        "abc"
    } else {
        "def"
    }
    "#;

    assert_eq!(eval(source), ExecutionResult::String("def".to_string()));
}

#[test]
fn test5() {
    let source = r#"
    fun add(a int, b float) float {
        a + b
    }

    add(3, 2.5)
    "#;
    assert_eq!(eval(source), ExecutionResult::Float(5.5));
}

#[test]
fn test6() {
    let source = r#"
    fun factorial(n int) int {
    	if n == 1 {
	    	1
	    } else {
		    n * factorial(n - 1)
	    }
    }

    factorial(5)
    "#;
    assert_eq!(eval(source), ExecutionResult::Int(120));
}

#[test]
fn test7() {
    let source = r#"
    struct Person {
	    name string
	    age int
    }

    fun makePerson1(name string, age int) Person {
	    Person { name: name, age: age }
    }

    fun makePerson2(name string, age int) struct { name string, age int } {
	    mkstruct { name: name, age: age }
    }

    val kuon1 = makePerson1("Kuon", 20);
    val kuon2 = makePerson2("Kuon", 20);

    kuon1.age + kuon2.age
    "#;
    assert_eq!(eval(source), ExecutionResult::Int(40));
}

#[test]
fn test8() {
    let source = r#"
    fun modifyStr(var s string) null {
        s = "def";
    }

    var a = "abc";
    modifyStr(a);
    a;
    "#;

    assert_eq!(eval(source), ExecutionResult::String("def".to_string()));
}

#[test]
fn test9() {
    let source = r#"
    fun modifyInt(var n int) null {
        n = 2;
    }

    var a = 1;
    modifyInt(a);
    a;
    "#;

    assert_eq!(eval(source), ExecutionResult::Int(2));
}

#[test]
fn test10() {
    let source = r#"
    struct PersonAge {
        years int,
        months int,
        important_field string
    }

    struct Person {
        name string,
        age PersonAge
    }

    fun modifyInt(var n int, newValue int) null {
        n = newValue;
    }

    fun modifyPersonAge(var a PersonAge) null {
        a.important_field = "abc";
        modifyInt(a.years, 15);
    }

    fun modifyPerson(var p Person) null {
        p.name = "kuon";
        modifyPersonAge(p.age);
        modifyInt(p.age.months, 8);
    }

    var p1 = Person {
        name: "idk",
        age: mkstruct {
            years: 2,
            months: 3,
            important_field: "idk"
        }
    };

    modifyPerson(p1);

    p1
    "#;

    assert_eq!(
        eval(source),
        ExecutionResult::Struct {
            fields: HashMap::from([
                (
                    "name".to_string(),
                    ExecutionResult::String("kuon".to_string())
                ),
                (
                    "age".to_string(),
                    ExecutionResult::Struct {
                        fields: HashMap::from([
                            (
                                "important_field".to_string(),
                                ExecutionResult::String("abc".to_string())
                            ),
                            ("months".to_string(), ExecutionResult::Int(8)),
                            ("years".to_string(), ExecutionResult::Int(15))
                        ])
                    }
                )
            ])
        }
    );
}

#[test]
fn test11() {
    let source = r#"
    fun (n int):abs() int {
        if n > 0 {
            n
        } else {
            -n
        }
    }

    fun (var n int):inc() null {
        n = n + 1;
    }

    val n = -3;
    n:inc();
    n:abs()
    "#;

    assert_eq!(eval(source), ExecutionResult::Int(2));
}
