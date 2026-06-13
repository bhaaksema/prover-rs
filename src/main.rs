use crate::formula::Formula;
use rustsat::{
    instances::Cnf,
    solvers::{Solve, SolveStats},
};
use std::{env, fs};
use tptp::{TPTPIterator, top};

mod formula;
mod tseitin;

fn main() {
    let path = env::args().nth(1).expect("provide path to .p file");
    let bytes = fs::read(&path).unwrap();
    let mut instance = tseitin::TseitinInstance::new();

    for result in &mut TPTPIterator::<()>::new(&bytes) {
        match result.unwrap() {
            top::TPTPInput::Annotated(formula) => {
                if let top::AnnotatedFormula::Fof(fof) = *formula {
                    instance.add_formula(Formula::from(fof.0.formula.0));
                }
            }
            _ => panic!("unsupported TPTPInput"),
        }
    }

    let cnf: Cnf = instance.into();

    let mut solver = rustsat_glucose::core::Glucose::default();
    solver.add_cnf(cnf.clone()).unwrap();
    let result = solver.solve().unwrap();
    println!("Glucose: {:?}, {:?}", result, solver.stats().cpu_solve_time);

    let mut solver = rustsat_minisat::core::Minisat::default();
    solver.add_cnf(cnf.clone()).unwrap();
    let result = solver.solve().unwrap();
    println!("Minisat: {:?}, {:?}", result, solver.stats().cpu_solve_time);

    let mut solver = rustsat_batsat::BasicSolver::default();
    solver.add_cnf(cnf.clone()).unwrap();
    let result = solver.solve().unwrap();
    println!("Batsat: {:?}, {:?}", result, solver.stats().cpu_solve_time);

    let mut solver = rustsat_cadical::CaDiCaL::default();
    solver.add_cnf(cnf.clone()).unwrap();
    let result = solver.solve().unwrap();
    println!("CaDiCal: {:?}, {:?}", result, solver.stats().cpu_solve_time);

    let mut solver = rustsat_kissat::Kissat::default();
    solver.add_cnf(cnf.clone()).unwrap();
    let result = solver.solve().unwrap();
    println!("Kissat: {:?}, {:?}", result, solver.stats().cpu_solve_time);
}
