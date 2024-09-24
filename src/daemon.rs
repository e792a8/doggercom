use daemonize::Daemonize;

pub fn daemonize() {
    let daemonize = Daemonize::new()
        .pid_file("/tmp/doggercom.pid")
        .working_directory("/tmp");
    match daemonize.start() {
        Ok(_) => println!("Daemonized."),
        Err(e) => eprintln!("Daemonization error, {}", e),
    }
}
