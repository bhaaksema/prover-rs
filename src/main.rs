use crate::formula::Formula;
use mrs_tptp::{AnnotatedFormula, FOFStatement, FormulaRole, TPTPInput, TPTPIterator};
use rustsat::{
    instances::Cnf,
    solvers::{Solve, SolveStats},
};
use std::{env, fs, thread};

const STACK_SIZE: usize = 128 << 20;

mod formula;
mod tseitin;

fn run() {
    let path = env::args().nth(1).expect("provide path to .p file");
    let input = fs::read_to_string(&path).unwrap();
    let mut instance = tseitin::TseitinInstance::new();

    for result in &mut TPTPIterator::new(&input) {
        match result.unwrap() {
            TPTPInput::Formula(formula) => {
                if let AnnotatedFormula::FOF(fof) = formula {
                    let formula = match fof.formula {
                        FOFStatement::Logical(formula) => Formula::from(formula),
                        FOFStatement::Sequent(..) => panic!("unsupported Sequent"),
                    };

                    match fof.role {
                        FormulaRole::Axiom => instance.add_axiom(formula),
                        FormulaRole::Conjecture => instance.add_conjecture(formula),
                        role => panic!("unsupported role {:?}", role),
                    }
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

fn main() {
    thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
