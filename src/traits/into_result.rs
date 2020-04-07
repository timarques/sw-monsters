use crate::error::Error;

pub trait IntoResult {

    type Item;

    fn ok(self) -> Result<Self::Item, Error>;

}

impl <A> IntoResult for Option<A> {

    type Item = A;

    fn ok(self) -> Result<Self::Item, Error> {
        self.ok_or(Error::new(""))
    }

}
