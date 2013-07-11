extern mod bson;
extern mod mongo;

use bson::encode::*;
//use bson::decode::*;
use bson::formattable::*;

use mongo::client::*;
use mongo::util::*;
//use mongo::db::*;
use mongo::coll::*;
//use mongo::cursor::*;

fn main() {
    let client = @Client::new();

    //Connect to the databse
    match client.connect(~"127.0.0.1", 27017 as uint) {
        Ok(_) => println("now connected to MongoDB on localhost:27017"),
        Err(e) => fail!("%s", MongoErr::to_str(e))
    }

    //Create a collection "bar" in a database "foo".
    //If foo does not exist, it will be created the first time
    //something is inserted.
    let collection = @Collection::new(~"foo", ~"bar", client);

    //Insert a document by a string.
    //The None argument specifies to use the default write concern.
    match collection.insert(~"{ \"name\": \"constants\", \"values\": { \"pi\": 3.1415926535, \"e\": 2.718281828 } }", None) {
        Ok(_) => println("Constants have been stored"),
        Err(e) => fail!("%s", MongoErr::to_str(e))
    }

    //Insert a document from a native struct.
    //This struct implemented bson::formattable::BsonFormattable.
    match collection.insert(~FooStruct::new(), None) {
        Ok(_) => println(fmt!("FooStruct has been stored")),
        Err(e) => fail!("%s", MongoErr::to_str(e))
    }

    //Retrieve the stuff we just put in.
    match collection.find(None, None, None) {
        Ok(c) => {
            //We need to move the cursor into a mutable slot or it's useless!
            let mut cursor = c;
            //The cursor is an instance of std::iterator::Iterator, so it has a lot of functionality
            for cursor.advance |doc| {
                println(fmt!("Here's a document: %s", doc.to_str()));
            }
        },
        Err(e) => fail!("%s", MongoErr::to_str(e))
    }

    //Update our FooStruct
    match collection.update(SpecNotation(~"{ \"flag\": true }"),
                            SpecNotation(~"{ \"$set\": { \"flag\": false } }"),
                            Some(~[MULTI,UPSERT]), None, None) {
        Ok(_) => println("Updated 'flag' to be false"),
        Err(e) => fail!("%s", MongoErr::to_str(e))
    }

    //Retrieve it again and see how it turned out
    match collection.find(None, None, None) {
        Ok(c) => {
            //We need to move the cursor into a mutable slot or it's useless!
            let mut cursor = c;
            //The cursor is an instance of std::iterator::Iterator, so it has a lot of functionality
            for cursor.advance |doc| {
                println(fmt!("Here's a document: %s", doc.to_str()));
            }
        },
        Err(e) => fail!("%s", MongoErr::to_str(e))
    }

    //Clear out the collection.
    match collection.remove(None, None, None, None) {
        Ok(_) => println("Cleared out the bar collection"),
        Err(e) => fail!("%s", MongoErr::to_str(e))
    }

    //Do a large batch insert
    let mut batch: ~[~str] = ~[];

    for std::int::range(0,1000) |i| {
        batch.push(fmt!("{ \"num\": %d, \"key%s\": \"foo\"}", i, (i % 100).to_str()));
    }

    //Continue if something goes wrong
    match collection.insert_batch(batch, Some(~[CONT_ON_ERR]), None, None) {
        Ok(_) => println("Inserted 1000 num objects."),
        Err(e) => fail!("%s", MongoErr::to_str(e))
    }

    //Now we can use Rust's iterator utility to do a variety of things
    match collection.find(None, None, None) {
        Ok(c) => {
            let mut cursor1 = copy c;
            let cursor2 = copy c; //this one can be immutable since we will chain to it

            println(fmt!("the 456th element of the collection: %s", cursor1.nth(456).unwrap().to_str()));

            println("only display the first 5 elements with a key called key5");
            for cursor2.filter(|&elt| elt.contains_key(~"key5")).take_(5).advance |doc| {
                println(fmt!("Here's a document: %s", doc.to_str()));
            }
        }
        Err(e) => fail!("%s", MongoErr::to_str(e))
    }

}

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
