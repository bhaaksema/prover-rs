use rustsat::{
    instances::{Cnf, ObjectVarManager, SatInstance},
    types::Lit,
};

use crate::formula::Formula;

pub(crate) struct TseitinInstance {
    instance: SatInstance<ObjectVarManager>,
    top: Option<Lit>,
}

impl TseitinInstance {
    pub fn new() -> Self {
        Self {
            instance: SatInstance::new(),
            top: None,
        }
    }

    pub fn add_formula(&mut self, formula: Formula) {
        let lit = self.to_lit(formula);
        self.instance.add_unit(lit);
    }

    fn get_top(&mut self) -> Lit {
        match self.top {
            Some(top) => top,
            None => {
                let top = self.instance.new_lit();
                self.instance.add_unit(top);
                self.top = Some(top);
                top
            }
        }
    }

    fn to_lit(&mut self, formula: Formula) -> Lit {
        match formula {
            Formula::Bot => !self.get_top(),
            Formula::Top => self.get_top(),
            Formula::Var(name) => self.instance.var_manager_mut().object_var(name).pos_lit(),
            Formula::And(formulas) => {
                let root_lit = self.instance.new_lit();
                let literals: Vec<Lit> = formulas
                    .into_iter()
                    .map(|formula| self.to_lit(formula))
                    .collect();

                self.instance.add_lit_impl_cube(root_lit, &literals);
                self.instance.add_cube_impl_lit(&literals, root_lit);
                root_lit
            }
            Formula::Or(formulas) => {
                let root_lit = self.instance.new_lit();
                let literals: Vec<Lit> = formulas
                    .into_iter()
                    .map(|formula| self.to_lit(formula))
                    .collect();

                self.instance.add_lit_impl_clause(root_lit, &literals);
                self.instance.add_clause_impl_lit(&literals, root_lit);
                root_lit
            }
            Formula::Imp(left, right) => {
                let root_lit = self.instance.new_lit();
                let (left_lit, right_lit) = (self.to_lit(*left), self.to_lit(*right));

                self.instance // t -> (a -> b) == t -> (!a | b)
                    .add_lit_impl_clause(root_lit, &[!left_lit, right_lit]);
                self.instance // (a -> b) -> t == (!a | b) -> t
                    .add_clause_impl_lit(&[!left_lit, right_lit], root_lit);
                root_lit
            }
        }
    }
}

impl Into<Cnf> for TseitinInstance {
    fn into(self) -> Cnf {
        self.instance.into_cnf().0
    }
}
