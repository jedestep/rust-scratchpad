extern mod bson;

use bson::encode::*;
use bson::decode::*;
use bson::formattable::*;

fn main() {
    println(fmt!("5 as a bson repr: %s", 5f.to_bson_t().to_str()));
    println(fmt!("5 as a bson string: %s", 5f.to_bson_t().to_bson().to_str()));
    println(fmt!("[1,2,3] as a bson repr: %s", (~[1u16,2,3]).to_bson_t().to_str()));
    println(fmt!("[1,2,3] as a bson string: %s", (~[1u16,2,3]).to_bson_t().to_bson().to_str()));
    println(fmt!("{foo: 'bar'} as a bson repr: %s", (~"{ \"foo\": \"bar\" }").to_bson_t().to_str()));
    println(fmt!("{foo: 'bar'} as a bson string: %s", (~"{ \"foo\": \"bar\" }").to_bson_t().to_bson().to_str()));
    println(fmt!("New FooStruct as a bson repr: %s", (FooStruct::new()).to_bson_t().to_str()));
    println(fmt!("New FooStruct as a bson string: %s", (FooStruct::new()).to_bson_t().to_bson().to_str()));
    println("");
    println(fmt!("Roundtripping representations: %? -> %s -> %?",
        FooStruct::new(), (FooStruct::new()).to_bson_t().to_str(),
        BsonFormattable::from_bson_t::<FooStruct>((FooStruct::new()).to_bson_t()).unwrap()));
    println(fmt!("Roundtripping strings: %? -> %s -> %s",
        FooStruct::new(), (FooStruct::new()).to_bson_t().to_bson().to_str(),
        (decode((FooStruct::new()).to_bson_t().to_bson()).unwrap()).to_str()));
}

#[deriving(ToStr)]
impl FooStruct {
    fn new() -> FooStruct {
        FooStruct { flag: true, widget: false, value: 0 }
    }
}

impl BsonFormattable for FooStruct {
    fn to_bson_t(&self) -> Document {
        let mut doc = BsonDocument::new();
        doc.put(~"flag", Bool(self.flag));
        doc.put(~"widget", Bool(self.widget));
        doc.put(~"value", Int32(self.value as i32));
        Embedded(~doc)
    }

    fn from_bson_t(doc: Document) -> Result<FooStruct, ~str> {
        match doc {
            Embedded(d) => {
                let mut s = FooStruct::new();
                if d.contains_key(~"flag") {
                    s.flag = match d.find(~"flag").unwrap() {
                        &Bool(b) => b,
                        _ => fail!("flag must be boolean")
                    }
                }
                if d.contains_key(~"widget") {
                    s.widget = match d.find(~"widget").unwrap() {
                        &Bool(b) => b,
                        _ => fail!("widget must be boolean")
                    }
                }
                if d.contains_key(~"value") {
                    s.value = match d.find(~"value").unwrap() {
                        &Int32(i) => i as uint,
                        &Int64(i) => i as uint,
                        &Double(f) => f as uint,
                        _ => fail!("value must be numeric")
                    }
                }
                return Ok(s);
            },
            _ => fail!("can only format Embedded as FooStruct")
        }
    }
}

struct FooStruct {
    flag: bool,
    widget: bool,
    value: uint
}
