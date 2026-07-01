use super::*;

#[derive(Debug, Clone)]
pub struct Guarantee {
    /// The ensures condition — a `bool`-typed expression that must hold
    /// at every return point of the function.
    pub postcondition: Option<TypeId>,
    /// The set of types / memory regions that are guaranteed to be preserved.
    pub frame: Option<TypeId>,
}

/// A chain of SCAP guarantees, representing the logical control stack.
/// At depth 0 we are in the outermost function which has no return pointer;
/// deeper entries correspond to nested call sites awaiting return.
#[derive(Debug, Clone)]
pub struct GuaranteeChain {
    pub stack: Vec<Guarantee>,
}

impl GuaranteeChain {
    pub fn new() -> Self {
        GuaranteeChain { stack: Vec::new() }
    }

    /// Push a callee's guarantee onto the chain (SCAP CALL rule).
    pub fn push(&mut self, g: Guarantee) {
        self.stack.push(g);
    }

    /// Pop the innermost guarantee on return (SCAP RET rule).
    pub fn pop(&mut self) -> Option<Guarantee> {
        self.stack.pop()
    }

    /// The current (innermost) guarantee, if any.
    pub fn current(&self) -> Option<&Guarantee> {
        self.stack.last()
    }
}

