use kuon::{
    analyzer,
    compiler::{self, value::Value},
    lexer::Lexer,
    parser, vm,
};

fn eval(code: &str) -> Value {
    let code_str = code.to_string();
    let mut lexer = Lexer::from_string(&code_str);

    let ast = parser::parse_source(&mut lexer).unwrap();
    analyzer::validate(&ast).unwrap();

    let chunk = compiler::compile_source(&ast).unwrap();

    vm::execute(&chunk)
}

#[test]
fn test1() {
    let source = "1 + 2";
    assert_eq!(eval(source), Value::Int(3));
}

#[test]
fn test2() {
    let source = "
    1 + 2;
    2 + 3
    ";
    assert_eq!(eval(source), Value::Int(5));
}

#[test]
fn test3() {
    let source = "
    val a = 3;
    val b = 4;
    a + b
    ";
    assert_eq!(eval(source), Value::Int(7));
}

#[test]
fn test4() {
    let source = "
    val a = 3;
    val b = 4;
    if a > b {
        \"abc\"
    } else {
        \"def\"
    }
    ";
    assert_eq!(eval(source), Value::String("def".to_string()));
}

#[test]
fn test5() {
    let source = "
    fun add(a int, b float) float {
        a + b
    }

    add(3, 2.5)
    ";
    assert_eq!(eval(source), Value::Float(5.5));
}

#[test]
fn test6() {
    let source = "
    fun factorial(n int) int {
    	if n == 1 {
	    	1
	    } else {
		    n * factorial(n - 1)
	    }
    }

    factorial(5)
    ";
    assert_eq!(eval(source), Value::Int(120));
}

#[test]
fn test7() {
    let source = "
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

    val kuon1 = makePerson1(\"Kuon\", 20);
    val kuon2 = makePerson2(\"Kuon\", 20);

    kuon1.age + kuon2.age
    ";
    assert_eq!(eval(source), Value::Int(40));
}
