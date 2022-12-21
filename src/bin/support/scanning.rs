use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use walkdir::WalkDir;

pub fn scan<F, T>(path: &Path, process: F, workers: usize) -> Vec<(PathBuf, io::Result<T>)>
where
    F: Fn(PathBuf) -> (PathBuf, io::Result<T>) + Copy + Send + 'static,
    T: Send + 'static,
{
    let (forward_sender, forward_receiver) = mpsc::channel::<PathBuf>();
    let (backward_sender, backward_receiver) = mpsc::channel::<(PathBuf, io::Result<T>)>();
    let forward_receiver = Arc::new(Mutex::new(forward_receiver));

    let _: Vec<_> = (0..workers)
        .map(|_| {
            let forward_receiver = forward_receiver.clone();
            let backward_sender = backward_sender.clone();
            thread::spawn(move || loop {
                let path = match forward_receiver.lock().unwrap().recv() {
                    Ok(path) => path,
                    Err(_) => break,
                };
                backward_sender.send(process(path)).unwrap();
            })
        })
        .collect();
    let mut count = 0;
    for entry in WalkDir::new(path)
        .into_iter()
        .map(|entry| entry.unwrap())
        .filter(|entry| !entry.file_type().is_dir())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .and_then(|extension| extension.to_str())
                .map(|extension| extension == "otf" || extension == "ttf")
                .unwrap_or(false)
        })
    {
        forward_sender.send(entry.path().into()).unwrap();
        count += 1;
    }
    return (0..count)
        .map(|_| backward_receiver.recv().unwrap())
        .collect();
}