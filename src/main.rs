use rodio;
use std::io;
use std::fs;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main(){
    let mut dir_name = String::new();
    let pattern = std::env::args().nth(1);
    match pattern{
        Some(x) => dir_name = String::from("Music/")+&x,
        None => dir_name = String::from("Music")
    }

    // Music Part
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let mut paths = vec![];

    for entry in fs::read_dir(dir_name.as_str()).unwrap() {
        let dir = entry.unwrap();

        if !dir.metadata().unwrap().is_dir(){
            paths.push(dir.path())
        }
    }
    //Shuffles paths
    paths.shuffle(&mut thread_rng());
    loop{
        for i in &paths{
            let file = std::fs::File::open(i).unwrap();
            sink.append(rodio::Decoder::new(io::BufReader::new(file)).unwrap());
        }

        sink.sleep_until_end();
    }
    /*loop{

        print!("Enter command: ");

        // The IO Part
        io::stdout().flush().unwrap();
        let mut us_input = String::new();
        io::stdin().read_line(&mut us_input);

        // Matches stuff
        match us_input.as_str().trim(){
            "play"=>println!("Yay"),
            _=>println!("Command Not Found")
        }
    }*/
}
