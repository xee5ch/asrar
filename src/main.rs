extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Asrar")
        .version("0.1.0")
        .author("Al S <xee5ch.gh.al@il5.in>")
        .about("pwgen clone written in Rust")
        .args_from_usage("-0, --no-numerals 'Don't include numbers in the generated passwords.
                          -1 'Print the generated passwords one per line.'
                          -A --no-capitalize 'Don't bother to include any capital letters in the generated passwords.'
                          -a --alt-phonics 'This option doesn't do anything special; it is present only for backwards compatibility.'
                          -B --ambiguous 'Don't  use  characters that could be confused by the user when printed, such as 'l' and '1', or '0' or 'O'.  This reduces the number of possible passwords significantly, and as such reduces the quality of the passwords.  It may be useful for users who have bad vision, but in general use of this option is not recommended.'
                          -c --capitalize 'Include at least one capital letter in the password.  This is the default if the standard output is a tty device.'
                          -C 'Print the generated passwords in columns.  This is the default if the standard output is a tty device.'
                          -N --num-passwords=[NUM] 'Generate num passwords.  This defaults to a screenful if passwords are printed by columns, and one password otherwise.'
                          -n --numerals 'Include at least one number in the password.  This is the default if the standard output is a tty device.'
                          -H --sha1=[FILE] 'Will use the sha1's hash of given file and the optional seed to create password. It will allow you to compute the same password later, if you remember the file, seed, and pwgen's options used.  ie: -H ~/your_favorite.mp3#your@email.com gives a list of possibles passwords for your pop3 account, and you can ask this list again and again.'
                          -r --remove-chars=[CHARS] 'Don't use the specified characters in password.  This option will disable the phomeme-based generator and uses the random password generator.'
                          -s --secure 'Generate completely random, hard-to-memorize passwords.  These should only be used for machine passwords, since otherwise it's almost guaranteed that users will simply write the password on a piece  of  paper taped to the monitor...'
                          -v --no-vowels 'Generate  random passwords that do not contain vowels or numbers that might be mistaken for vowels.  It provides less secure passwords to allow system administrators to not have to worry with random passwords accidentally contain offensive substrings.'
                          -y --symbols 'Include at least one special character in the password.'")
        .arg(Arg::with_name("pw_length")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("Number of chars to generate"))
        .arg(Arg::with_name("num_pw")
                 .required(true)
                 .takes_value(true)
                 .index(2)
                 .help("Number of passwords to generate"))
        .get_matches();
    let pw_length = matches.value_of("pw_length").unwrap();
    let num_pw = matches.value_of("num_pw").unwrap();
    println!("{} * {}", pw_length,num_pw);
}