extern crate crypto;

use std::error::Error;
use std::fmt;
use std::cmp;
use std::sync::{Arc, RwLock};
//use crypto::aes::{self, KeySize};

enum Status {
    BeingProcessed,
    Completed
}

#[derive(Clone)]
struct Job{
    plaintext: Vec<u8>,
    iv: [u8;16],
    keys: Vec<u8>,
    len: usize
}

struct Receipt {
    ciphertext: Vec<u8>,
    status: Status,
}

struct Manager {
    jobs: Vec<Job>,
    min_len: usize,
    receipts: Vec<Arc<RwLock<Receipt>>>,
}

impl Manager {
    fn new () -> Manager {
        return Manager {
            jobs: Vec::new(),
            min_len: usize::max_value(),
            receipts: Vec::new()
        }
    }

    fn submit_job(&mut self, job: Job) -> Arc<RwLock<Receipt>> {

        let submitted = Receipt { ciphertext: vec![ (!0) as u8; 20],
                                  status: Status::BeingProcessed };
        let p = Arc::new(RwLock::new(submitted));
        self.receipts.push(p.clone());
        self.min_len = cmp::min(self.min_len, job.len);
        self.jobs.push(job);

        println!("min_len = {:?}",self.min_len );
        if self.jobs.len() == 8 {

            // Batch encryption (faked)
            for (i, job) in self.jobs.iter().enumerate() { // XXX: get rid of clone() here
                fake_encrypt(&job.plaintext,
                             &mut Arc::clone(&self.receipts[i]).write().unwrap().ciphertext,
                             &job.keys,
                             &job.iv,
                             self.min_len);
            }
            
            for i in 0..self.jobs.len() {
                self.jobs[i].len -= self.min_len;
                if self.jobs[i].len == 0 {
                    Arc::clone(&self.receipts[i]).write().unwrap().status = Status::Completed;
                }
            }

            let mut jobDone = true ;
            // BUG: if *all* the jobs are done, we must reset min_len
             for i in 0..self.jobs.len() {
               
                if self.jobs[i].len != 0 {
                    jobDone = false;
                   
                }
            }
            if jobDone {
                println!("all jobs are done");
                self.min_len = usize::max_value();
            }
             for i in 0..self.receipts.len(){
            println!("job {} {:?}",i,self.receipts[i].read().unwrap().ciphertext);
            }

            
        }

        return p;

    }


}

fn fake_encrypt(input: &[u8], mut output: &mut [u8], key: &[u8], nonce: &[u8], len: usize) {

    for i in 0..len {
      output[i] = input[i] ^ key[i];
    }
    println!("input = {:?}",input );
    println!("key = {:?}",key );
    println!("output = {:?}",output );
}

// fn test(){
//     let plain = vec![0 as u8; 14];
//     let keys = plain.clone;
//     let output = vec![0 as u8; 20];

//      fake_encrypt(&job.plaintext,
//                              &mut Arc::clone(&self.receipts[i]).write().unwrap().ciphertext,
//                              &job.keys,
//                              &job.iv,
//                              self.min_len);
// }
fn main() {
    

    // let mut args = build_Args();
    // let mut manager = build_Manager(args);

    // let mut input: Vec<u8> = Vec::new();
    //     input.push('a' as u8 );input.push('a');input.push('a');input.push('a');input.push('a');
    //     input.push('a');input.push('a');input.push('a');input.push('a');input.push('a');
    //     input.push('a');input.push('a');input.push('a');input.push('a');
        let mut manager = Manager::new();

        let mut input : Vec<u8> = vec!['a' as u8; 14];
        let  output: Vec<u8> = input.clone();
        let mut keys: Vec<u8> = vec!['b' as u8; 14];
        keys.push(8 as u8);
        keys.push(9 as u8);

        let len: usize = input.len() as usize;

        let mut job: Job = Job {
            plaintext: input.clone(),
            iv: [0;16],
            len: len,
            keys: keys.clone(),
        };

        let mut job2 = job.clone();
        keys.pop();
        keys.push('b' as u8);
        job2.keys =  keys.clone();
        let mut job3 = job.clone();
        keys.pop();
        keys.push('c' as u8);
        job3.keys =  keys.clone();
        let mut job4 = job.clone();
        keys.pop();
        keys.push('d' as u8);
        job4.keys =  keys.clone();
        let mut job5 = job.clone();
        keys.pop();
        keys.push('e' as u8);
        job5.keys =  keys.clone();
        let mut job6 = job.clone();
        keys.pop();
        keys.push('f' as u8);
        job6.keys =  keys.clone();
        let mut job7 = job.clone();
        keys.pop();
        keys.push('g' as u8);
        job7.keys =  keys.clone();
        let mut job8 = job.clone();
        keys.pop();
        keys.push('h' as u8 );
        input.push('b' as u8);
        job8.plaintext = input.clone();
        job8.keys =  keys.clone();

        manager.submit_job(job);
        manager.submit_job(job2);
        manager.submit_job(job3);
        manager.submit_job(job4);
        manager.submit_job(job5);
        manager.submit_job(job6);
        manager.submit_job(job7);
        manager.submit_job(job8);

        // println!("job1: {:?}", job.ciphertext);
        // println!("job2: {:?}", job2.ciphertext);
        // println!("job3: {:?}", job3.ciphertext);
        // println!("job4: {:?}", job4.ciphertext);
        // println!("job5: {:?}", job5.ciphertext);
        // println!("job6: {:?}", job6.ciphertext);
        // println!("job7: {:?}", job7.ciphertext);
        // println!("job8: {:?}", job8.ciphertext);
        // let x  =  (0 as u8 ) ^ ('b' as u8);
        for i in 0..manager.receipts.len(){
            println!("job {} {:?}",i,manager.receipts[i].read().unwrap().ciphertext);
        }
        // println!("{:?}",x );


}
