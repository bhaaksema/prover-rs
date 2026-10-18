use crate::formula::Formula;
use mrs_tptp::{AnnotatedFormula, FOFStatement, FormulaRole, TPTPInput, TPTPIterator};
use rustsat::{
    instances::Cnf,
    solvers::{Solve, SolveStats},
};
use std::{env, fs, thread};

const STACK_SIZE: usize = 128 << 20;

cfg_select! {
    feature = "batsat" => { type Solver = rustsat_batsat::BasicSolver; }
    feature = "cadical" => { type Solver = rustsat_cadical::CaDiCaL<'static, 'static>; }
    feature = "glucose" => { type Solver = rustsat_glucose::core::Glucose; }
    feature = "kissat" => { type Solver = rustsat_kissat::Kissat<'static>; }
    _ => { type Solver = rustsat_minisat::core::Minisat; }
}

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

    let mut solver = Solver::default();
    solver.add_cnf(cnf).unwrap();
    let result = solver.solve().unwrap();
    println!(
        "{}: {:?}, {:?}",
        solver.signature(),
        result,
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
