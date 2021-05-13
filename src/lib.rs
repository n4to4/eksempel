pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn strlen2<S>(s: S) -> usize
where
    S: AsRef<str>,
{
    s.as_ref().len()
}

pub fn strlen_dyn(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn bool_then<T>(b: bool, f: impl FnOnce() -> T) -> Option<T> {
    if b {
        Some(f())
    } else {
        None
    }
}

pub trait Hei {
    fn hei(&self);
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

pub fn foo() {
    //bar(&["J", "Jon"]);
    //bar(&[String::from("J"), String::from("Jon")]);
    bar(&[Box::new("J"), Box::new(String::from("Jon"))]);
}

pub fn bar(s: &[Box<dyn Hei>]) {
    for h in s {
        h.hei();
    }
}

pub fn sized_or_not() {
    fn is_sized<T>() {}

    //is_sized::<dyn Hei>();

    //is_sized::<str>();
    is_sized::<&str>();

    //is_sized::<[u8]>();
    is_sized::<&[u8]>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runit() {
        foo();
    }
}
