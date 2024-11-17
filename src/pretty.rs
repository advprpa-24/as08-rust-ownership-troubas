use crate::term::*;
use std::fmt;

/// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    match term {
        Term::Var(txt) => {
            txt.to_string()
        }
        Term::Abs(txt, term) => {
            format!("λ{}. ({})", txt, pretty_print(term))
        }
        Term::App(t1, t2) => {
            format!("{} {}", pretty_print(t1), pretty_print(t2))
        }
    }
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_variable() {
        let term = var("x");
        assert_eq!(pretty_print(&term), "x");
    }

    #[test]
    fn test_print_abstraction() {
        let term = abs("x", var("M"));
        assert_eq!(pretty_print(&term), "λx. (M)");
    }

    #[test]
    fn test_print_applicaton() {
        let term = app(var("x"), var("y"));
        assert_eq!(pretty_print(&term), "x y");
    }
}
