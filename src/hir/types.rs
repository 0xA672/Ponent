use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeData {
    Int {
        bits: u8,
        signed: bool,
    },
    Float {
        bits: u8,
    },
    Bool,
    Char,
    Byte,
    USize,
    Struct {
        def_id: DefId,
        args: Vec<TypeId>,
    },
    Enum {
        def_id: DefId,
        args: Vec<TypeId>,
    },
    Tuple {
        elems: Vec<TypeId>,
    },
    Array {
        elem: TypeId,
        size: u64,
    },
    Slice {
        elem: TypeId,
    },
    Ref {
        ty: TypeId,
        mutable: bool,
    },
    Pointer {
        ty: TypeId,
    },
    Ptr {
        size: TypeId,
        pointee: TypeId,
    },
    Fn {
        params: Vec<TypeId>,
        ret: TypeId,
    },
    DynTrait {
        traits: Vec<DefId>,
    },
    Exists {
        name: String,
        base: TypeId,
    },
    GenericParam {
        index: usize,
        name: String,
    },
    AssociatedType {
        trait_id: DefId,
        name: String,
        self_ty: TypeId,
    },
    Never,
    Unit,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefId(pub usize);

pub struct TypeContext {
    types: Vec<Arc<TypeData>>,
    type_map: HashMap<TypeData, TypeId>,
    bindings: HashMap<TypeId, TypeId>,
    invariants: HashMap<TypeId, Expr>,
    def_id_to_type_id: HashMap<DefId, TypeId>,
}

impl TypeContext {
    pub fn new() -> Self {
        let mut ctx = TypeContext {
            types: Vec::new(),
            type_map: HashMap::new(),
            bindings: HashMap::new(),
            invariants: HashMap::new(),
            def_id_to_type_id: HashMap::new(),
        };
        let unit = ctx.alloc(TypeData::Unit);
        let never = ctx.alloc(TypeData::Never);
        let error = ctx.alloc(TypeData::Error);
        let bool_ty = ctx.alloc(TypeData::Bool);
        let char_ty = ctx.alloc(TypeData::Char);
        let byte_ty = ctx.alloc(TypeData::Byte);
        let usize_ty = ctx.alloc(TypeData::USize);
        ctx
    }

    pub fn get_invariant(&self, id: TypeId) -> Option<&Expr> {
        self.invariants.get(&id)
    }

    pub fn alloc(&mut self, data: TypeData) -> TypeId {
        if let Some(&id) = self.type_map.get(&data) {
            return id;
        }
        let id = TypeId(self.types.len());
        self.types.push(Arc::new(data.clone()));
        self.type_map.insert(data, id);
        id
    }

    pub fn get(&mut self, id: TypeId) -> &TypeData {
        self.resolve_binding(id)
    }

    fn resolve_binding(&mut self, id: TypeId) -> &TypeData {
        if let Some(&bound) = self.bindings.get(&id) {
            let root = self.resolve_binding(bound);
            if root != bound {
                self.bindings.insert(id, root);
            }
            root
        } else {
            &self.types[id.0]
        }
    }

    pub fn get_def_id_for_type(&self, id: TypeId) -> Option<DefId> {
        match self.resolve_binding(id) {
            TypeData::Struct { def_id, .. } => Some(*def_id),
            TypeData::Enum { def_id, .. } => Some(*def_id),
            _ => None,
        }
    }

    pub fn register_def_id(&mut self, def_id: DefId, type_id: TypeId) {
        self.def_id_to_type_id.insert(def_id, type_id);
    }

    pub fn get_type_id_for_def_id(&self, def_id: DefId) -> Option<TypeId> {
        self.def_id_to_type_id.get(&def_id).copied()
    }

    pub fn int(&mut self, bits: u8, signed: bool) -> TypeId {
        self.alloc(TypeData::Int { bits, signed })
    }

    pub fn uint(&mut self, bits: u8) -> TypeId {
        self.int(bits, false)
    }

    pub fn float(&mut self, bits: u8) -> TypeId {
        self.alloc(TypeData::Float { bits })
    }

    pub fn bool(&self) -> TypeId {
        TypeId(3)
    }

    pub fn char(&self) -> TypeId {
        TypeId(4)
    }

    pub fn byte(&self) -> TypeId {
        TypeId(5)
    }

    pub fn usize(&self) -> TypeId {
        TypeId(6)
    }

    pub fn unit(&self) -> TypeId {
        TypeId(0)
    }

    pub fn never(&self) -> TypeId {
        TypeId(1)
    }

    pub fn error(&self) -> TypeId {
        TypeId(2)
    }

    pub fn struct_ty(&mut self, def_id: DefId, args: Vec<TypeId>) -> TypeId {
        let id = self.alloc(TypeData::Struct { def_id, args });
        self.def_id_to_type_id.insert(def_id, id);
        id
    }

    pub fn enum_ty(&mut self, def_id: DefId, args: Vec<TypeId>) -> TypeId {
        let id = self.alloc(TypeData::Enum { def_id, args });
        self.def_id_to_type_id.insert(def_id, id);
        id
    }

    pub fn tuple(&mut self, elems: Vec<TypeId>) -> TypeId {
        self.alloc(TypeData::Tuple { elems })
    }

    pub fn array(&mut self, elem: TypeId, size: u64) -> TypeId {
        self.alloc(TypeData::Array { elem, size })
    }

    pub fn slice(&mut self, elem: TypeId) -> TypeId {
        self.alloc(TypeData::Slice { elem })
    }

    pub fn reference(&mut self, ty: TypeId, mutable: bool) -> TypeId {
        self.alloc(TypeData::Ref { ty, mutable })
    }

    pub fn pointer(&mut self, ty: TypeId) -> TypeId {
        self.alloc(TypeData::Pointer { ty })
    }

    pub fn ptr(&mut self, size: TypeId, pointee: TypeId) -> TypeId {
        self.alloc(TypeData::Ptr { size, pointee })
    }

    pub fn function(&mut self, params: Vec<TypeId>, ret: TypeId) -> TypeId {
        self.alloc(TypeData::Fn { params, ret })
    }

    pub fn dyn_trait(&mut self, traits: Vec<DefId>) -> TypeId {
        self.alloc(TypeData::DynTrait { traits })
    }

    pub fn exists(&mut self, name: String, base: TypeId, invariant: Expr) -> TypeId {
        let id = self.alloc(TypeData::Exists { name, base });
        self.invariants.insert(id, invariant);
        id
    }

    pub fn generic_param(&mut self, index: usize, name: String) -> TypeId {
        self.alloc(TypeData::GenericParam { index, name })
    }

    pub fn associated_type(&mut self, trait_id: DefId, name: String, self_ty: TypeId) -> TypeId {
        self.alloc(TypeData::AssociatedType {
            trait_id,
            name,
            self_ty,
        })
    }

    fn occurs_check(&mut self, param: TypeId, ty: TypeId) -> bool {
        if param == ty {
            return true;
        }
        match self.resolve_binding(ty) {
            TypeData::Struct { args, .. } | TypeData::Enum { args, .. } => {
                args.iter().any(|&a| self.occurs_check(param, a))
            }
            TypeData::Tuple { elems } => elems.iter().any(|&e| self.occurs_check(param, e)),
            TypeData::Array { elem, .. } => self.occurs_check(param, *elem),
            TypeData::Slice { elem } => self.occurs_check(param, *elem),
            TypeData::Ref { ty, .. } => self.occurs_check(param, *ty),
            TypeData::Pointer { ty } => self.occurs_check(param, *ty),
            TypeData::Ptr { size, pointee } => {
                self.occurs_check(param, *size) || self.occurs_check(param, *pointee)
            }
            TypeData::Fn { params, ret } => {
                params.iter().any(|&p| self.occurs_check(param, p))
                    || self.occurs_check(param, *ret)
            }
            TypeData::Exists { base, .. } => self.occurs_check(param, *base),
            TypeData::AssociatedType { self_ty, .. } => self.occurs_check(param, *self_ty),
            TypeData::GenericParam { .. } => false,
            TypeData::Int { .. }
            | TypeData::Float { .. }
            | TypeData::Bool
            | TypeData::Char
            | TypeData::Byte
            | TypeData::USize
            | TypeData::Never
            | TypeData::Unit
            | TypeData::Error
            | TypeData::DynTrait { .. } => false,
        }
    }

    pub fn unify(&mut self, a: TypeId, b: TypeId) -> Result<TypeId, TypeError> {
        let data_a = self.resolve_binding(a).clone();
        let data_b = self.resolve_binding(b).clone();

        if data_a == data_b {
            return Ok(a);
        }

        match (&data_a, &data_b) {
            (TypeData::Error, _) => Ok(b),
            (_, TypeData::Error) => Ok(a),
            (
                TypeData::GenericParam { index: i1, .. },
                TypeData::GenericParam { index: i2, .. },
            ) if i1 == i2 => Ok(a),
            (TypeData::GenericParam { .. }, _) => {
                if self.occurs_check(a, b) {
                    return Err(TypeError::RecursiveType {
                        ty: a,
                        span: Span::new(0, 0),
                    });
                }
                self.bindings.insert(a, b);
                Ok(b)
            }
            (_, TypeData::GenericParam { .. }) => {
                if self.occurs_check(b, a) {
                    return Err(TypeError::RecursiveType {
                        ty: b,
                        span: Span::new(0, 0),
                    });
                }
                self.bindings.insert(b, a);
                Ok(a)
            }
            _ => Err(TypeError::Mismatch {
                expected: b,
                found: a,
                span: Span::new(0, 0),
            }),
        }
    }

    pub fn subtype(&mut self, sub: TypeId, sup: TypeId) -> bool {
        if sub == sup {
            return true;
        }

        let sub_data = self.resolve_binding(sub);
        let sup_data = self.resolve_binding(sup);

        match (sub_data, sup_data) {
            (TypeData::Error, _) => true,
            (_, TypeData::Error) => true,
            (TypeData::Never, _) => true,
            (TypeData::Unit, TypeData::Unit) => true,
            (
                TypeData::Ref {
                    ty: t1,
                    mutable: m1,
                },
                TypeData::Ref {
                    ty: t2,
                    mutable: m2,
                },
            ) => {
                if *m1 == *m2 {
                    self.subtype(*t1, *t2)
                } else {
                    false
                }
            }
            (
                TypeData::Fn {
                    params: p1,
                    ret: r1,
                },
                TypeData::Fn {
                    params: p2,
                    ret: r2,
                },
            ) => {
                if p1.len() != p2.len() {
                    return false;
                }
                for (a, b) in p1.iter().zip(p2.iter()) {
                    if !self.subtype(*a, *b) {
                        return false;
                    }
                }
                self.subtype(*r1, *r2)
            }
            (TypeData::Array { elem: e1, size: s1 }, TypeData::Array { elem: e2, size: s2 }) => {
                *s1 == *s2 && self.subtype(*e1, *e2)
            }
            (TypeData::Slice { elem: e1 }, TypeData::Slice { elem: e2 }) => self.subtype(*e1, *e2),
            (TypeData::Tuple { elems: e1 }, TypeData::Tuple { elems: e2 }) => {
                if e1.len() != e2.len() {
                    return false;
                }
                e1.iter().zip(e2.iter()).all(|(a, b)| self.subtype(*a, *b))
            }
            (
                TypeData::Int {
                    bits: b1,
                    signed: s1,
                },
                TypeData::Int {
                    bits: b2,
                    signed: s2,
                },
            ) => *s1 == *s2 && *b1 == *b2,
            (TypeData::Float { bits: b1 }, TypeData::Float { bits: b2 }) => *b1 == *b2,
            _ => false,
        }
    }

    fn find_type(&self, data: &TypeData) -> Option<TypeId> {
        self.type_map.get(data).copied()
    }

    pub fn subst(&mut self, ty: TypeId, subst: &Subst) -> TypeId {
        match self.resolve_binding(ty) {
            TypeData::GenericParam { index, .. } => subst.get(*index).copied().unwrap_or(ty),
            TypeData::Int { bits, signed } => {
                let data = TypeData::Int {
                    bits: *bits,
                    signed: *signed,
                };
                self.find_type(&data)
                    .expect("built-in Int type should exist")
            }
            TypeData::Float { bits } => {
                let data = TypeData::Float { bits: *bits };
                self.find_type(&data)
                    .expect("built-in Float type should exist")
            }
            TypeData::Bool
            | TypeData::Char
            | TypeData::Byte
            | TypeData::USize
            | TypeData::Never
            | TypeData::Unit
            | TypeData::Error => ty,
            TypeData::Struct { def_id, args } => {
                let new_args: Vec<TypeId> = args.iter().map(|&a| self.subst(a, subst)).collect();
                self.struct_ty_no_alloc(*def_id, new_args)
                    .expect("struct type should exist")
            }
            TypeData::Enum { def_id, args } => {
                let new_args: Vec<TypeId> = args.iter().map(|&a| self.subst(a, subst)).collect();
                self.enum_ty_no_alloc(*def_id, new_args)
                    .expect("enum type should exist")
            }
            TypeData::Tuple { elems } => {
                let new_elems: Vec<TypeId> = elems.iter().map(|&e| self.subst(e, subst)).collect();
                self.tuple_ty_no_alloc(new_elems)
                    .expect("tuple type should exist")
            }
            TypeData::Array { elem, size } => {
                let new_elem = self.subst(*elem, subst);
                self.array_ty_no_alloc(new_elem, *size)
                    .expect("array type should exist")
            }
            TypeData::Slice { elem } => {
                let new_elem = self.subst(*elem, subst);
                self.slice_ty_no_alloc(new_elem)
                    .expect("slice type should exist")
            }
            TypeData::Ref { ty, mutable } => {
                let new_ty = self.subst(*ty, subst);
                self.ref_ty_no_alloc(new_ty, *mutable)
                    .expect("ref type should exist")
            }
            TypeData::Pointer { ty } => {
                let new_ty = self.subst(*ty, subst);
                self.pointer_ty_no_alloc(new_ty)
                    .expect("pointer type should exist")
            }
            TypeData::Ptr { size, pointee } => {
                let new_size = self.subst(*size, subst);
                let new_pointee = self.subst(*pointee, subst);
                self.ptr_ty_no_alloc(new_size, new_pointee)
                    .expect("ptr type should exist")
            }
            TypeData::Fn { params, ret } => {
                let new_params: Vec<TypeId> =
                    params.iter().map(|&p| self.subst(p, subst)).collect();
                let new_ret = self.subst(*ret, subst);
                self.fn_ty_no_alloc(new_params, new_ret)
                    .expect("function type should exist")
            }
            TypeData::DynTrait { traits } => ty,
            TypeData::Exists { name, base } => {
                let new_base = self.subst(*base, subst);
                self.exists_ty_no_alloc(name.clone(), new_base)
                    .expect("exists type should exist")
            }
            TypeData::AssociatedType {
                trait_id,
                name,
                self_ty,
            } => {
                let new_self = self.subst(*self_ty, subst);
                self.associated_ty_no_alloc(*trait_id, name.clone(), new_self)
                    .expect("associated type should exist")
            }
            _ => ty,
        }
    }

    fn struct_ty_no_alloc(&self, def_id: DefId, args: Vec<TypeId>) -> Option<TypeId> {
        self.find_type(&TypeData::Struct { def_id, args })
    }

    fn enum_ty_no_alloc(&self, def_id: DefId, args: Vec<TypeId>) -> Option<TypeId> {
        self.find_type(&TypeData::Enum { def_id, args })
    }

    fn tuple_ty_no_alloc(&self, elems: Vec<TypeId>) -> Option<TypeId> {
        self.find_type(&TypeData::Tuple { elems })
    }

    fn array_ty_no_alloc(&self, elem: TypeId, size: u64) -> Option<TypeId> {
        self.find_type(&TypeData::Array { elem, size })
    }

    fn slice_ty_no_alloc(&self, elem: TypeId) -> Option<TypeId> {
        self.find_type(&TypeData::Slice { elem })
    }

    fn ref_ty_no_alloc(&self, ty: TypeId, mutable: bool) -> Option<TypeId> {
        self.find_type(&TypeData::Ref { ty, mutable })
    }

    fn pointer_ty_no_alloc(&self, ty: TypeId) -> Option<TypeId> {
        self.find_type(&TypeData::Pointer { ty })
    }

    fn ptr_ty_no_alloc(&self, size: TypeId, pointee: TypeId) -> Option<TypeId> {
        self.find_type(&TypeData::Ptr { size, pointee })
    }

    fn fn_ty_no_alloc(&self, params: Vec<TypeId>, ret: TypeId) -> Option<TypeId> {
        self.find_type(&TypeData::Fn { params, ret })
    }

    fn exists_ty_no_alloc(&self, name: String, base: TypeId) -> Option<TypeId> {
        self.find_type(&TypeData::Exists { name, base })
    }

    fn associated_ty_no_alloc(
        &self,
        trait_id: DefId,
        name: String,
        self_ty: TypeId,
    ) -> Option<TypeId> {
        self.find_type(&TypeData::AssociatedType {
            trait_id,
            name,
            self_ty,
        })
    }

    pub fn is_numeric(&mut self, ty: TypeId) -> bool {
        match self.resolve_binding(ty) {
            TypeData::Int { .. } | TypeData::Float { .. } => true,
            _ => false,
        }
    }

    pub fn is_integer(&mut self, ty: TypeId) -> bool {
        match self.resolve_binding(ty) {
            TypeData::Int { .. } | TypeData::USize => true,
            _ => false,
        }
    }

    pub fn is_unsigned(&mut self, ty: TypeId) -> bool {
        match self.resolve_binding(ty) {
            TypeData::Int { signed, .. } => !signed,
            TypeData::USize => true,
            _ => false,
        }
    }

    pub fn is_signed(&mut self, ty: TypeId) -> bool {
        match self.resolve_binding(ty) {
            TypeData::Int { signed, .. } => *signed,
            _ => false,
        }
    }

    pub fn is_float(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Float { .. })
    }

    pub fn is_bool(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Bool)
    }

    pub fn is_char(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Char)
    }

    pub fn is_byte(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Byte)
    }

    pub fn is_usize(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::USize)
    }

    pub fn is_unit(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Unit)
    }

    pub fn is_never(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Never)
    }

    pub fn is_error(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Error)
    }

    pub fn is_reference(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Ref { .. })
    }

    pub fn is_pointer(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Pointer { .. })
    }

    pub fn is_struct(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Struct { .. })
    }

    pub fn is_enum(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Enum { .. })
    }

    pub fn is_tuple(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Tuple { .. })
    }

    pub fn is_array(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Array { .. })
    }

    pub fn is_slice(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Slice { .. })
    }

    pub fn is_fn(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Fn { .. })
    }

    pub fn is_dyn_trait(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::DynTrait { .. })
    }

    pub fn is_exists(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::Exists { .. })
    }

    pub fn is_generic_param(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::GenericParam { .. })
    }

    pub fn is_associated_type(&mut self, ty: TypeId) -> bool {
        matches!(self.resolve_binding(ty), TypeData::AssociatedType { .. })
    }

    pub fn bits_of_int(&mut self, ty: TypeId) -> Option<u8> {
        match self.resolve_binding(ty) {
            TypeData::Int { bits, .. } => Some(*bits),
            _ => None,
        }
    }

    pub fn signedness_of_int(&mut self, ty: TypeId) -> Option<bool> {
        match self.resolve_binding(ty) {
            TypeData::Int { signed, .. } => Some(*signed),
            _ => None,
        }
    }

    pub fn bits_of_float(&mut self, ty: TypeId) -> Option<u8> {
        match self.resolve_binding(ty) {
            TypeData::Float { bits } => Some(*bits),
            _ => None,
        }
    }

    pub fn size_of_array(&mut self, ty: TypeId) -> Option<u64> {
        match self.resolve_binding(ty) {
            TypeData::Array { size, .. } => Some(*size),
            _ => None,
        }
    }

    pub fn elem_of_array(&mut self, ty: TypeId) -> Option<TypeId> {
        match self.resolve_binding(ty) {
            TypeData::Array { elem, .. } => Some(*elem),
            _ => None,
        }
    }

    pub fn elem_of_slice(&mut self, ty: TypeId) -> Option<TypeId> {
        match self.resolve_binding(ty) {
            TypeData::Slice { elem } => Some(*elem),
            _ => None,
        }
    }

    pub fn pointee_of_ref(&mut self, ty: TypeId) -> Option<TypeId> {
        match self.resolve_binding(ty) {
            TypeData::Ref { ty: t, .. } => Some(*t),
            _ => None,
        }
    }

    pub fn mutability_of_ref(&mut self, ty: TypeId) -> Option<bool> {
        match self.resolve_binding(ty) {
            TypeData::Ref { mutable, .. } => Some(*mutable),
            _ => None,
        }
    }

    pub fn pointee_of_pointer(&mut self, ty: TypeId) -> Option<TypeId> {
        match self.resolve_binding(ty) {
            TypeData::Pointer { ty: t } => Some(*t),
            _ => None,
        }
    }

    pub fn params_of_fn(&mut self, ty: TypeId) -> Option<&[TypeId]> {
        match self.resolve_binding(ty) {
            TypeData::Fn { params, .. } => Some(params),
            _ => None,
        }
    }

    pub fn ret_of_fn(&mut self, ty: TypeId) -> Option<TypeId> {
        match self.resolve_binding(ty) {
            TypeData::Fn { ret, .. } => Some(*ret),
            _ => None,
        }
    }

    pub fn tuple_elems(&mut self, ty: TypeId) -> Option<&[TypeId]> {
        match self.resolve_binding(ty) {
            TypeData::Tuple { elems } => Some(elems),
            _ => None,
        }
    }

    pub fn base_of_exists(&mut self, ty: TypeId) -> Option<TypeId> {
        match self.resolve_binding(ty) {
            TypeData::Exists { base, .. } => Some(*base),
            _ => None,
        }
    }

    pub fn name_of_exists(&mut self, ty: TypeId) -> Option<&String> {
        match self.resolve_binding(ty) {
            TypeData::Exists { name, .. } => Some(name),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Subst {
    map: HashMap<usize, TypeId>,
}

impl Subst {
    pub fn new() -> Self {
        Subst {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, index: usize, ty: TypeId) {
        self.map.insert(index, ty);
    }

    pub fn get(&self, index: usize) -> Option<&TypeId> {
        self.map.get(&index)
    }

    pub fn extend(&mut self, other: &Subst) {
        for (&k, &v) in other.map.iter() {
            self.map.insert(k, v);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Default for Subst {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum TypeError {
    Mismatch {
        expected: TypeId,
        found: TypeId,
        span: Span,
    },
    UndefinedName {
        name: String,
        span: Span,
        suggestions: Vec<String>,
    },
    TypeNotFound {
        name: String,
        span: Span,
    },
    CannotInfer {
        span: Span,
    },
    GenericArgumentCount {
        expected: usize,
        found: usize,
        span: Span,
    },
    TraitNotImplemented {
        ty: TypeId,
        trait_name: String,
        span: Span,
    },
    InvariantViolation {
        ty: TypeId,
        expr: String,
        span: Span,
    },
    MutableBorrow {
        span: Span,
    },
    ImmutableBorrow {
        span: Span,
    },
    OutOfBounds {
        index: u64,
        size: u64,
        span: Span,
    },
    DivisionByZero {
        span: Span,
    },
    Overflow {
        span: Span,
    },
    NeverType {
        span: Span,
    },
    CircularDependency {
        name: String,
        span: Span,
    },
    DuplicateDefinition {
        name: String,
        span: Span,
        previous: Span,
    },
    PrivateField {
        name: String,
        span: Span,
    },
    PrivateType {
        name: String,
        span: Span,
    },
    PrivateFunction {
        name: String,
        span: Span,
    },
    PatternNotExhaustive {
        span: Span,
    },
    PatternRedundant {
        span: Span,
    },
    PatternTypeMismatch {
        expected: TypeId,
        found: TypeId,
        span: Span,
    },
    RecursiveType {
        ty: TypeId,
        span: Span,
    },
}

use crate::ast::Expr;
use crate::ast::Span;
