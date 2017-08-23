use std::mem::transmute;

struct Wrapper<T> where T: ApiTrait<'static>{
    lib: Lib,
    api: T
}

impl<T> Wrapper<T> where T: ApiTrait<'static>{
    pub fn new() -> Wrapper<T> {
        let lib = Lib {
            something: 3
        };
        let static_lib: &'static Lib = unsafe{transmute(&lib)};
        let api = T::load(static_lib);
        Wrapper{
            lib: lib,
            api: api
        }
    }
}

trait ApiTrait<'a>{
    fn load(lib: &'a Lib) -> Self;
}


struct Lib {
    pub something: u32
}



struct Api {
    whatever: u32
}

impl<'a> ApiTrait<'a> for Api {
    fn load(lib: &'a Lib) -> Self{
        Api{
            whatever: lib.something
        }
    }
}


fn main(){
    let lib = Lib{
        something: 4
    };

    let api = Api::load(&lib);
    drop(lib);
}