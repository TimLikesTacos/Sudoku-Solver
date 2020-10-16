
pub struct something <T> {
    it: Vec<T>
}

pub trait theTrait {
    fn o ();
}

// pub trait otherTrait{
//     fn m ();
// }

impl <T>something<T> {
    fn new () -> something<T> {
        something{
            it: Vec::new(),
        }
    }
}

impl <T: From<u32>> theTrait for something <T> {
    fn o () {}
}

impl <T> theTrait for something<T> {
    fn o() {}
}

