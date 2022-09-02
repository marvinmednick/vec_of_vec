use vec_of_vec::{VecOfVec,FlattendArray};
use vec_of_vec::MinMax::{Value,Min,Max,NA};

fn main() { 
    let vertexes = 7;
    let iterations = vertexes-1;

    let mut data : VecOfVec<u32> = VecOfVec::new(vertexes, iterations,NA);


    data.set(3,0,44);
   
    let mut count = 0;
    let header : String = (0..vertexes).map(|val| format!("{:2} ",val)).collect();
    println!("{:17} {}","Vertex",header);
    for row in data.get_row_iter() {
        let row_format : String = row.iter().map(|val| { 
            match val {
                Value(x) => format!("{:>2} ",x),
                Min    => format!("{:>2} ","m"),
                Max    => format!("{:>2} ","M"),
                NA    => format!("{:>2} ","NA"),
            }}).collect();
        println!("Iteration {:2} :    {}", count,row_format);
        count += 1
    }

    println!("Iter 0,Vertex 3 -> {}", data.get_string(3,0));
    println!("Iter 1,Vertex 3 -> {}", data.get_string(3,1));

    println!("Iter 4,Vertex 1 -> {}", data.get_string(1,4));
    println!("Iter 6,Vertex 5 -> {}", data.get_string(6,5));

    println!("using display");
    data.display("COL".to_string(),"ROW".to_string());

    data.set(1,6,44);
    data.set(2,6,22);
    data.set(3,6,4);
    data.set(4,6,100);
    data.set(5,6,17);
    data.set(6,6,-4);
    data.set(7,6,24);

    data.display("COL".to_string(),"ROW".to_string());

    println!("\n--------- Flattened Array ------- \n");
    let mut data : FlattendArray<u32> = FlattendArray::new(vertexes, iterations);

    data.set(3,0,44);
   
    let mut count = 0;
    let header : String = (0..vertexes).map(|val| format!("{:2} ",val)).collect();
    println!("{:17} {}","Vertex",header);
    for row in 0..iterations {
        let mut row_data = Vec::new();
        for col in 0..vertexes {
            row_data.push(format!("{:2} ",data.get_string(col,row)));
        }
        let row_format : String = row_data.join("");
        println!("Iteration {:2} :    {}", row,row_format);
    }

    println!("Iter 0,Vertex 3 -> {}", data.get_string(3,0));
    println!("Iter 1,Vertex 3 -> {}", data.get_string(3,1));

    println!("Iter 4,Vertex 1 -> {}", data.get_string(1,4));
    println!("Iter 6,Vertex 5 -> {}", data.get_string(6,5));

    println!("\nAs Interator \n");
        
    for row in data.get_row_iter() {
        let row_format : String = row.iter().map(|val| { 
            match val {
                Some(x) => format!("{:>2} ",x),
                None    => format!("{:>2} ","N"),
            }}).collect();
        println!("Iteration {:2} :    {}", count,row_format);
        count += 1
    }

}

