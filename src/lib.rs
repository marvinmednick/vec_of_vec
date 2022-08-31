#![feature(test)]
use std::fmt::format;
use std::slice::Chunks;


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
        for index in 0..total_size  {
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


#[cfg(test)]
mod array_test {

    use crate::FlattendArray;

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

}

