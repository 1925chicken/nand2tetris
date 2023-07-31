struct SymbolTable {
    table: std::collections::HashMap<String, u16>,
}

impl SymbolTable {
    fn new() -> Self {
        let table = std::collections::HashMap::new();
        Self { table }
    }

    fn add_entry(&mut self, symbol: String, address: u16) {
        self.table.insert(symbol, address);
    }

    fn contains(&mut self, symbol: &String) -> bool {
        self.table.contains_key(symbol)
    }

    fn get_address(&mut self, symbol: &String) -> Option<&u16> {
        match self.table.contains_key(symbol) {
            true => self.table.get(symbol),
            false => None,
        }
    }
}
