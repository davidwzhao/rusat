#[derive(Clone,Debug)]
pub enum AstNodeType {
    // A list of disjuncted ConjunctionClauses
    DisjunctionClause,

    // A list of conjuncted Values
    ConjunctionClause,

    // A value is either a (negated) Variable or a nested DisjunctionClause
    Value,

    // A negation
    Negation,

    // A variable
    Variable(String)
}

#[derive(Clone,Debug)]
pub struct AstNode {
    entry: AstNodeType,
    children: Vec<AstNode>,
}

impl AstNode {
    pub fn new(e: AstNodeType) -> AstNode {
        AstNode {
            entry: e,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, c: AstNode) {
        self.children.push(c);
    }

    pub fn to_string(&self) -> String {
        let mut representation: String;

        // get the representations of children
        let children_strings: Vec<String> = self.children.iter().map(|x| x.to_string()).collect();

        use AstNodeType::*;
        match &self.entry {
            DisjunctionClause => {
                representation = "(".to_owned() + &children_strings.join(" | ") + ")";
            }

            ConjunctionClause => {
                representation = "(".to_owned() + &children_strings.join(" & ") + ")";
            }

            Value => {
                representation = children_strings.join("");
            }

            Negation => {
                representation = "~".to_string();
            }

            Variable(s) => {
                representation = s.to_string();
            }
        }

        representation
    }
}
