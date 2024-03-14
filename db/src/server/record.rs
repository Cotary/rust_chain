

pub struct Record {
    id: i32,
    name: String,
    age: i32,
}

impl Record{
    fn New(id:i32,name:String,age:i32)->Record{
        return Record{
            id,
            name,
            age,
        }
    }
}