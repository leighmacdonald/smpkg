pub fn build(name: String) -> Result<(), Error> {
    match checkout(name) {
        Err(e) => return Err(e),
        Ok(_) => todo!(),
    }
}
