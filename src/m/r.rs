pub trait Tr {
    fn de(&self);

    fn fe(&self);
}

pub struct Br;

impl Tr for Br {
    fn de(&self) {
        println!("de method..");
    }

    fn fe(&self) {
        println!("fe method..");
    }
}

#[cfg(test)]
mod test {
    use super::Br;
    use super::Tr;

    #[test]
    fn help() {
        let b = Br;
        b.fe();
    }
}
