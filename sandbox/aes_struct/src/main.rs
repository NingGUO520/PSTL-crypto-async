extern crate crypto;

use std::error::Error;
use std::fmt;
use std::cmp;
use std::sync::{Arc, RwLock};

#[derive(PartialEq)]
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
fn poll(receipt : & mut Arc<RwLock<Receipt>>) -> Vec<u8>{

    let mut text : Vec<u8> = Vec::new();
    loop {
         if receipt.read().unwrap().status == Status::Completed {
            text = receipt.read().unwrap().ciphertext.to_vec();
            break;
        }else {
            continue;
        }

    }
    return text;

}

 
impl Manager {
    fn new () -> Manager {
        return Manager {
            jobs: Vec::new(),
            min_len: usize::max_value(),
            receipts: Vec::new()
        }
    }

    fn flush_job(&mut self){
        self.jobs = Vec::new();
        self.receipts = Vec::new();
        self.min_len = usize::max_value();
    }

    fn submit_job(&mut self, job: Job) -> Arc<RwLock<Receipt>> {

        let submitted = Receipt { ciphertext: vec![ (!0) as u8; 16],
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
                self.flush_job();
            }
            
            
        }

        return p;

    }


}

fn fake_encrypt(input: &[u8], output: &mut [u8], key: &[u8], nonce: &[u8], len: usize) {

    for i in 0..len {
      output[i] = input[i] ^ key[i];
    }

}
fn main() {
    
        let mut manager = Manager::new();

        let mut input : Vec<u8> = vec![0 ,1,2,3,4,5,6,7];
        let mut keys: Vec<u8> = vec!['b' as u8; 14];
        keys.push(8 as u8);
        keys.push(9 as u8);

        let len: usize = input.len() as usize;

        let job: Job = Job {
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

        let mut job9 = job.clone();
        keys.pop();
        keys.push('b' as u8);
        job2.keys =  keys.clone();
        let mut job10 = job.clone();
        keys.pop();
        keys.push('c' as u8);
        job3.keys =  keys.clone();


        manager.submit_job(job);
        manager.submit_job(job2);
        manager.submit_job(job3);
        manager.submit_job(job4);
        manager.submit_job(job5);
        manager.submit_job(job6);
        manager.submit_job(job7);
        manager.submit_job(job8);
        for i in 0..manager.receipts.len(){
              let text = poll(& mut manager.receipts[i]);
                println!("text = {:?}",text );
        }
         manager.submit_job(job9);
        manager.submit_job(job10);


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
              let text = poll(& mut manager.receipts[i]);
                println!("text = {:?}",text );
            println!("job {} {:?}",i,manager.receipts[i].read().unwrap().ciphertext);
        }
       
        // println!("{:?}",x );


}
