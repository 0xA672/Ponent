use crate::ast::*;
use crate::hir::symbol::*;
use crate::hir::types::*;

pub struct SymbolProviderImpl<'a> {
    symbols: &'a SymbolTable,
}

impl<'a> SymbolProviderImpl<'a> {
    pub fn new(symbols: &'a SymbolTable) -> Self {
        SymbolProviderImpl { symbols }
    }
}

impl<'a> SymbolProvider for SymbolProviderImpl<'a> {
    fn get_alias_body(&self, def_id: DefId) -> Option<(Vec<TypeParam>, TypeId)> {
        let binding = self.symbols.lookup_type_by_def_id(def_id)?;
        if let TypeKind::Alias = binding.kind {
            let body = binding.alias_body?;
            Some((binding.params.clone(), body))
        } else {
            None
        }
    }

    fn get_struct_definition(&self, def_id: DefId) -> Option<(Vec<TypeParam>, Vec<StructField>)> {
        let binding = self.symbols.lookup_type_by_def_id(def_id)?;
        if let TypeKind::Struct = binding.kind {
            Some((binding.params.clone(), binding.fields.clone()))
        } else {
            None
        }
    }

    fn get_enum_definition(&self, def_id: DefId) -> Option<(Vec<TypeParam>, Vec<EnumVariant>)> {
        let binding = self.symbols.lookup_type_by_def_id(def_id)?;
        if let TypeKind::Enum = binding.kind {
            Some((binding.params.clone(), binding.variants.clone()))
        } else {
            None
        }
    }

    fn is_type_alias(&self, def_id: DefId) -> bool {
        self.symbols
            .lookup_type_by_def_id(def_id)
            .map(|b| b.kind == TypeKind::Alias)
            .unwrap_or(false)
    }
}
