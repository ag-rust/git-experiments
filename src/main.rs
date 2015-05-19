extern crate git2;

use git2::Repository;
use git2::StatusOptions;

fn main() {
    let res = Repository::open(".");

    let r = match res {
        Ok(val) => val,
        Err(_) => panic!("could not open respository")
    };

    r.statuses(None)
     .unwrap()
     .iter()
     .map(|s| { println!("{:?}", s.head_to_index().unwrap().status()) })
     .collect::<Vec<_>>();
}
