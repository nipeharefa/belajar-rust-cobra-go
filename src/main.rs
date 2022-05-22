

struct Command {
    // name: String,
    key: String,
    // args: Vec<String>,
    commands: Vec<Command>,
    run_cmd: RunCommand
}


fn new_command(k: String, callback: RunCommand) -> Command {
    // let mut commands = Vec::new();
    let c = Command {
        // name: String::new(),
        key: k,
        // args: Vec::new(),
        commands: Vec::new(),
        run_cmd: callback,
    };

    return c;
}

fn default() {
    println!("Hello");
}

type RunCommand = fn();

// impl Example for Command {}
impl Command {

    fn add(&mut self, cmd: Command){
        self.commands.push(cmd);
    }

    fn total_command(&self) -> usize {
        return self.commands.len();
    }
}

trait Executor {
    fn run (&self, arg: String);
}

impl Executor for Command {
    fn run(&self, arg: String) {
        for name in self.commands.iter() {

            let a = name.key.eq(&arg);
            if a {
                (name.run_cmd)();
            }
        }
    }
}


fn main() {

    let mut a = new_command(String::new(), default);


    let help_callback = || {
        println!("help");
    };
    let help_cmd = new_command(String::from("help"), help_callback);

    a.add(help_cmd);
    println!("Total comand {}", a.total_command());
    
    let mut is_running = true;

    while is_running {
        let mut input = String::new();
        println!("Enter command: ");
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input == "" {
            is_running = false;
        } else {
            let arg = String::from(input);
            a.run(arg);
        }
    }
}
