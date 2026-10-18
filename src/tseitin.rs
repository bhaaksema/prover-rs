use internment::Intern;
use rustsat::{
    instances::{Cnf, ObjectVarManager, SatInstance},
    types::Lit,
};
use std::collections::HashMap;

use crate::formula::Formula;

pub(crate) struct TseitinInstance {
    instance: SatInstance<ObjectVarManager>,
    top: Option<Lit>,
    cache: HashMap<Intern<Formula>, Lit>,
}

impl TseitinInstance {
    pub fn new() -> Self {
        Self {
            instance: SatInstance::new(),
            top: None,
            cache: HashMap::new(),
        }
    }

    pub fn add_axiom(&mut self, formula: Formula) {
        let lit = self.define_formula(Intern::new(formula));
        self.instance.add_unit(lit);
    }

    pub fn add_conjecture(&mut self, formula: Formula) {
        let lit = self.define_formula(Intern::new(formula));
        self.instance.add_unit(!lit);
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

    fn define_formula(&mut self, formula: Intern<Formula>) -> Lit {
        if let Some(&lit) = self.cache.get(&formula) {
            return lit;
        }
        let lit = match &*formula {
            Formula::Bot => !self.get_top(),
            Formula::Top => self.get_top(),
            Formula::Var(name) => self
                .instance
                .var_manager_mut()
                .object_var(name.clone())
                .pos_lit(),
            Formula::And(left, right) => {
                let root_lit = self.instance.new_lit();
                let literals = [self.define_formula(*left), self.define_formula(*right)];

                self.instance.add_lit_impl_cube(root_lit, &literals);
                self.instance.add_cube_impl_lit(&literals, root_lit);
                root_lit
            }
            Formula::Or(left, right) => {
                let root_lit = self.instance.new_lit();
                let literals = [self.define_formula(*left), self.define_formula(*right)];

                self.instance.add_lit_impl_clause(root_lit, &literals);
                self.instance.add_clause_impl_lit(&literals, root_lit);
                root_lit
            }
            Formula::Imp(left, right) => {
                let root_lit = self.instance.new_lit();
                let (left_lit, right_lit) =
                    (self.define_formula(*left), self.define_formula(*right));

                self.instance // t -> (a -> b) == t -> (!a | b)
                    .add_lit_impl_clause(root_lit, &[!left_lit, right_lit]);
                self.instance // (a -> b) -> t == (!a | b) -> t
                    .add_clause_impl_lit(&[!left_lit, right_lit], root_lit);
                root_lit
            }
        };
        self.cache.insert(formula, lit);
        lit
    }
}

impl From<TseitinInstance> for Cnf {
    fn from(value: TseitinInstance) -> Self {
        value.instance.into_cnf().0
    }
}
