#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Symbol {
    pub id: u64,
    pub name: [char; 8],
}

impl Symbol {
    fn new(id: u64, name: [char; 8]) -> Self {
        Symbol { id, name }
    }
}

// A concrete type that will implement the SymbolCollection trait
pub struct SymbolVector {
    symbols: Vec<Symbol>,
}

fn find_symbol_by_id(symbols: &[Symbol], id: u64) -> Option<&Symbol> {
    symbols.iter().find(|&symbol| symbol.id == id)
}

impl SymbolCollection for Vec<Symbol> {
    fn add(&mut self, symbol: Symbol) -> Result<(), String> {
        self.push(symbol);
        Ok(())
    }

    fn get(&self, id: u64) -> Option<&Symbol> {
        self.iter().find(|&s| s.id == id)
    }

    fn update(&mut self, id: u64, name: [char; 8]) -> Result<(), String> {
        if let Some(symbol) = self.iter_mut().find(|s| s.id == id) {
            symbol.name = name;
            Ok(())
        } else {
            Err(format!("Symbol with id {} not found", id))
        }
    }

    fn remove(&mut self, id: u64) -> Result<Symbol, String> {
        // Logic to remove a Symbol by id
        if let Some(index) = self.iter().position(|s| s.id == id) {
            Ok(self.clone().remove(index))
        } else {
            Err(format!("Symbol with id {} not found", id))
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = &Symbol> + '_> {
        // Logic to return an iterator over the symbols
        Box::new(self.iter())
    }
}

pub trait SymbolPool {
    // Method to create a new Symbol, which will either recycle an existing one or create a new one.
    fn create(&mut self) -> Symbol;

    // Method to release a Symbol back to the pool.
    fn release(&mut self, symbol: Symbol);

    // Method to clear the pool.
    fn clear(&mut self);
}

pub trait SymbolCollection {
    // Add a new Symbol to the collection.
    fn add(&mut self, symbol: Symbol) -> Result<(), String>;
    fn get(&self, id: u64) -> Option<&Symbol>;
    fn update(&mut self, id: u64, name: [char; 8]) -> Result<(), String>;
    fn remove(&mut self, id: u64) -> Result<Symbol, String>;
    fn iter(&self) -> Box<dyn Iterator<Item = &Symbol> + '_>;
}

