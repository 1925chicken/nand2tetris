pub struct SymbolTable {
    table: std::collections::HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let table = std::collections::HashMap::new();
        Self { table }
    }

    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.table.insert(symbol, address);
    }

    pub fn contains(&mut self, symbol: &String) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn get_address(&mut self, symbol: &String) -> Option<&u16> {
        match self.table.contains_key(symbol) {
            true => self.table.get(symbol),
            false => None,
        }
    }
}
