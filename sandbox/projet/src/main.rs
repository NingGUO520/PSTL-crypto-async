use queue::Queue;
use std::ptr;

#[derive(Copy, Clone)]
enum Status{
	BeingProcessed,
	Completed,
}

struct AesAgs {
	input: [Vec<char>;8],
	output: [Vec<char>;8],
	iv: [Vec<char>;8],
	keys: [Vec<char>;8]

}


#[derive(Clone)]
struct AesJob {
	plain : Vec<char>,
	cipher : Vec<char>,
	keys : Vec<char>,
	iv: [Vec<char>;8],
	len: u32,
	status: Status,

}
struct Manager{
	args: AesAgs,
	lens : [usize;8],
	unused_lane : Queue<usize>,
	jobs_in_lane : [AesJob;8]
}



fn get_min(lens : [usize;8])->(usize, usize){
	let mut min:usize = lens[0];
	let mut min_index:usize = 0;
	for i in 1..8{
		if lens[i]< min {
			min = lens[i];
			min_index = i;
		}
	}
	(min,min_index)
}

fn submit_job( mut state : Manager , mut job :AesJob){
	let lane = state.unused_lane.dequeue().unwrap();
	state.jobs_in_lane[lane] = job.clone();
	state.args.input[lane] = job.plain;
	state.args.output[lane] = job.cipher;
	state.args.keys[lane] = job.keys;
	state.args.iv = job.iv;

	job.status = Status::BeingProcessed;
	
	let (min, min_index) = get_min(state.lens);
	
	// loop 0..min
	// 	EncryptX8(input[i],output[i],..)
	for i in 0..8{
		state.lens[i] -=min ; 
	}

	for i in 0..8{
		if state.lens[i]==0 {
			state.jobs_in_lane[i].status = Status::Completed;
			// state.jobs_in_lane[i] =  ptr::null();
			state.unused_lane.queue(i).unwrap();		
		}
	}
}
fn main() {

 //    let mut string = String::new();
	// let v2:Vec<char> = Vec::with_capacity(16);
	// // let mut args = AesAgs{
	// 	input: [[0];8],
	// 	output: [[0];8],
	// 	iv: [v2;8],
	// 	keys: [[0];8]
	// };

	// let mut manager = Manager{
	// 	args: AesAgs,
	// 	lens : [usize;8],
	// 	unused_lane : Queue<usize>,
	// 	jobs_in_lane : 
	// } 






    println!("Hello, world!");
}
