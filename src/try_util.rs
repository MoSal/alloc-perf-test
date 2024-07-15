pub(crate) trait BoolExt {
    fn err_if<E, F>(self, f: F) -> Result<(), E>
        where F: FnOnce() -> E;
    fn err_if_not<E, F>(self, f: F) -> Result<(), E>
        where F: FnOnce() -> E;
}

impl BoolExt for bool {
    fn err_if<E, F>(self, f: F) -> Result<(), E>
        where F: FnOnce() -> E
    {
        match self {
            true => Err(f()),
            false => Ok(()),
        }
    }

    fn err_if_not<E, F>(self, f: F) -> Result<(), E>
        where F: FnOnce() -> E
    {
        (!self).err_if(f)
    }
}
