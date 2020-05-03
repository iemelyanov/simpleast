mod ast0 {
    enum BinOp {
        Add,
        Sub,
        Mul,
        Div,
    }

    enum Variable {
        A,
        B,
        C,
    }

    enum Expr {
        Literal(f64),
        Variable(Variable),
        Binary {
            lhs: Box<Expr>,
            op: BinOp,
            rhs: Box<Expr>,
        },
    }

    fn play() {
        //  b * b - 4 * a * c => ast( - (* b b) (* 4 (* a  c)))
        let tree = Expr::Binary {
            lhs: Box::new(Expr::Binary {
                lhs: Box::new(Expr::Variable(Variable::B)),
                op: BinOp::Mul,
                rhs: Box::new(Expr::Variable(Variable::B)),
            }),
            op: BinOp::Sub,
            rhs: Box::new(Expr::Binary {
                lhs: Box::new(Expr::Literal(4 as f64)),
                op: BinOp::Mul,
                rhs: Box::new(Expr::Binary {
                    lhs: Box::new(Expr::Variable(Variable::A)),
                    op: BinOp::Mul,
                    rhs: Box::new(Expr::Variable(Variable::C)),
                }),
            }),
        };
    }
}

mod ast1 {
    enum Result {
        Numeric(f64),
        // For prety printing
        StringRepr(String),
        None,
    }

    #[derive(Debug, Copy, Clone)]
    enum BinOp {
        Add,
        Sub,
        Mul,
        Div,
    }

    #[derive(Debug)]
    enum UnOp {
        Add,
        Sub,
    }

    #[derive(Debug)]
    enum Literal {
        Numeric(f64),
    }

    #[derive(Debug)]
    struct BinaryExpr {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    }

    #[derive(Debug)]
    struct GroupExpr {
        expr: Box<Expr>,
    }

    #[derive(Debug)]
    struct LiteralExpr {
        val: Literal,
    }

    #[derive(Debug)]
    struct UnaryExpr {
        op: UnOp,
        rhs: Box<Expr>,
    }

    #[derive(Debug)]
    enum Expr {
        Binary(BinaryExpr),
        Group(GroupExpr),
        Literal(LiteralExpr),
        Unary(UnaryExpr),
    }

    impl Expr {
        fn accept(&self, visitor: &mut dyn Visitor) -> Result {
            match self {
                Expr::Binary(ref expr) => visitor.visit_binary(expr),
                Expr::Group(ref expr) => visitor.visit_group(expr),
                Expr::Literal(ref expr) => visitor.visit_literal(expr),
                Expr::Unary(ref expr) => visitor.visit_unary(expr),
            }
        }
    }

    trait Visitor {
        fn visit_binary(&mut self, expr: &BinaryExpr) -> Result;
        fn visit_group(&mut self, expr: &GroupExpr) -> Result;
        fn visit_literal(&mut self, expr: &LiteralExpr) -> Result;
        fn visit_unary(&mut self, expr: &UnaryExpr) -> Result;
    }

    struct Repr {}

    impl Repr {
        fn parenthesize(&mut self, name: &str, exprs: Vec<&Expr>) -> String {
            let mut s = String::from("(");
            s.push_str(name);

            for e in exprs {
                s.push_str(" ");
                match e.accept(self) {
                    Result::StringRepr(v) => s.push_str(v.as_str()),
                    _ => {}
                }
            }
            s.push_str(")");

            s
        }

        fn repr(&mut self, ast: &Expr) -> String {
            match ast.accept(self) {
                Result::StringRepr(s) => s,
                _ => String::from("?"),
            }
        }
    }

    impl Visitor for Repr {
        fn visit_binary(&mut self, expr: &BinaryExpr) -> Result {
            Result::StringRepr(self.parenthesize(
                format!("{:?}", expr.op).as_str(),
                vec![&expr.lhs, &expr.rhs],
            ))
        }

        fn visit_group(&mut self, expr: &GroupExpr) -> Result {
            Result::StringRepr(self.parenthesize("group", vec![&expr.expr]))
        }

        fn visit_literal(&mut self, expr: &LiteralExpr) -> Result {
            Result::StringRepr(String::from(format!("{:?}", expr.val)))
        }

        fn visit_unary(&mut self, expr: &UnaryExpr) -> Result {
            Result::StringRepr(
                self.parenthesize(format!("{:?}", expr.op).as_str(), vec![&expr.rhs]),
            )
        }
    }

    struct Interpreter {}

    impl Interpreter {
        pub fn eval(&mut self, ast: &Expr) -> Result {
            ast.accept(self)
        }
    }

    impl Visitor for Interpreter {
        fn visit_binary(&mut self, expr: &BinaryExpr) -> Result {
            let left = expr.lhs.accept(self);
            let right = expr.rhs.accept(self);

            if let (Result::Numeric(a), Result::Numeric(b)) = (left, right) {
                let r = match expr.op {
                    BinOp::Add => a + b,
                    BinOp::Sub => a - b,
                    BinOp::Div => a / b,
                    BinOp::Mul => a * b,
                };

                return Result::Numeric(r);
            }

            Result::None
        }

        fn visit_group(&mut self, expr: &GroupExpr) -> Result {
            expr.expr.accept(self)
        }

        fn visit_literal(&mut self, expr: &LiteralExpr) -> Result {
            match expr.val {
                Literal::Numeric(v) => Result::Numeric(v),
            }
        }

        fn visit_unary(&mut self, expr: &UnaryExpr) -> Result {
            match expr.rhs.accept(self) {
                Result::Numeric(v) => match expr.op {
                    UnOp::Add => {
                        return Result::Numeric(v);
                    }
                    UnOp::Sub => {
                        return Result::Numeric(-v);
                    }
                },
                _ => {}
            }

            Result::None
        }
    }

    pub fn play() {
        let ast = Box::new(Expr::Binary(BinaryExpr {
            lhs: Box::new(Expr::Unary(UnaryExpr {
                op: UnOp::Sub,
                rhs: Box::new(Expr::Literal(LiteralExpr {
                    val: Literal::Numeric(3.14),
                })),
            })),
            op: BinOp::Mul,
            rhs: Box::new(Expr::Group(GroupExpr {
                expr: Box::new(Expr::Literal(LiteralExpr {
                    val: Literal::Numeric(2.81),
                })),
            })),
        }));

        let mut repr = Repr {};
        println!("{}", repr.repr(&ast));

        let mut intrpret = Interpreter {};
        match intrpret.eval(&ast) {
            Result::Numeric(v) => println!("{}", v),
            _ => {}
        }
    }
}

mod ast2 {

    enum Result {
        Numeric(f64),
        String(String),
        None,
    }

    #[derive(Debug)]
    enum Literal {
        Numeric(f64),
    }

    #[derive(Debug, Copy, Clone)]
    enum BinOp {
        Add,
        Sub,
        Mul,
        Div,
    }

    #[derive(Debug)]
    enum UnOp {
        Add,
        Sub,
    }

    struct BinaryExpr<T> {
        op: BinOp,
        lhs: Box<dyn Expr<T>>,
        rhs: Box<dyn Expr<T>>,
    }

    struct GroupExpr<T> {
        expr: Box<dyn Expr<T>>,
    }

    struct UnaryExpr<T> {
        op: UnOp,
        rhs: Box<dyn Expr<T>>,
    }

    struct LiteralExpr {
        val: Literal,
    }

    trait Expr<T> {
        fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
    }

    impl<T> Expr<T> for BinaryExpr<T> {
        fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
            visitor.visit_binary(self)
        }
    }

    impl<T> Expr<T> for GroupExpr<T> {
        fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
            visitor.visit_group(self)
        }
    }

    impl<T> Expr<T> for UnaryExpr<T> {
        fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
            visitor.visit_unary(self)
        }
    }

    impl<T> Expr<T> for LiteralExpr {
        fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
            visitor.visit_literal(self)
        }
    }

    trait Visitor<T> {
        fn visit_binary(&mut self, expr: &BinaryExpr<T>) -> T;
        fn visit_group(&mut self, expr: &GroupExpr<T>) -> T;
        fn visit_unary(&mut self, expr: &UnaryExpr<T>) -> T;
        fn visit_literal(&mut self, expr: &LiteralExpr) -> T;
    }

    struct Repr {}

    impl Repr {
        fn parenthesize(&mut self, name: &str, exprs: Vec<&Box<dyn Expr<String>>>) -> String {
            let mut s = String::from("(");
            s.push_str(name);

            for e in exprs {
                s.push_str(" ");
                s.push_str(e.accept(self).as_str());
            }
            s.push_str(")");

            s
        }

        pub fn repr(&mut self, ast: &dyn Expr<String>) -> String {
            ast.accept(self)
        }
    }

    impl Visitor<String> for Repr {
        fn visit_binary(&mut self, expr: &BinaryExpr<String>) -> String {
            self.parenthesize(
                format!("{:?}", expr.op).as_str(),
                vec![&expr.lhs, &expr.rhs],
            )
        }

        fn visit_group(&mut self, expr: &GroupExpr<String>) -> String {
            self.parenthesize("group", vec![&expr.expr])
        }

        fn visit_unary(&mut self, expr: &UnaryExpr<String>) -> String {
            self.parenthesize(format!("{:?}", expr.op).as_str(), vec![&expr.rhs])
        }

        fn visit_literal(&mut self, expr: &LiteralExpr) -> String {
            String::from(format!("{:?}", expr.val))
        }
    }

    struct Interpreter {}

    impl Interpreter {
        pub fn eval(&mut self, ast: &dyn Expr<Result>) -> Result {
            ast.accept(self)
        }
    }

    impl Visitor<Result> for Interpreter {
        fn visit_binary(&mut self, expr: &BinaryExpr<Result>) -> Result {
            let left = expr.lhs.accept(self);
            let right = expr.rhs.accept(self);

            if let (Result::Numeric(a), Result::Numeric(b)) = (left, right) {
                let r = match expr.op {
                    BinOp::Add => a + b,
                    BinOp::Sub => a - b,
                    BinOp::Div => a / b,
                    BinOp::Mul => a * b,
                };

                return Result::Numeric(r);
            }

            Result::None
        }

        fn visit_group(&mut self, expr: &GroupExpr<Result>) -> Result {
            expr.expr.accept(self)
        }

        fn visit_literal(&mut self, expr: &LiteralExpr) -> Result {
            match expr.val {
                Literal::Numeric(v) => Result::Numeric(v),
            }
        }

        fn visit_unary(&mut self, expr: &UnaryExpr<Result>) -> Result {
            match expr.rhs.accept(self) {
                Result::Numeric(v) => match expr.op {
                    UnOp::Add => {
                        return Result::Numeric(v);
                    }
                    UnOp::Sub => {
                        return Result::Numeric(-v);
                    }
                },
                _ => {}
            }

            Result::None
        }
    }

    pub fn play() {
        let ast = Box::new(BinaryExpr {
            lhs: Box::new(UnaryExpr {
                op: UnOp::Sub,
                rhs: Box::new(LiteralExpr {
                    val: Literal::Numeric(3.14),
                }),
            }),
            op: BinOp::Mul,
            rhs: Box::new(GroupExpr {
                expr: Box::new(LiteralExpr {
                    val: Literal::Numeric(2.81),
                }),
            }),
        });

        // let mut repr = Repr {};
        // println!("{}", repr.repr(&*ast));

        let mut intrpret = Interpreter {};
        match intrpret.eval(&*ast) {
            Result::Numeric(v) => println!("{}", v),
            _ => {}
        }
    }
}

mod ast3 {
    enum Result {
        Numeric(f64),
        // For prety printing
        StringRepr(String),
        None,
    }

    #[derive(Debug, Copy, Clone)]
    enum BinOp {
        Add,
        Sub,
        Mul,
        Div,
    }

    #[derive(Debug)]
    enum UnOp {
        Add,
        Sub,
    }

    #[derive(Debug)]
    enum Literal {
        Numeric(f64),
    }

    #[derive(Debug)]
    struct BinaryExpr<'a> {
        op: BinOp,
        lhs: &'a Expr<'a>,
        rhs: &'a Expr<'a>,
    }

    #[derive(Debug)]
    struct GroupExpr<'a> {
        expr: &'a Expr<'a>,
    }

    #[derive(Debug)]
    struct LiteralExpr {
        val: Literal,
    }

    #[derive(Debug)]
    struct UnaryExpr<'a> {
        op: UnOp,
        rhs: &'a Expr<'a>,
    }

    #[derive(Debug)]
    enum Expr<'a> {
        Binary(BinaryExpr<'a>),
        Group(GroupExpr<'a>),
        Literal(LiteralExpr),
        Unary(UnaryExpr<'a>),
    }

    impl<'a> Expr<'a> {
        fn accept(&self, visitor: &mut dyn Visitor) -> Result {
            match self {
                Expr::Binary(ref expr) => visitor.visit_binary(expr),
                Expr::Group(ref expr) => visitor.visit_group(expr),
                Expr::Literal(ref expr) => visitor.visit_literal(expr),
                Expr::Unary(ref expr) => visitor.visit_unary(expr),
            }
        }
    }

    trait Visitor {
        fn visit_binary(&mut self, expr: &BinaryExpr) -> Result;
        fn visit_group(&mut self, expr: &GroupExpr) -> Result;
        fn visit_literal(&mut self, expr: &LiteralExpr) -> Result;
        fn visit_unary(&mut self, expr: &UnaryExpr) -> Result;
    }

    struct Repr {}

    impl Repr {
        fn parenthesize(&mut self, name: &str, exprs: Vec<&Expr>) -> String {
            let mut s = String::from("(");
            s.push_str(name);

            for e in exprs {
                s.push_str(" ");
                match e.accept(self) {
                    Result::StringRepr(v) => s.push_str(v.as_str()),
                    _ => {}
                }
            }
            s.push_str(")");

            s
        }

        fn repr(&mut self, ast: &Expr) -> String {
            match ast.accept(self) {
                Result::StringRepr(s) => s,
                _ => String::from("?"),
            }
        }
    }

    impl Visitor for Repr {
        fn visit_binary(&mut self, expr: &BinaryExpr) -> Result {
            Result::StringRepr(self.parenthesize(
                format!("{:?}", expr.op).as_str(),
                vec![&expr.lhs, &expr.rhs],
            ))
        }

        fn visit_group(&mut self, expr: &GroupExpr) -> Result {
            Result::StringRepr(self.parenthesize("group", vec![&expr.expr]))
        }

        fn visit_literal(&mut self, expr: &LiteralExpr) -> Result {
            Result::StringRepr(String::from(format!("{:?}", expr.val)))
        }

        fn visit_unary(&mut self, expr: &UnaryExpr) -> Result {
            Result::StringRepr(
                self.parenthesize(format!("{:?}", expr.op).as_str(), vec![&expr.rhs]),
            )
        }
    }

    struct Interpreter {}

    impl Interpreter {
        pub fn eval(&mut self, ast: &Expr) -> Result {
            ast.accept(self)
        }
    }

    impl Visitor for Interpreter {
        fn visit_binary(&mut self, expr: &BinaryExpr) -> Result {
            let left = expr.lhs.accept(self);
            let right = expr.rhs.accept(self);

            if let (Result::Numeric(a), Result::Numeric(b)) = (left, right) {
                let r = match expr.op {
                    BinOp::Add => a + b,
                    BinOp::Sub => a - b,
                    BinOp::Div => a / b,
                    BinOp::Mul => a * b,
                };

                return Result::Numeric(r);
            }

            Result::None
        }

        fn visit_group(&mut self, expr: &GroupExpr) -> Result {
            expr.expr.accept(self)
        }

        fn visit_literal(&mut self, expr: &LiteralExpr) -> Result {
            match expr.val {
                Literal::Numeric(v) => Result::Numeric(v),
            }
        }

        fn visit_unary(&mut self, expr: &UnaryExpr) -> Result {
            match expr.rhs.accept(self) {
                Result::Numeric(v) => match expr.op {
                    UnOp::Add => {
                        return Result::Numeric(v);
                    }
                    UnOp::Sub => {
                        return Result::Numeric(-v);
                    }
                },
                _ => {}
            }

            Result::None
        }
    }

    use typed_arena::Arena;

    pub fn play() {
        let arena: Arena<Expr> = Arena::new();

        let ast = arena.alloc(Expr::Binary(BinaryExpr {
            lhs: arena.alloc(Expr::Unary(UnaryExpr {
                op: UnOp::Sub,
                rhs: arena.alloc(Expr::Literal(LiteralExpr {
                    val: Literal::Numeric(3.14),
                })),
            })),
            op: BinOp::Mul,
            rhs: arena.alloc(Expr::Group(GroupExpr {
                expr: arena.alloc(Expr::Literal(LiteralExpr {
                    val: Literal::Numeric(2.81),
                })),
            })),
        }));

        let mut repr = Repr {};
        println!("{}", repr.repr(&ast));

        let mut intrpret = Interpreter {};
        match intrpret.eval(&ast) {
            Result::Numeric(v) => println!("{}", v),
            _ => {}
        }
    }
}

fn main() {
    ast1::play();
    ast2::play();
    ast3::play();
}
