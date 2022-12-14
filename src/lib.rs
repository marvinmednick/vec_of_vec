#![feature(test)]
use std::slice::Chunks;
use log::{error};
use std::fmt;

#[derive(Debug,Clone,PartialOrd,Ord,PartialEq,Eq)]
pub enum MinMax<T> {
    Min,
    Value(T),
    Max,
    NA,
}

// Implement `Display` for `MinMax`.
impl<T: fmt::Display> fmt::Display for MinMax<T>

{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MinMax::Min => write!(f,  "Min"),
            MinMax::Max => write!(f,   "Max"),
            MinMax::NA  => write!(f,   "NA"),
            MinMax::Value(ref x) =>  write!(f, "{}",x)
        }
    }
}

#[derive(Debug)]
pub struct VecOfVec<T: std::cmp::Ord> {
    array : Vec::<Vec<MinMax<T>>>,
    width: usize,
    height: usize

}

impl<T:std::fmt::Debug+Clone+std::fmt::Display+std::cmp::Ord> VecOfVec<T> {
    pub fn new(x: usize, y: usize, default: MinMax<T>) -> VecOfVec<T> {
        let mut array = Vec::<Vec<MinMax<T>>>::new();
        for _row in 0..y  {
            let mut row_data = Vec::<MinMax<T>>::new();
            for _col in 0..x {
                row_data.push(default.clone());
            }
            array.push(row_data);
        }

        VecOfVec { array: array, width: x, height: y }
    }

    pub fn get(&self,x: usize, y: usize) -> MinMax<T> {
        if x < self.width && y < self.height {
            self.array[y][x].clone()
        }
        else {
            MinMax::NA
        }
    }

    pub fn get_string(&self,x: usize, y: usize) ->  String{
        if x < self.width && y < self.height {
            match &self.array[y][x] {
                MinMax::Value(val) => format!("{}",val),
                MinMax::Min =>   format!("{}","m"),
                MinMax::Max =>   format!("{}","M"),
                MinMax::NA =>   format!("{}","NA"),
            }
        }
        else {
                format!("{}","N")
        }
    }


    pub fn get_row_iter(&self) -> std::slice::Iter<Vec<MinMax<T>>> {
        self.array.iter()
    }

    pub fn set(&mut self, x : usize, y : usize , value : MinMax<T>)  {
        if x < self.width && y < self.height {
            self.array[y][x] = value;
        }
        else {
            error!("Invalid set indexes x {} (max {}) y {} (max {}) ",x,y,self.width,self.height);
        }
    }

    pub fn display(&self, col_name: String, row_name: String) {
        let mut count = 0;
        let header : String = (0..self.width).map(|val| format!("{:2} ",val)).collect();
        println!("{:11} {}",col_name,header);
        for row in self.get_row_iter() {
            let row_format : String = row.iter().map(|val| { 
                match val {
                    MinMax::Value(x) => format!("{:>2} ",x),
                    MinMax::Min    => format!("{:>2} ","m"),
                    MinMax::Max    => format!("{:>2} ","M"),
                    MinMax::NA    => format!("{:>2} ","NA"),
                }}).collect();
            println!("{} {:2} :    {}",row_name, count,row_format);
            count += 1
        }

    }


}


#[derive(Debug)]
pub struct FlattendArray<T> {
    array : Vec::<Option<T>>,
    width: usize,
    height: usize

}

impl<T:std::fmt::Debug+Clone+std::fmt::Display> FlattendArray<T> {

    pub fn new(width: usize, height: usize) -> FlattendArray<T> {
        let mut array = Vec::<Option<T>>::new();
        let total_size = width * height;
        for _index in 0..total_size  {
            array.push(None);
        }

        FlattendArray { array: array, width: width, height: height }
    }

    pub fn get(&self,x: usize, y: usize) -> Option<T> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.array[index].clone()
        }
        else {
            None
        }
    }

    pub fn get_string(&self,x: usize, y: usize) ->  String{
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            match &self.array[index] {
                Some(val) => format!("{}",val),
                None =>   format!("{}","N"),
            }
        }
        else {
                format!("{}","N")
        }
    }


    pub fn get_row_iter(&self) -> Chunks<'_, Option<T>> {
        self.array.chunks(self.width)
    }


    pub fn set(&mut self, x : usize, y : usize , value : T)  {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.array[index] = Some(value);
        }
    }

    pub fn unset(&mut self, x : usize, y : usize )  {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.array[index] = None;
        }
    }

}

/*
fn main() { 
    let vertexes = 7;
    let iterations = vertexes-1;

    let mut data : VecOfVec<u32> = VecOfVec::new(vertexes, iterations);


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
*/


#[cfg(test)]
mod array_test {

    use crate::VecOfVec;
    use crate::FlattendArray;
    use crate::MinMax::{Min,Value,Max,NA};

    #[test]
    fn vec_of_vec_basic() {
        let size = 8; 
        let mut data : VecOfVec<u32> = VecOfVec::new(size, size,NA);


        for i in 0..size {
            assert_eq!(data.get(i,i), NA)
        }
        data.set(1,2,Value(12));
        data.set(2,3,Value(23));
        data.set(3,4,Value(34));
        data.set(6,7,Value(67));
        data.set(8,8,Value(88));
        assert_eq!(data.get(1,2),Value(12));
        assert_eq!(data.get(2,3),Value(23));
        assert_eq!(data.get(3,4),Value(34));
        assert_eq!(data.get(6,7),Value(67));
        assert_eq!(data.get(8,8),NA);
    }

    #[test]
    fn flattend_array_basic() {
        let size = 8; 
        let mut data : FlattendArray<u32> = FlattendArray::new(size, size);


        data.set(1,2,12);
        data.set(2,3,23);
        data.set(3,4,34);
        data.set(6,7,67);
        data.set(8,8,88);
        for i in 0..size {
            assert_eq!(data.get(i,i), None)
        }
        assert_eq!(data.get(1,2),Some(12));
        assert_eq!(data.get(2,3),Some(23));
        assert_eq!(data.get(3,4),Some(34));
        assert_eq!(data.get(6,7),Some(67));
        assert_eq!(data.get(8,8),None);
    }

    #[test]
    fn min_max_compare_test() {
        assert!(Value(0)>Min);
        assert!(Value(10)>Value(9));
        assert!(Value(100000)<Max);
        assert!(Value(100000)<NA);
        assert!(Min::<u64> < NA::<u64>);
    }

    #[test]
    fn min_max_format_test() {
        assert_eq!(format!("{}",Value(0)),"0");
        assert_eq!(format!("{}",Value(100)),"100");
        assert_eq!(format!("{}",Min::<u32>),"Min");
        assert_eq!(format!("{}",Max::<u32>),"Max");
        assert_eq!(format!("{}",NA::<u32>),"NA");
    }

}


extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

}

