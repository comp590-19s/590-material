
pub struct DotGen {
    lines: Vec<String>,
    id: usize,
}

impl DotGen {
    pub fn new() -> DotGen {
        DotGen {
            lines: Vec::new(),
            id: 0
        }
    }

    pub fn emit_pair(&mut self) -> usize {
        let pair_id = self.id;
        self.id += 1;
        self.lines.push(format!("n{} [label=\"<l>|<r>\", shape=\"record\"];", pair_id));
        pair_id
    }

    pub fn emit_char(&mut self, label: char) -> usize {
        let leaf_id = self.id;
        self.id += 1;
        self.lines.push(format!("n{} [label=\"{}\"];", leaf_id, label));
        leaf_id
    }

    pub fn emit_edges(&mut self, pair: usize, lhs: usize, rhs: usize) {
        self.lines.push(format!("n{}:l -> n{};", pair, lhs));
        self.lines.push(format!("n{}:r -> n{};", pair, rhs));
    }

    pub fn to_string(&self) -> String {
        let mut dot = String::from("digraph {\n");
        for line in &self.lines {
            dot.push_str(&format!("\t{}\n", line));
        }
        dot.push_str("}");
        dot
    }
}
