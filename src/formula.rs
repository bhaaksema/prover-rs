use tptp::common::NonassocConnective;
use tptp::fof;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Formula {
    Bot,
    Top,
    Var(String),
    And(Vec<Formula>),
    Or(Vec<Formula>),
    Imp(Box<Formula>, Box<Formula>),
}

impl From<fof::LogicFormula<'_>> for Formula {
    fn from(value: fof::LogicFormula<'_>) -> Self {
        match value {
            fof::LogicFormula::Binary(formula) => match formula {
                fof::BinaryFormula::Assoc(formula) => formula.into(),
                fof::BinaryFormula::Nonassoc(formula) => formula.into(),
            },
            fof::LogicFormula::Unary(formula) => formula.into(),
            fof::LogicFormula::Unitary(formula) => formula.into(),
        }
    }
}

impl From<fof::UnitFormula<'_>> for Formula {
    fn from(value: fof::UnitFormula<'_>) -> Self {
        match value {
            fof::UnitFormula::Unary(formula) => formula.into(),
            fof::UnitFormula::Unitary(formula) => formula.into(),
        }
    }
}

impl From<fof::BinaryAssoc<'_>> for Formula {
    fn from(value: fof::BinaryAssoc<'_>) -> Self {
        match value {
            fof::BinaryAssoc::Or(fof::OrFormula(formulas)) => {
                Formula::Or(formulas.into_iter().map(Formula::from).collect())
            }
            fof::BinaryAssoc::And(fof::AndFormula(formulas)) => {
                Formula::And(formulas.into_iter().map(Formula::from).collect())
            }
        }
    }
}

impl From<fof::BinaryNonassoc<'_>> for Formula {
    fn from(value: fof::BinaryNonassoc<'_>) -> Self {
        let fof::BinaryNonassoc { left, op, right } = value;

        match op {
            NonassocConnective::LRImplies => {
                Formula::Imp(Box::new((*left).into()), Box::new((*right).into()))
            }
            NonassocConnective::RLImplies => {
                Formula::Imp(Box::new((*right).into()), Box::new((*left).into()))
            }
            NonassocConnective::Equivalent => {
                let left: Formula = (*left).into();
                let right: Formula = (*right).into();

                Formula::And(vec![
                    Formula::Imp(Box::new(left.clone()), Box::new(right.clone())),
                    Formula::Imp(Box::new(right), Box::new(left)),
                ])
            }
            NonassocConnective::NotEquivalent => {
                let value = fof::BinaryNonassoc {
                    left,
                    op: NonassocConnective::Equivalent,
                    right,
                };
                Formula::Imp(Box::new(value.into()), Box::new(Formula::Bot))
            }
            NonassocConnective::NotOr => Formula::Imp(
                Box::new(Formula::Or(vec![(*left).into(), (*right).into()])),
                Box::new(Formula::Bot),
            ),
            NonassocConnective::NotAnd => Formula::Imp(
                Box::new(Formula::And(vec![(*left).into(), (*right).into()])),
                Box::new(Formula::Bot),
            ),
        }
    }
}

impl From<fof::UnaryFormula<'_>> for Formula {
    fn from(value: fof::UnaryFormula<'_>) -> Self {
        match value {
            fof::UnaryFormula::Unary(_, formula) => {
                Formula::Imp(Box::new((*formula).into()), Box::new(Formula::Bot))
            }
            fof::UnaryFormula::InfixUnary(_) => panic!("unsupported Inequality"),
        }
    }
}

impl From<fof::UnitaryFormula<'_>> for Formula {
    fn from(value: fof::UnitaryFormula<'_>) -> Self {
        match value {
            fof::UnitaryFormula::Atomic(formula) => (*formula).into(),
            fof::UnitaryFormula::Parenthesised(formula) => (*formula).into(),
            fof::UnitaryFormula::Quantified(_) => panic!("unsupported Quantified"),
        }
    }
}

impl From<fof::AtomicFormula<'_>> for Formula {
    fn from(value: fof::AtomicFormula<'_>) -> Self {
        match value {
            fof::AtomicFormula::Plain(formula) => Formula::Var(formula.to_string()),
            fof::AtomicFormula::Defined(formula) => match formula {
                fof::DefinedAtomicFormula::Plain(formula) => match formula.to_string().as_str() {
                    "$true" => Formula::Top,
                    "$false" => Formula::Bot,
                    _ => Formula::Var(formula.to_string()),
                },
                fof::DefinedAtomicFormula::Infix(_) => panic!("unsupported Infix"),
            },
            fof::AtomicFormula::System(formula) => Formula::Var(formula.to_string()),
        }
    }
}
