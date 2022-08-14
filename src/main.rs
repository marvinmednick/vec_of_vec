

struct BellmanData {
    array : Vec::<Vec<u32>>,
    num_vertex: usize,
    num_iterations: usize

}

impl BellmanData {

    pub fn new(num_vertex: usize, num_iterations: usize) -> BellmanData {
        let mut array = Vec::<Vec<u32>>::new();
        for y in 0..num_iterations  {
            let mut row = Vec::<u32>::new();
            for x in 0..num_vertex {
                let value = y*num_vertex + x;
                row.push(value.try_into().unwrap());
            }
            array.push(row);
        }

        BellmanData { array: array, num_vertex: num_vertex, num_iterations: num_iterations }
    }

    pub fn get(&self,vertex: usize, iteration: usize) -> u32 {
        self.array[iteration][vertex]

    }

    pub fn get_row_iter(&self) -> std::slice::Iter<'_, Vec<u32>>{
        self.array.iter()
    }

    pub fn set(&mut self, vertex : usize, iteration : usize , value : u32)  {
        self.array[iteration][vertex] = value;

    }

}


fn main() { 
    let vertexes = 7;
    let iterations = vertexes-1;

    let data = BellmanData::new(vertexes, iterations);
   
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

    let mut count = 0;
    for row in data.get_row_iter() {
        let row_format : String = row.iter().map(|val| format!("{:2} ",val)).collect();
        println!("Iteration {:2} :    {}", count,row_format);
        count += 1
    }

    println!("Iteration 1,3 -> {}", data.get(3,1));
    println!("Iteration 4,1 -> {}", data.get(1,4));



}
