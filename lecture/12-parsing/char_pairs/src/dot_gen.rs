use super::parser::Node;

pub struct DotGen {
    lines: Vec<String>,
    id: usize,
}

impl DotGen {
    pub fn new(node: &Node) -> DotGen {
        let mut dot_gen = DotGen {
            lines: Vec::new(),
            id: 0
        };
        dot_gen.visit(node);
        dot_gen
    }

    pub fn to_string(&self) -> String {
        let mut dot = String::from("digraph {\n");
        for line in &self.lines {
            dot.push_str(&format!("\t{}\n", line));
        }
        dot.push_str("}");
        dot
    }

    fn visit(&mut self, dot: &Node) -> usize {
        match dot {
            Node::Char(c) => self.emit_char(*c),
            Node::Pair(lhs, rhs) => {
                let pair_id = self.emit_pair();
                let lhs_id = self.visit(lhs); 
                let rhs_id = self.visit(rhs); 
                self.emit_edges(pair_id, lhs_id, rhs_id);
                pair_id
            }
        }
    }

    fn emit_pair(&mut self) -> usize {
        let pair_id = self.id;
        self.id += 1;
        self.lines.push(format!("n{} [label=\"<l>|<r>\", shape=\"record\"];", pair_id));
        pair_id
    }

    fn emit_char(&mut self, label: char) -> usize {
        let leaf_id = self.id;
        self.id += 1;
        self.lines.push(format!("n{} [label=\"{}\"];", leaf_id, label));
        leaf_id
    }

    fn emit_edges(&mut self, pair: usize, lhs: usize, rhs: usize) {
        self.lines.push(format!("n{}:l -> n{};", pair, lhs));
        self.lines.push(format!("n{}:r -> n{};", pair, rhs));
    }

}
