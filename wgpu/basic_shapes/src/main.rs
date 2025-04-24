use basic_shapes::run;

fn main(){
    println!("halo");
    pollster::block_on(run());
}