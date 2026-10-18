use mrs_tptp::{AnnotatedFormula, FOFStatement, FormulaRole, TPTPInput, TPTPIterator};
use rustsat::solvers::{Solve, SolveStats};
use std::{env, fs, thread};

use prover_rs::{Solver, formula::Formula, tseitin};

const STACK_SIZE: usize = 128 << 20;

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

    let mut solver = Solver::default();
    solver.add_cnf(instance.into()).unwrap();
    println!(
        "{}: {:?}, {:?}",
        solver.signature(),
        solver.solve().unwrap(),
        solver.stats().cpu_solve_time
    );
}

fn main() {
    thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}
