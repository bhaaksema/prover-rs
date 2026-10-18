pub mod formula;
pub mod tseitin;

cfg_select! {
    feature = "batsat" => { pub type Solver = rustsat_batsat::BasicSolver; }
    feature = "cadical" => { pub type Solver = rustsat_cadical::CaDiCaL<'static, 'static>; }
    feature = "glucose" => { pub type Solver = rustsat_glucose::core::Glucose; }
    feature = "kissat" => { pub type Solver = rustsat_kissat::Kissat<'static>; }
    _ => { pub type Solver = rustsat_minisat::core::Minisat; }
}
