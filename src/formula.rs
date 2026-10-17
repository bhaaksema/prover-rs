use mrs_tptp::{BinaryConnective, FOFAtomicFormula, FOFFormula};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Formula {
    Bot,
    Top,
    Var(String),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Imp(Box<Formula>, Box<Formula>),
}

impl Formula {
    fn not(formula: Formula) -> Self {
        Formula::Imp(Box::new(formula), Box::new(Formula::Bot))
    }

    fn and(left: Formula, right: Formula) -> Self {
        Formula::And(Box::new(left), Box::new(right))
    }

    fn or(left: Formula, right: Formula) -> Self {
        Formula::Or(Box::new(left), Box::new(right))
    }

    fn iff(left: Formula, right: Formula) -> Self {
        Formula::and(
            Formula::Imp(Box::new(left.clone()), Box::new(right.clone())),
            Formula::Imp(Box::new(right), Box::new(left)),
        )
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
                    BinaryConnective::Impl => Formula::Imp(Box::new(left), Box::new(right)),
                    BinaryConnective::RevImpl => Formula::Imp(Box::new(right), Box::new(left)),
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
