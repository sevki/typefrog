use crate::{fact::Atom, horn::Dialect, internment::Interned};

type Arg = Interned<Atom>;

pub(crate) type Unary = (Arg,);
pub(crate) type Binary = (Arg, Arg);
pub(crate) type Ternary = (Arg, Arg, Arg);

pub(crate) fn format_unary<D: Dialect>(arg: &Unary) -> String {
    format!(
        "{o}{a}{c}",
        o = D::OPEN_PAREN,
        a = arg.0,
        c = D::CLOSE_PAREN
    )
}

pub(crate) fn format_binary<D: Dialect>(arg: &Binary) -> String {
    format!(
        "{o}{a0}, {a1}{c}",
        a0 = arg.0,
        a1 = arg.1,
        o = D::OPEN_PAREN,
        c = D::CLOSE_PAREN
    )
}

pub(crate) fn format_ternary<D: Dialect>(arg: &Ternary) -> String {
    format!(
        "{o}{a0}, {a1}, {a2}{c}",
        o = D::OPEN_PAREN,
        a0 = arg.0,
        a1 = arg.1,
        a2 = arg.2,
        c = D::CLOSE_PAREN
    )
}

pub trait InternComment {
    fn intern_comment(&self) -> String;
}

impl InternComment for Unary {
    fn intern_comment(&self) -> String {
        self.0.uid().to_string()
    }
}

impl InternComment for Binary {
    fn intern_comment(&self) -> String {
        format!("{} {}", self.0.uid(), self.1.uid())
    }
}

impl InternComment for Ternary {
    fn intern_comment(&self) -> String {
        format!("{} {} {}", self.0.uid(), self.1.uid(), self.2.uid())
    }
}
