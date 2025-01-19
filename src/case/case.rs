use crate::error::error::CustomError::ParseIntError;

trait Namable {
    fn name (&self) -> &'static str;
}
struct Foo;
impl Namable for Foo {
    fn name (&self) -> &'static str { "Foo" }
}
struct Bar;
impl Namable for Bar {
    fn name (&self) -> &'static str { "Bar" }
}
enum AllNamables {
    Foo (Foo),
    Bar (Bar),
}
impl Namable for AllNamables {
    fn name (&self) -> &'static str {
        match self {
            AllNamables::Foo (f) => f.name(),
            AllNamables::Bar (b) => b.name(),
        }
    }
}


impl From<Foo> for AllNamables {
    fn from (f: Foo) -> Self { AllNamables::Foo (f) }
}
impl From<Bar> for AllNamables {
    fn from (b: Bar) -> Self { AllNamables::Bar (b) }
}

impl TryFrom<AllNamables> for Foo {
    type Error = ();

    fn try_from(value: AllNamables) -> Result<Self, Self::Error> {
        match value {
            AllNamables::Foo (f) => Ok(f),
            _ => Err(())
        }
    }
}

impl TryFrom<AllNamables> for Bar {
    type Error = ();
    fn try_from(value: AllNamables) -> Result<Self, Self::Error> {
        match value {
            AllNamables::Bar (b) => Ok(b),
            _ => Err(())
        }
    }
}


struct UserCertification {
    current_work: AllNamables,
}
impl UserCertification {
    fn new<T: Into<AllNamables>> (current_work: T) -> Self {
        UserCertification { current_work: current_work.into(), }
    }
}


pub fn fun() {
    let foo = Foo;
    let bar = Bar;
    let a = 1;
    // let x: UserCertification;
    //if (a==1){
    let x = UserCertification::new(foo);
    //}else {
    //x = UserCertification::new(bar);
    //}
    //let x :Foo =  x.current_work.into();
    let y: Foo = x.current_work.try_into().unwrap();

}