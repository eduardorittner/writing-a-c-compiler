use std::ops::Index;

macro_rules! node_type(
    ($type:ident, $typeId:ident) => {

        impl<'a> From<&'a Node> for &'a $type {
            fn from(node: &'a Node) -> &'a $type {
                match node {
                    Node::$type(inner) => inner,
                    _ => panic!(),
                }
            }
        }

        impl Index<$typeId> for Tree {
            type Output = $type;

            // SAFETY: Since the ast is immutable and we only construct valid `NodeId`s, all `NodeId`s
            // are always valid
            fn index(&self, index: $typeId) -> &Self::Output {
                self.nodes.get(index.0).unwrap().into()
            }
        }
    };
);

/// A Node is a enum of all possible AST nodes
///
/// All variants contain a struct with the same node which holds the actual data, this is done so a
/// function can specify exactly what AST nodes it consumes/produces. Any reference to other nodes
/// is done through an Id, which is just the index of that Node inside the `Tree` vector. Ids are
/// also typed to again allow for explictly declaring what each node holds, even though they're all
/// stored contiguously. Trying to access a node as another variant will result in a panic, this
/// should never happen if we construct the AST correctly, since it's immutable.
pub enum Node {
    Program(Program),
    FnDef(FnDef),
    Stmt(Stmt),
    Expr(Expr),
    Ident(Ident),
}

// TODO every node should contain a reference to its underlying token

pub struct Program(FnDefId);

pub struct FnDef {
    name: IdentId,
    body: StmtId,
}

pub enum Expr {
    Constant(i64),
}

pub struct Ident(String); // Todo String should be interned

pub struct NodeId(usize);
pub struct ProgramId(usize);
pub struct FnDefId(usize);
pub struct StmtId(usize);
pub struct ExprId(usize);
pub struct IdentId(usize);

node_type!(Program, ProgramId);
node_type!(FnDef, FnDefId);
node_type!(Stmt, StmtId);
node_type!(Expr, ExprId);

pub enum Stmt {
    Return(ExprId),
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Index<NodeId> for Tree {
    type Output = Node;

    // SAFETY: Since the ast is immutable and we only construct valid `NodeId`s, all `NodeId`s
    // are always valid
    fn index(&self, index: NodeId) -> &Self::Output {
        self.nodes.get(index.0).unwrap()
    }
}
