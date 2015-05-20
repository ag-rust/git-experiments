extern crate git2;

use git2::Repository;
use git2::Diff;
use git2::DiffOptions;

fn main() {
    let res = Repository::open("/home/vagrant/git-add-interactive/.git");

    let r = match res {
        Ok(val) => val,
        Err(e) => panic!("could not open respository: {}", e)
    };

    println!("workdir: {:?}", r.workdir());

    let object = r.revparse_single("HEAD").unwrap();
    let head = object.as_tree();

    println!("HEAD is: {:?}", object.id());

    let mut options = DiffOptions::new();
    options.include_ignored(true);
    options.include_untracked(true);
    options.include_unmodified(false);

    let diff = Diff::tree_to_workdir_with_index(&r, head, Some(&mut options)).unwrap();

    println!("number of deltas: {}", diff.deltas().len());


    for d in diff.deltas() {
        println!("status {:?}", d.status());
        println!("orig path {:?}", d.old_file().path());
        println!("new path {:?}", d.new_file().path());
    }
}
