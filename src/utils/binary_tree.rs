use std::collections::HashSet;

#[derive(Clone)]
pub struct TNode {
    pub left: Option<Box<TNode>>,
    pub right: Option<Box<TNode>>,
    pub operation: Op,
}

impl TNode {
    pub fn new_boxed(op: Op, l: Option<Box<TNode>>, r: Option<Box<TNode>>) -> Self {
        Self {
            left: l,
            right: r,
            operation: op,
        }
    }
    /// New generic node
    pub fn new(op: Op, l: TNode, r: TNode) -> Self {
        Self {
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
            operation: op,
        }
    }
    pub fn new_empty(op: Op) -> Self {
        Self {
            left: None,
            right: None,
            operation: op,
        }
    }
    /// New negation node `¬` or `!`
    pub fn neg_node(l: TNode) -> Self {
        Self {
            left: Some(Box::new(l)),
            right: None,
            operation: Op::Negation,
        }
    }
    /// New conjunction node `∧` or `&`
    pub fn conj_node(l: TNode, r: TNode) -> Self {
        TNode::new(Op::Conjuction, l, r)
    }
    /// New disjunction node `∨` or `|`
    pub fn disj_node(l: TNode, r: TNode) -> Self {
        TNode::new(Op::Disjunction, l, r)
    }
    /// New exclusive disjunction node `⊕` or `^`
    pub fn exl_dis_node(l: TNode, r: TNode) -> Self {
        TNode::new(Op::ExlDisjunction, l, r)
    }
    /// New Material condition node `⇒` or `>`
    ///
    /// https://en.wikipedia.org/wiki/Material_conditional
    pub fn mat_cond_node(l: TNode, r: TNode) -> Self {
        TNode::new(Op::MatCondition, l, r)
    }
    /// New logical equivalence node `⇔` or `=`
    pub fn eq_node(l: TNode, r: TNode) -> Self {
        TNode::new(Op::LogicalEquivalence, l, r)
    }
    /// New boolean node (0 or 1)
    pub fn bool_node(state: bool) -> Self {
        Self {
            left: None,
            right: None,
            operation: Op::Leaf(state),
        }
    }
    /// New char node (A..Z)
    pub fn var_node(state: char) -> Self {
        Self {
            left: None,
            right: None,
            operation: Op::Value(state),
        }
    }
}
/// Operations
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Op {
    Negation,
    Conjuction,
    Disjunction,
    ExlDisjunction,
    MatCondition,
    LogicalEquivalence,
    Leaf(bool),
    Value(char),
    Set(HashSet<i32>),
}

#[derive(Clone)]
pub struct LogicalBT {
    pub head: Option<TNode>,
    string: String,
}

impl LogicalBT {
    pub fn new(head: TNode) -> Self {
        Self {
            head: Some(head),
            string: String::new(),
        }
    }

    pub fn collapse(&self) -> bool {
        match &self.head {
            Some(x) => LogicalBT::collapse_run(&Box::new(x.clone())),
            None => {
                println!("Empty binary tree");
                false
            }
        }
    }

    fn collapse_run(node: &Box<TNode>) -> bool {
        let mut r: Option<bool> = None;
        let mut l: Option<bool> = None;
        // go to the leafs
        if let Some(left) = &node.left {
            l = Some(LogicalBT::collapse_run(left));
        }

        if let Some(right) = &node.right {
            r = Some(LogicalBT::collapse_run(right));
        }
        // get branch values or false if None
        // (case when we're in a leaf and not using r and l)
        let r = if let Some(x) = r { x } else { false };
        let l = if let Some(x) = l { x } else { false };

        match node.operation {
            Op::Negation => !l,
            Op::Conjuction => l && r,
            Op::Disjunction => l || r,
            Op::ExlDisjunction => l ^ r,
            Op::MatCondition => {
                if l && !r {
                    false
                } else {
                    true
                }
            }
            Op::LogicalEquivalence => l == r,
            Op::Leaf(res) => res,
            Op::Value(_) => true,
            Op::Set(_) => true,
        }
    }

    fn fill_string(&mut self, node: &Box<TNode>) {
        if let Some(left) = &node.left {
            Some(self.fill_string(left));
        }
        if let Some(right) = &node.right {
            Some(self.fill_string(right));
        }
        match node.operation {
            Op::Negation => self.string += "!",
            Op::Conjuction => self.string += "&",
            Op::Disjunction => self.string += "|",
            Op::ExlDisjunction => self.string += "^",
            Op::MatCondition => self.string += ">",
            Op::LogicalEquivalence => self.string += "=",
            Op::Leaf(res) => self.string += &(res as u32).to_string(),
            Op::Value(c) => self.string += &c.to_string(),
            Op::Set(_) => {}
        }
    }
    pub fn get_string(&mut self) -> String {
        let head = self.head.as_ref().unwrap().clone();
        self.fill_string(&Box::new(head));
        self.string.clone()
    }
}

pub enum CharType {
    OperationUni,
    OperationBin,
    Boolean,
}
