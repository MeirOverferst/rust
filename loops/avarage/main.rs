fn main(){



    let arr: [u32;10]=[1,2,3,4,5,6,7,8,9,10];
    let sum:f32 =arr.iter().sum();
    // let avg:f32 = sum /(arr.len() as u32);
    let avg:f32 = sum /arr.len() as f32;
    // println!("{}",sumer);
    println!("{}",avg);

   

}

