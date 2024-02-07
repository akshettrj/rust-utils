use std::borrow::Cow;

pub trait ToCow {
    fn cow(self) -> Cow<'static, str>;
}

impl ToCow for &'static str {
    fn cow(self) -> Cow<'static, str> {
        Cow::Borrowed(self)
    }
}

impl ToCow for String {
    fn cow(self) -> Cow<'static, str> {
        Cow::Owned(self)
    }
}

impl ToCow for Cow<'static, str> {
    fn cow(self) -> Cow<'static, str> {
        self.to_owned()
    }
}
