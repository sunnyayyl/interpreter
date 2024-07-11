use core::Token;
use paste::paste;

macro_rules! defaultTokenLiteralImpl {
    () => {
        fn token_literal(&self) -> String {
            self.token.token_literal()
        }
    };
}
macro_rules! workaround{
    ($type:ident,$($object:ident),+)=>{
        #[derive(Clone)]
        pub enum $type{
            $(
                $object($object)
            ),+
        }
        impl $type{
            pub fn token_literal(&self)->String{
                match self{
                    $(
                        $type::$object(o)=>o.token_literal(),
                    )+
                }
            }
            pub fn string(&self)->String{
                match self{
                    $(
                        $type::$object(o)=>o.string(),
                    )+
                }
            }
            paste!{ 
                $(
                    pub fn [<from_some_ $object:snake>](o:Option<$object>)->Option<$type>{
                        match o{
                            Some(o)=>Some($type::$object(o)),
                            None=>None
                        }
                    }
                )+
            }

        }
    }

}

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub struct Program {
    pub statements: Vec<Statement>,
}
impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}
impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
    fn string(&self) -> String {
        let mut out = String::new();
        for i in self.statements.iter() {
            out.push_str(i.string().as_str())
        }
        out
    }
}
#[derive(Clone)]

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>,
}
impl Node for LetStatement {
    defaultTokenLiteralImpl!();
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(self.token_literal().as_str());
        out.push_str(" ");
        out.push_str(self.name.string().as_str());
        out.push_str(" = ");
        match &self.value {
            Some(e) => out.push_str(e.string().as_str()),
            None => {}
        }
        out.push_str(";");
        out
    }
}
#[derive(Clone)]

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Expression>,
}
impl Node for ReturnStatement {
    defaultTokenLiteralImpl!();
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str(self.token_literal().as_str());
        out.push_str(" ");
        match &self.return_value {
            Some(e) => out.push_str(e.string().as_str()),
            None => {}
        }
        out.push_str(";");
        out
    }
}
#[derive(Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}
impl Node for Identifier {
    defaultTokenLiteralImpl!();
    fn string(&self) -> String {
        self.value.clone()
    }
}
impl Identifier {
    pub fn default() -> Self {
        Self {
            token: Token::default(),
            value: String::from(""),
        }
    }
}
#[derive(Clone)]

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Expression>,
}
impl Node for ExpressionStatement {
    defaultTokenLiteralImpl!();
    fn string(&self) -> String {
        match &self.expression {
            Some(e) => return e.string(),
            None => {}
        }
        String::from("")
    }
}
workaround!(Expression, Identifier);
workaround!(
    Statement,
    LetStatement,
    ReturnStatement,
    ExpressionStatement
);
