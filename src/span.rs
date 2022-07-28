#[derive(Copy, Clone, Debug)]
pub struct Span {
    pub start: i32,
    pub end: i32,
}

#[derive(Copy, Clone, Debug)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}
