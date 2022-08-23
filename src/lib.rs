
use std::fmt::format;

#[derive(Debug)]
struct VecofVec<T> {
    array : Vec::<Vec<Option<T>>>,
    x: usize,
    y: usize

}

impl<T:std::fmt::Debug+Clone+std::fmt::Display> VecofVec<T> {

    pub fn new(x: usize, y: usize) -> VecofVec<T> {
        let mut array = Vec::<Vec<Option<T>>>::new();
        for row in 0..y  {
            let mut row_data = Vec::<Option<T>>::new();
            for col in 0..x {
                row_data.push(None);
            }
            array.push(row_data);
        }

        VecofVec { array: array, x: x, y: y }
    }

    pub fn get(&self,x: usize, y: usize) -> Option<T> {
        self.array[y][x].clone()
    }

    pub fn get_string(&self,x: usize, y: usize) ->  String{
        match &self.array[y][x] {
            Some(val) => format!("{}",val),
            None =>   format!("{}","N"),
        }
    }


    pub fn get_row_iter(&self) -> std::slice::Iter<Vec<Option<T>>> {
        self.array.iter()
    }

    pub fn set(&mut self, x : usize, y : usize , value : T)  {
        self.array[y][x] = Some(value);

    }

}


fn main() { 
    let vertexes = 7;
    let iterations = vertexes-1;

    let mut data : VecofVec<u32> = VecofVec::new(vertexes, iterations);


    data.set(3,0,44);
   
    let mut count = 0;
    let header : String = (0..vertexes).map(|val| format!("{:2} ",val)).collect();
    println!("{:17} {}","Vertex",header);
    for row in data.get_row_iter() {
        let row_format : String = row.iter().map(|val| { 
            match val {
                Some(x) => format!("{:>2} ",val.unwrap()),
                None    => format!("{:>2} ","N"),
            }}).collect();
        println!("Iteration {:2} :    {}", count,row_format);
        count += 1
    }

    println!("Iter 0,Vertex 3 -> {}", data.get_string(3,0));
    println!("Iter 1,Vertex 3 -> {}", data.get_string(3,1));

    println!("Iter 4,Vertex 1 -> {}", data.get_string(1,4));
    println!("Iter 6,Vertex 5 -> {}", data.get_string(6,5));



}
