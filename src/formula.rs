use internment::Intern;
use mrs_tptp::{BinaryConnective, FOFAtomicFormula, FOFFormula};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Formula {
    Bot,
    Top,
    Var(String),
    And(Intern<Formula>, Intern<Formula>),
    Or(Intern<Formula>, Intern<Formula>),
    Imp(Intern<Formula>, Intern<Formula>),
}

impl Formula {
    fn not(formula: Formula) -> Self {
        Formula::imp(formula, Formula::Bot)
    }

    fn and(left: Formula, right: Formula) -> Self {
        Formula::And(Intern::new(left), Intern::new(right))
    }

    fn or(left: Formula, right: Formula) -> Self {
        Formula::Or(Intern::new(left), Intern::new(right))
    }

    fn imp(left: Formula, right: Formula) -> Self {
        Formula::Imp(Intern::new(left), Intern::new(right))
    }

    fn iff(left: Formula, right: Formula) -> Self {
        let (left, right) = (Intern::new(left), Intern::new(right));
        Formula::and(Formula::Imp(left, right), Formula::Imp(right, left))
    }
}

impl From<FOFFormula<'_>> for Formula {
    fn from(value: FOFFormula<'_>) -> Self {
        match value {
            FOFFormula::Atomic(formula) => formula.into(),
            FOFFormula::Negation(formula) => Formula::not((*formula).into()),
            FOFFormula::Binary {
                left,
                connective,
                right,
            } => {
                let left: Formula = (*left).into();
                let right: Formula = (*right).into();

                match connective {
                    BinaryConnective::And => Formula::and(left, right),
                    BinaryConnective::Or => Formula::or(left, right),
                    BinaryConnective::Impl => Formula::imp(left, right),
                    BinaryConnective::RevImpl => Formula::imp(right, left),
                    BinaryConnective::Iff => Formula::iff(left, right),
                    BinaryConnective::Xor => Formula::not(Formula::iff(left, right)),
                    BinaryConnective::Nor => Formula::not(Formula::or(left, right)),
                    BinaryConnective::Nand => Formula::not(Formula::and(left, right)),
                }
            }
            FOFFormula::Parens(formula) => (*formula).into(),
            FOFFormula::Quantified { .. } => panic!("unsupported Quantified"),
            FOFFormula::Equality(..) | FOFFormula::Inequality(..) => panic!("unsupported Infix"),
        }
    }
}

impl From<FOFAtomicFormula<'_>> for Formula {
    fn from(value: FOFAtomicFormula<'_>) -> Self {
        match value {
            FOFAtomicFormula::True => Formula::Top,
            FOFAtomicFormula::False => Formula::Bot,
            formula => Formula::Var(formula.to_string()),
        }
    }
}
