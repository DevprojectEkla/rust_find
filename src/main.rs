use rust_cli::cli_utils::info;
use rust_cli::cli_utils::{print_results, print_c, print_c_no_nl,RED, CYAN, YELLOW, WHITE, GREEN};
use walkdir::WalkDir;
use std::env;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::sync::{Arc, Mutex};


///this struct allows you to initiate a Search for a target file or a directory starting from a given
///directory source, it displays a spinner during the search and display the results along the way
///and in the end gives a numbered list of the total results
pub struct Search <'a> {// we need to pass a lifetime 'a to the struct in order to pass references &str
// note that &str is good for data that won't change, we don't need to manipulate them by taking
// ownership of their value

    source: &'a str,
    target: &'a str,
    extension: &'a str,
    results_dir: Arc<Mutex<Vec<String>>>,
    results_file: Arc<Mutex<Vec<String>>>,
    job_completed: Arc<Mutex<bool>>,

}
impl <'a> Search <'a>{
    pub fn new(source:&'a str, target: &'a str, extension:&'a str)-> Self {
        let results_array_dir = Vec::new();
        let results_array_file = Vec::new();
        Self {  source, 
                target,
                extension,
                results_dir: Arc::new(Mutex::new(results_array_dir)), 
                results_file:  Arc::new(Mutex::new(results_array_file)),
                job_completed: Arc::new(Mutex::new(false)),
        }
    }
    fn info_search(&mut self) {
        info(format!("Searching for {} in {}",self.source, self.target).as_str());
        
    }
    fn search(&mut self) -> JoinHandle<()> {
        self.info_search();
        //following is important for the borrow checker the self.fields cannot be accessed
        //outside the scope of the method but the thread lives longer so the .to_string() allows a
        //clone of the &str which is only a ref
        let source = self.source.to_string();
        let target = self.target.to_string();
        let results_dir_cloned = self.results_dir.clone();
        let results_file_cloned = self.results_file.clone();
            let handle = thread::spawn(move || {
        for entry in WalkDir::new(&source) {
            if let Ok(entry) = entry {
                let path = entry.path();
                let path_str = path.to_string_lossy().to_string();
            
            if path_str.contains(&target) {
                if path.is_dir() {
                    results_dir_cloned.lock().unwrap().push(path_str.clone());
                    print_c(&format!("\rdirectory was found: {}", path_str.clone()),GREEN);
                } else {
                    if path.file_name().unwrap().to_string_lossy().to_string().contains(&target) {
                    results_file_cloned.lock().unwrap().push(path_str.clone());
                    print_c(&format!("\rfile was found: {}", path_str.clone()), GREEN);}
                }
            }
        

    }
}

            });

        handle
    }

fn display_results(&mut self) {
    let results_dir_locked = self.results_dir.lock().unwrap();
    let results_file_locked = self.results_file.lock().unwrap();

    print_results(&results_dir_locked, "\rThe target directories are:", format!("\rthe directory {} was not found", self.target).as_str());
    print_results(&results_file_locked, "\rThe target files are:", format!("\rthe file {} was not found", self.target).as_str());
    print_c("\rSearch completed", CYAN);
}

    

    fn display_spinner(&mut self, label: &str) -> thread::JoinHandle<()> {
        let spinner: Vec<&str> = vec!["-", "/", "|", "\\"];
        let label_clone = label.to_string();
        let job = self.job_completed.clone();
    
        let handle = thread::spawn(move || {
            //the * is a dereferencing operator which returns the value of the reference, in that
            //case the bool of the Mutex variable, .lock() is provided by Mutex and ensure that the
            //data is not accessed by another thread it returns a Result so we need to unwrap it
            //and get the Result if it Ok or a panic if there was an error
            while !*job.lock().unwrap() {
                for char in &spinner {
                    let text_spinner = format!("\r{} {}", label_clone, char);
                    print_c_no_nl(&text_spinner, YELLOW);
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    thread::sleep(Duration::from_millis(100));
            }
        }
    });
        handle

}
///this is the only public method that you need on this struct to get it working
/// it spawns two threads, one for the search and another one to display a simple spinner during
/// the search
pub fn start(&mut self) {
        let thread1 = self.display_spinner("Searching".as_ref());
        let thread2 = self.search();
        thread2.join().unwrap();
        *self.job_completed.lock().unwrap() = true;
        thread1.join().unwrap();
        self.display_results();
    }
    
}



fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        print_c("Usage: rufind source_directory target_directory [extension]", CYAN);
        std::process::exit(1);
    }
    let args = env::args().collect::<Vec<String>>();
    let source:  &str = args[1].as_str();
    let target:  &str = args[2].as_str();
    let mut extension = "*".as_ref();
    if args.len() == 4 {
    extension = args[3].as_str();}
    
    let mut search = Search::new(source, target, extension);
    search.start()
}

    
