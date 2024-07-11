use lexer::Lexer;
use parser::{ast::{Node, Statement}, Parser};


#[test]
fn test_let_statements(){
    let input=String::from("let x=5;
    let y=10;
    let foobar = 0;");
    let mut lexer=Lexer::new(input);
    let mut parser=Parser::new(&mut lexer);
    let program=parser.parse_program();
    parser.check_parser_errors();
    match program{
        Some(p)=>{
            
            if p.statements.len()!=3{
                panic!("program.statements expected 3, got {}",p.statements.len())
            }
            let test_case=["x","y","foobar"];
            for (i,v) in (test_case).iter().enumerate(){
                let statment=&p.statements[i];
                test_let_statement(statment, v.to_string())
            }
            
        },
        None=>{
            panic!("ParseProgram returned None")
        }
    }
}
fn test_let_statement( s: &Statement, name:String){
    let token_literal:String=s.token_literal();
    if token_literal!=String::from("let"){
        panic!("token_literal not let, got {}",token_literal)
    }
    match s{
        Statement::LetStatement(ls)=>{
            if ls.name.value!=name{
                panic!("Name of let statement should be {}, got {}",name,ls.name.value)
            }
            if ls.name.token_literal()!=name{
                panic!("token literal should be {}, got {}",name,ls.name.value)
            }
        }
        _=>panic!("statement is not LetStatement")
    }
    
}
#[test]
fn test_return_statements(){
    let input=String::from("return 5;
    return 10;
    return 345;");
    let mut lexer=Lexer::new(input);
    let mut parser=Parser::new(&mut lexer);
    let program=parser.parse_program();
    parser.check_parser_errors();
    match program{
        Some(p)=>{
            if p.statements.len()!=3{
                panic!("program.statements expected 3, got {}",p.statements.len())
            }
            for (i,v) in p.statements.iter().enumerate(){
                match v{
                    Statement::ReturnStatement(rs)=>{
                        if rs.token_literal()!="return"{
                            panic!("token_literal not return, got {}",rs.token_literal())
                        }
                    }
                    _=>panic!("statement {} is not ReturnStatement",i)
                }
            }
        }
        None=>{
            panic!("ParseProgram returned None")
        }
    }
}