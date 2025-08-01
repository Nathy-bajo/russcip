use crate::row::BasisStatus;
use crate::row::Row;
use crate::scip::ScipPtr;
use crate::{Variable, ffi};
use std::rc::Rc;

/// A column in the LP relaxation.u
#[derive(Debug, Clone)]
pub struct Col {
    pub(crate) raw: *mut ffi::SCIP_COL,
    pub(crate) scip: Rc<ScipPtr>,
}

impl Col {
    /// Returns a raw pointer to the underlying `ffi::SCIP_COL` struct.
    pub fn inner(&self) -> *mut ffi::SCIP_COL {
        self.raw
    }

    /// Returns the index of the column.
    pub fn index(&self) -> usize {
        let id = unsafe { ffi::SCIPcolGetIndex(self.raw) };
        assert!(id >= 0);
        id as usize
    }

    /// Returns the objective coefficient of the column.
    pub fn obj(&self) -> f64 {
        unsafe { ffi::SCIPcolGetObj(self.raw) }
    }

    /// Returns the lower bound of the column.
    pub fn lb(&self) -> f64 {
        unsafe { ffi::SCIPcolGetLb(self.raw) }
    }

    /// Returns the upper bound of the column.
    pub fn ub(&self) -> f64 {
        unsafe { ffi::SCIPcolGetUb(self.raw) }
    }

    /// Returns the best bound of the column with respect to the objective function.
    pub fn best_bound(&self) -> f64 {
        unsafe { ffi::SCIPcolGetBestBound(self.raw) }
    }

    /// Returns the variable associated with the column.
    pub fn var(&self) -> Variable {
        let var_ptr = unsafe { ffi::SCIPcolGetVar(self.raw) };

        Variable {
            raw: var_ptr,
            scip: Rc::clone(&self.scip),
        }
    }

    /// Returns the primal LP solution of the column.
    pub fn primal_sol(&self) -> f64 {
        unsafe { ffi::SCIPcolGetPrimsol(self.raw) }
    }

    /// Returns the minimal LP solution value, this column ever assumed.
    pub fn min_primal_sol(&self) -> f64 {
        unsafe { ffi::SCIPcolGetMinPrimsol(self.raw) }
    }

    /// Returns the maximal LP solution value, this column ever assumed.
    pub fn max_primal_sol(&self) -> f64 {
        unsafe { ffi::SCIPcolGetMaxPrimsol(self.raw) }
    }

    /// Returns the basis status of a column in the LP solution.
    pub fn basis_status(&self) -> BasisStatus {
        unsafe { ffi::SCIPcolGetBasisStatus(self.raw) }.into()
    }

    /// Returns the probindex of the corresponding variable.
    pub fn var_probindex(&self) -> Option<usize> {
        let probindex = unsafe { ffi::SCIPcolGetVarProbindex(self.raw) };
        if probindex < 0 {
            None
        } else {
            Some(probindex as usize)
        }
    }

    /// Returns whether the column is of integral type.
    pub fn is_integral(&self) -> bool {
        (unsafe { ffi::SCIPcolIsIntegral(self.raw) }) != 0
    }

    /// Returns whether the column is removable from the LP.
    pub fn is_removable(&self) -> bool {
        (unsafe { ffi::SCIPcolIsRemovable(self.raw) }) != 0
    }

    /// Returns the position of the column in the current LP.
    pub fn lp_pos(&self) -> Option<usize> {
        let pos = unsafe { ffi::SCIPcolGetLPPos(self.raw) };
        if pos < 0 { None } else { Some(pos as usize) }
    }

    /// Returns the depth in the tree where the column entered the LP.
    pub fn lp_depth(&self) -> Option<usize> {
        let depth = unsafe { ffi::SCIPcolGetLPDepth(self.raw) };
        if depth < 0 {
            None
        } else {
            Some(depth as usize)
        }
    }

    /// Returns whether the column is in the current LP.
    pub fn is_in_lp(&self) -> bool {
        (unsafe { ffi::SCIPcolIsInLP(self.raw) }) != 0
    }

    /// Returns the number of non-zero entries.
    pub fn n_non_zeros(&self) -> usize {
        let n_non_zeros = unsafe { ffi::SCIPcolGetNNonz(self.raw) };
        assert!(n_non_zeros >= 0);
        n_non_zeros as usize
    }

    /// Returns the number of non-zero entries that correspond to rows currently in the LP.
    pub fn n_lp_non_zeros(&self) -> usize {
        let n_lp_non_zeros = unsafe { ffi::SCIPcolGetNLPNonz(self.raw) };
        assert!(n_lp_non_zeros >= 0);
        n_lp_non_zeros as usize
    }

    /// Returns the rows of non-zero entries.
    pub fn rows(&self) -> Vec<Row> {
        let n_non_zeros = self.n_non_zeros();
        let rows_ptr = unsafe { ffi::SCIPcolGetRows(self.raw) };
        let rows = unsafe { std::slice::from_raw_parts(rows_ptr, n_non_zeros) };
        rows.iter()
            .map(|&row_ptr| Row {
                raw: row_ptr,
                scip: Rc::clone(&self.scip),
            })
            .collect()
    }

    /// Returns the coefficients of non-zero entries.
    pub fn vals(&self) -> Vec<f64> {
        let n_non_zeros = self.n_non_zeros();
        let vals_ptr = unsafe { ffi::SCIPcolGetVals(self.raw) };
        let vals = unsafe { std::slice::from_raw_parts(vals_ptr, n_non_zeros) };
        vals.to_vec()
    }

    /// Returns the node number of the last node in current branch and bound run, where strong branching was used on the given column.
    pub fn strong_branching_node(&self) -> Option<i64> {
        let node = unsafe { ffi::SCIPcolGetStrongbranchNode(self.raw) };
        if node < 0 { None } else { Some(node) }
    }

    /// Returns the number of times, strong branching was applied in current run on the given column.
    pub fn n_strong_branches(&self) -> usize {
        let n_strong_branches = unsafe { ffi::SCIPcolGetNStrongbranchs(self.raw) };
        assert!(n_strong_branches >= 0);
        n_strong_branches as usize
    }

    /// Returns the age of a column, i.e., the total number of successive times a column was in the LP and was 0.0 in the solution.
    pub fn age(&self) -> usize {
        let age = unsafe { ffi::SCIPcolGetAge(self.raw) };
        assert!(age >= 0);
        age as usize
    }
}

impl PartialEq for Col {
    fn eq(&self, other: &Self) -> bool {
        self.index() == other.index() && self.raw == other.raw
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::eventhdlr;
    use crate::{
        BasisStatus, Event, EventMask, Eventhdlr, Model, ModelWithProblem, ProblemOrSolving,
        SCIPEventhdlr, Solving, VarType, minimal_model,
    };

    struct ColTesterEventHandler;

    impl Eventhdlr for ColTesterEventHandler {
        fn get_type(&self) -> EventMask {
            EventMask::FIRST_LP_SOLVED
        }

        fn execute(&mut self, model: Model<Solving>, _eventhdlr: SCIPEventhdlr, event: Event) {
            assert_eq!(event.event_type(), EventMask::FIRST_LP_SOLVED);
            let vars = model.vars();
            let first_var = vars[0].clone();
            let col = first_var.col().unwrap();
            assert_eq!(col.index(), 0);
            assert_eq!(col.index(), 0);
            assert_eq!(col.index(), 0);
            assert_eq!(col.obj(), 1.0);
            assert_eq!(col.lb(), 0.0);
            assert_eq!(col.ub(), 1.0);
            assert_eq!(col.best_bound(), 0.0);
            assert_eq!(col.primal_sol(), 1.0);
            assert_eq!(col.min_primal_sol(), 1.0);
            assert_eq!(col.max_primal_sol(), 1.0);
            assert_eq!(col.basis_status(), BasisStatus::Basic);
            assert_eq!(col.var_probindex(), Some(0));
            assert!(col.is_integral());
            assert!(!col.is_removable());
            assert_eq!(col.lp_pos(), Some(0));
            assert_eq!(col.lp_depth(), Some(0));
            assert!(col.is_in_lp());
            assert_eq!(col.n_non_zeros(), 1);
            assert_eq!(col.n_lp_non_zeros(), 1);
            assert_eq!(col.vals(), vec![1.0]);
            assert_eq!(col.strong_branching_node(), None);
            assert_eq!(col.n_strong_branches(), 0);
            assert_eq!(col.age(), 0);
        }
    }

    #[test]
    fn test_col() {
        let mut model = minimal_model();
        let x = model.add_var(0.0, 1.0, 1.0, "x", VarType::Binary);

        let cons = model.add_cons(vec![&x], &[1.0], 1.0, 1.0, "cons1");
        model.set_cons_modifiable(&cons, true);

        let e = ColTesterEventHandler;
        model.add(eventhdlr(e).name("ColTesterEventHandler"));
        model.solve();
    }
}
