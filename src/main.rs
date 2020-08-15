use clap::Clap;
use rand::Rng;

/// This program can say 'hello' to you, show you 'quote' of the day
/// and can even help you 'add' up numbers!
#[derive(Clap)]
#[clap(version = "1.0", author = "Robin Wang<mytechtip@github>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Hello(Hello),
    Quote(Quote),
    Add(Add),
}

/// Say hello
#[derive(Clap)]
struct Hello {
    /// Your name
    #[clap(default_value="World")]
    person: String
}

/// Get a number of quotes to start your day
#[derive(Clap)]
struct Quote {
    /// The number of quotes you want
    #[clap(default_value="1")]
    number: u8
}


/// Add up a list of numbers
#[derive(Clap)]
struct Add {
    /// The list of numbers to be added
    input: Vec<i32>
}

pub trait Display {
    fn show(&self) -> String;
}

impl Display for Hello {
    fn show(&self) -> String {
        format!("Hello {}!", self.person)
    }
}

impl Display for Quote {
    fn show(&self) -> String {

        let quotes = load_quotes();
        let mut all = String::new();
        let mut i = 0;
        let mut rng = rand::thread_rng();

        while i<self.number {
            all += &format!("Quote of the day: {}\n", quotes[rng.gen_range(0, quotes.len())]);
            i += 1;
        }
        all
    }
}

impl Display for Add {
    fn show(&self) -> String {
        // add the value together
        let mut sum : i32 = 0;
        for it in &self.input {
            sum += it;
        }
        format!("Sum is {}", sum)
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Hello(t) => {
            println!("{}", t.show());
        }
        SubCommand::Quote(t) => {
            println!("{}", t.show());
        }
        SubCommand::Add(t) => {
            println!("{}", t.show());
        }
    }
}

fn load_quotes() -> Vec<String> {
    // init a list of quotes courtesy of https://type.fit/api/quotes
    let mut quotes : Vec<String> = Vec::new();
    quotes.push("Genius is one percent inspiration and ninety-nine percent perspiration. - Thomas Edison".to_string());
    quotes.push("You can observe a lot just by watching. - Yogi Berra".to_string());
    quotes.push("A house divided against itself cannot stand. - Abraham Lincoln".to_string());
    quotes.push("Difficulties increase the nearer we get to the goal. - Johann Wolfgang von Goethe".to_string());
    quotes.push("Fate is in your hands and no one elses - Byron Pulsifer".to_string());
    quotes.push("Be the chief but never the lord. - Lao Tzu".to_string());
    quotes.push("Nothing happens unless first we dream. - Carl Sandburg".to_string());
    quotes.push("Well begun is half done. - Aristotle".to_string());
    quotes.push("Life is a learning experience, only if you learn. - Yogi Berra".to_string());
    quotes.push("Self-complacency is fatal to progress. - Margaret Sangster".to_string());
    quotes.push("Peace comes from within. Do not seek it without. - Buddha".to_string());
    quotes.push("What you give is what you get. - Byron Pulsifer".to_string());
    quotes.push("We can only learn to love by loving. - Iris Murdoch".to_string());
    quotes.push("Life is change. Growth is optional. Choose wisely. - Karen Clark".to_string());
    quotes.push("You'll see it when you believe it. - Wayne Dyer".to_string());
    quotes.push("Today is the tomorrow we worried about yesterday. - null".to_string());
    quotes.push("It's easier to see the mistakes on someone else's paper. - null".to_string());
    quotes.push("Every man dies. Not every man really lives. - null".to_string());
    quotes.push("To lead people walk behind them. - Lao Tzu".to_string());
    quotes.push("Having nothing, nothing can he lose. - William Shakespeare".to_string());
    quotes.push("Trouble is only opportunity in work clothes. - Henry J. Kaiser".to_string());
    quotes.push("A rolling stone gathers no moss. - Publilius Syrus".to_string());
    quotes.push("Ideas are the beginning points of all fortunes. - Napoleon Hill".to_string());
    quotes.push("Everything in life is luck. - Donald Trump".to_string());
    quotes.push("Doing nothing is better than being busy doing nothing. - Lao Tzu".to_string());
    quotes.push("Trust yourself. You know more than you think you do. - Benjamin Spock".to_string());
    quotes.push("Study the past, if you would divine the future. - Confucius".to_string());
    quotes.push("The day is already blessed, find peace within it. - null".to_string());
    quotes.push("From error to error one discovers the entire truth. - Sigmund Freud".to_string());
    quotes.push("Well done is better than well said. - Benjamin Franklin".to_string());
    quotes.push("Bite off more than you can chew, then chew it. - Ella Williams".to_string());
    quotes.push("Work out your own salvation. Do not depend on others. - Buddha".to_string());
    quotes.push("One today is worth two tomorrows. - Benjamin Franklin".to_string());
    quotes.push("Once you choose hope, anythings possible. - Christopher Reeve".to_string());
    quotes.push("God always takes the simplest way. - Albert Einstein".to_string());
    quotes.push("One fails forward toward success. - Charles Kettering".to_string());
    quotes.push("From small beginnings come great things. - null".to_string());
    quotes.push("Learning is a treasure that will follow its owner everywhere - Chinese proverb".to_string());
    quotes.push("Be as you wish to seem. - Socrates".to_string());
    quotes.push("The world is always in movement. - V. Naipaul".to_string());
    quotes.push("Never mistake activity for achievement. - John Wooden".to_string());
    quotes.push("What worries you masters you. - Haddon Robinson".to_string());
    quotes.push("One faces the future with ones past. - Pearl Buck".to_string());
    quotes.push("Goals are the fuel in the furnace of achievement. - Brian Tracy".to_string());
    quotes.push("Who sows virtue reaps honour. - Leonardo da Vinci".to_string());
    quotes.push("Be kind whenever possible. It is always possible. - Dalai Lama".to_string());
    quotes.push("Talk doesn't cook rice. - Chinese proverb".to_string());
    quotes.push("He is able who thinks he is able. - Buddha".to_string());
    quotes.push("A goal without a plan is just a wish. - Larry Elder".to_string());
    quotes.push("To succeed, we must first believe that we can. - Michael Korda".to_string());
    quotes
}