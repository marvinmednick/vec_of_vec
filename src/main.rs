
use std::fmt::format;

#[derive(Debug)]
struct BellmanData<T> {
    array : Vec::<Vec<Option<T>>>,
    num_vertex: usize,
    num_iterations: usize

}

impl<T:std::fmt::Debug+Clone+std::fmt::Display> BellmanData<T> {

    pub fn new(num_vertex: usize, num_iterations: usize) -> BellmanData<T> {
        let mut array = Vec::<Vec<Option<T>>>::new();
        for y in 0..num_iterations  {
            let mut row = Vec::<Option<T>>::new();
            for x in 0..num_vertex {
                row.push(None);
            }
            array.push(row);
        }

        BellmanData { array: array, num_vertex: num_vertex, num_iterations: num_iterations }
    }

    pub fn get(&self,vertex: usize, iteration: usize) -> Option<T> {
        self.array[iteration][vertex].clone()
    }

    pub fn get_string(&self,vertex: usize, iteration: usize) ->  String{
        match &self.array[iteration][vertex] {
            Some(x) => format!("{}",x),
            None =>   format!("{}","N"),
        }
    }


    pub fn get_row_iter(&self) -> std::slice::Iter<Vec<Option<T>>> {
        self.array.iter()
    }

    pub fn set(&mut self, vertex : usize, iteration : usize , value : Option<T>)  {
        self.array[iteration][vertex] = value;

    }

}


fn main() { 
    let vertexes = 7;
    let iterations = vertexes-1;

    let data : BellmanData<u32> = BellmanData::new(vertexes, iterations);
   
    /*
    let mut xy_array = Vec::<Vec<u32>>::new();
    for y in 0..iterations  {
        let mut row = Vec::<u32>::new();
        for x in 0..vertexes {
            let value = y*vertexes + x;
            row.push(value.try_into().unwrap());

        }
        xy_array.push(row);

    }


    let mut count = 0;
    println!("{:17} {:2}","Vertex",1);
    for row in xy_array {
        let row_format : String = row.iter().map(|val| format!("{:2} ",val)).collect();
        println!("Iteration {:2} :    {}", count,row_format);
        count += 1
    }

    */
    let mut count = 0;
    let header : String = (0..vertexes).map(|val| format!("{:2} ",val)).collect();
    println!("{:17} {}","Vertex",header);
    for row in data.get_row_iter() {
        let row_format : String = row.iter().map(|val| { 
            match val {
                Some(x) => format!("{:2} ",val.unwrap()),
                None    => format!("{:2} ","N"),
            }}).collect();
        println!("Iteration {:2} :    {}", count,row_format);
        count += 1
    }

    println!("Iter 1,Vertex 3 -> {}", data.get_string(3,1));

    println!("Iter 4,Vertex 1 -> {}", data.get_string(1,4));
    println!("Iter 6,Vertex 5 -> {}", data.get_string(6,5));



}
