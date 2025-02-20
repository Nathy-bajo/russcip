use crate::scip::ScipPtr;
use crate::{ffi, Col};
use core::panic;
use scip_sys::SCIP_Status;
use std::rc::Rc;

/// A type alias for a variable ID.
pub type VarId = usize;

/// A wrapper for a mutable reference to a SCIP variable.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Variable {
    pub(crate) raw: *mut ffi::SCIP_VAR,
    pub(crate) scip: Rc<ScipPtr>,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.index() == other.index() && self.raw == other.raw
    }
}

impl Eq for Variable {}

impl Variable {
    /// Returns a raw pointer to the underlying `ffi::SCIP_VAR` struct.
    pub fn inner(&self) -> *mut ffi::SCIP_VAR {
        self.raw
    }

    /// Returns the index of the variable.
    pub fn index(&self) -> usize {
        let id = unsafe { ffi::SCIPvarGetIndex(self.raw) };
        assert!(id >= 0);
        id as usize
    }

    /// Returns the name of the variable.
    pub fn name(&self) -> String {
        let name = unsafe { ffi::SCIPvarGetName(self.raw) };
        let name = unsafe { std::ffi::CStr::from_ptr(name) };
        name.to_str().unwrap().to_string()
    }

    /// Returns the objective coefficient of the variable.
    pub fn obj(&self) -> f64 {
        unsafe { ffi::SCIPvarGetObj(self.raw) }
    }

    /// Returns the lower bound of the variable.
    pub fn lb(&self) -> f64 {
        unsafe { ffi::SCIPvarGetLbLocal(self.raw) }
    }

    /// Returns the upper bound of the variable.
    pub fn ub(&self) -> f64 {
        unsafe { ffi::SCIPvarGetUbLocal(self.raw) }
    }

    /// Returns the local lower bound of the variable.
    pub fn lb_local(&self) -> f64 {
        unsafe { ffi::SCIPvarGetLbLocal(self.raw) }
    }

    /// Returns the local upper bound of the variable.
    pub fn ub_local(&self) -> f64 {
        unsafe { ffi::SCIPvarGetUbLocal(self.raw) }
    }

    /// Returns the type of the variable.
    pub fn var_type(&self) -> VarType {
        let var_type = unsafe { ffi::SCIPvarGetType(self.raw) };
        var_type.into()
    }

    /// Returns the status of the variable.
    pub fn status(&self) -> VarStatus {
        let status = unsafe { ffi::SCIPvarGetStatus(self.raw) };
        status.into()
    }

    /// Returns the column associated with the variable.
    pub fn col(&self) -> Option<Col> {
        if self.is_in_lp() {
            let col_ptr = unsafe { ffi::SCIPvarGetCol(self.raw) };
            let col = Col {
                raw: col_ptr,
                scip: Rc::clone(&self.scip),
            };
            Some(col)
        } else {
            None
        }
    }

    /// Returns whether the variable is a column variable in the LP relaxation.
    pub fn is_in_lp(&self) -> bool {
        (unsafe { ffi::SCIPvarIsInLP(self.raw) }) != 0
    }

    /// Returns the solution value of the variable in the current node.
    pub fn sol_val(&self) -> f64 {
        unsafe { ffi::SCIPgetVarSol(self.scip.raw, self.raw) }
    }

    /// Returns whether the variable is deleted.
    pub fn is_deleted(&self) -> bool {
        unsafe { ffi::SCIPvarIsDeleted(self.raw) != 0 }
    }

    /// Returns whether the variable is transformed.
    pub fn is_transformed(&self) -> bool {
        unsafe { ffi::SCIPvarIsTransformed(self.raw) != 0 }
    }

    /// Returns whether the variable is original.
    pub fn is_original(&self) -> bool {
        unsafe { ffi::SCIPvarIsOriginal(self.raw) != 0 }
    }

    /// Returns whether the variable is negated.
    pub fn is_negated(&self) -> bool {
        unsafe { ffi::SCIPvarIsNegated(self.raw) != 0 }
    }

    /// Returns whether the variable is removable (due to aging in the LP).
    pub fn is_removable(&self) -> bool {
        unsafe { ffi::SCIPvarIsRemovable(self.raw) != 0 }
    }

    /// Returns whether the variable is a directed counterpart of an original variable.
    pub fn is_trans_from_orig(&self) -> bool {
        unsafe { ffi::SCIPvarIsTransformedOrigvar(self.raw) != 0 }
    }

    /// Returns whether the variable is active (i.e., neither fixed nor aggregated).
    pub fn is_active(&self) -> bool {
        unsafe { ffi::SCIPvarIsActive(self.raw) != 0 }
    }
}

/// The type of variable in an optimization problem.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VarType {
    /// The variable is a continuous variable.
    Continuous,
    /// The variable is an integer variable.
    Integer,
    /// The variable is a binary variable.
    Binary,
    /// The variable is an implicit integer variable.
    ImplInt,
}

impl From<VarType> for ffi::SCIP_Vartype {
    fn from(var_type: VarType) -> Self {
        match var_type {
            VarType::Continuous => ffi::SCIP_Vartype_SCIP_VARTYPE_CONTINUOUS,
            VarType::Integer => ffi::SCIP_Vartype_SCIP_VARTYPE_INTEGER,
            VarType::Binary => ffi::SCIP_Vartype_SCIP_VARTYPE_BINARY,
            VarType::ImplInt => ffi::SCIP_Vartype_SCIP_VARTYPE_IMPLINT,
        }
    }
}

impl From<ffi::SCIP_Vartype> for VarType {
    fn from(var_type: ffi::SCIP_Vartype) -> Self {
        match var_type {
            ffi::SCIP_Vartype_SCIP_VARTYPE_CONTINUOUS => VarType::Continuous,
            ffi::SCIP_Vartype_SCIP_VARTYPE_INTEGER => VarType::Integer,
            ffi::SCIP_Vartype_SCIP_VARTYPE_BINARY => VarType::Binary,
            ffi::SCIP_Vartype_SCIP_VARTYPE_IMPLINT => VarType::ImplInt,
            _ => panic!("Unknown VarType {:?}", var_type),
        }
    }
}

/// An enum representing the status of a SCIP variable.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VarStatus {
    /// The variable is an original variable in the problem.
    Original,
    /// The variable is a loose variable in the problem.
    Loose,
    /// The variable is a column variable in the problem.
    Column,
    /// The variable is a fixed variable in the problem.
    Fixed,
    /// The variable is an aggregated variable in the problem.
    Aggregated,
    /// The variable is a multi-aggregated variable in the problem.
    MultiAggregated,
    /// The variable is a negated variable in the problem.
    NegatedVar,
}

impl From<SCIP_Status> for VarStatus {
    fn from(status: SCIP_Status) -> Self {
        match status {
            ffi::SCIP_Varstatus_SCIP_VARSTATUS_ORIGINAL => VarStatus::Original,
            ffi::SCIP_Varstatus_SCIP_VARSTATUS_LOOSE => VarStatus::Loose,
            ffi::SCIP_Varstatus_SCIP_VARSTATUS_COLUMN => VarStatus::Column,
            ffi::SCIP_Varstatus_SCIP_VARSTATUS_FIXED => VarStatus::Fixed,
            ffi::SCIP_Varstatus_SCIP_VARSTATUS_AGGREGATED => VarStatus::Aggregated,
            ffi::SCIP_Varstatus_SCIP_VARSTATUS_MULTAGGR => VarStatus::MultiAggregated,
            ffi::SCIP_Varstatus_SCIP_VARSTATUS_NEGATED => VarStatus::NegatedVar,
            _ => panic!("Unhandled SCIP variable status {:?}", status),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{minimal_model, Model, ObjSense, ProblemOrSolving};

    #[test]
    fn var_data() {
        let mut model = Model::new().include_default_plugins().create_prob("test");
        let var = model.add_var(0.0, 1.0, 2.0, "x", VarType::ImplInt);

        assert_eq!(var.index(), 0);
        assert_eq!(var.lb(), 0.0);
        assert_eq!(var.lb_local(), 0.0);
        assert_eq!(var.ub(), 1.0);
        assert_eq!(var.ub_local(), 1.0);
        assert_eq!(var.obj(), 2.0);
        assert_eq!(var.name(), "x");
        assert_eq!(var.var_type(), VarType::ImplInt);
        assert_eq!(var.status(), VarStatus::Original);
        assert!(!var.is_in_lp());
        assert!(!var.is_deleted());
        assert!(!var.is_transformed());
        assert!(var.is_original());
        assert!(!var.is_negated());
        assert!(!var.is_removable());
        assert!(var.is_active());

        assert!(!var.inner().is_null());
    }

    #[test]
    fn var_memory_safety() {
        let mut model = Model::new()
            .hide_output()
            .include_default_plugins()
            .create_prob("test")
            .set_obj_sense(ObjSense::Maximize);

        let x1 = model.add_var(0., f64::INFINITY, 3., "x1", VarType::Integer);

        drop(model);
        assert_eq!(x1.name(), "x1");
    }

    #[test]
    fn var_sol_val() {
        let mut model = minimal_model();
        let x = model.add_var(0.0, 1.0, 1.0, "x", VarType::Binary);
        let _cons = model.add_cons(vec![&x], &[1.0], 1.0, 1.0, "cons1");

        model.solve();

        assert_eq!(x.sol_val(), 1.0);
    }
}
