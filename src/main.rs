

struct BellmanData {
    array : Vec::<Vec<u32>>,``
    num_vertex: u32,
    num_iterations: u32

}

impl BellmanData {

    pub fn new(num_vertex: u32, num_iterations: u32) -> Self {
        let array = Vec::<Vec<u32>>::new();
        // TODO -- Initialize
        BellmanData { array: array, num_vertex: num_vertex, num_iterations: num_iterations }
    }

    pub fn get(vertex, iteration) -> u32 {

    }

    pub fn set(vertex, iteration, value)  {

    }

}


fn main() { 
    let vertexes = 7;
    let iterations = vertexes-1;
   
    let mut xy_array = Vec::<Vec<u32>>::new();
    for y in 0..iterations  {
        let mut row = Vec::<u32>::new();
        for x in 0..vertexes {
            let value = y*vertexes + x;
            row.push(value);

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
}
