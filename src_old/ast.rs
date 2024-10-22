
enum Node {

}

struct Arg {
    name: String,
    typ: String, // TODO
}

struct ApplyExpr {
    func: String,
    args: Vec<Arg>,
}
enum Expr {
    Apply {
    },
}