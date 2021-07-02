use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let int_input = ((&args[1].parse::<u128>().unwrap()) - 3) / 3;
    let input = int_input.to_string();
    let mut output: String = String::from("");

    // hardcoded first 9 values
    if int_input == 1 { output.push_str("million"); println!("Output: {}", output); process::exit(0); }
    else if int_input == 2 { output.push_str("billion"); println!("Output: {}", output); process::exit(0); }
    else if int_input == 3 { output.push_str("trillion"); println!("Output: {}", output); process::exit(0); }
    else if int_input == 4 { output.push_str("quadrillion"); println!("Output: {}", output); process::exit(0); }
    else if int_input == 5 { output.push_str("quintillion"); println!("Output: {}", output); process::exit(0); }
    else if int_input == 6 { output.push_str("sextillion"); println!("Output: {}", output); process::exit(0); }
    else if int_input == 7 { output.push_str("septillion"); println!("Output: {}", output); process::exit(0); }
    else if int_input == 8 { output.push_str("octillion"); println!("Output: {}", output); process::exit(0); }
    else if int_input == 9 { output.push_str("nonillion"); println!("Output: {}", output); process::exit(0); }
    
    let mut num: Vec<usize> = vec![];

    for i in 0..input.len() {
        num.push((input[i..i+1].as_bytes()[0] - 48).into());
    }
    let conv_units: [String; 10] =  [String::from(""), String::from("mi"), String::from("bi"), String::from("tri"), String::from("quadri"), 
                                     String::from("quinti"), String::from("sexti"), String::from("septi"), String::from("octi"), String::from("noni")];
    let units: [String; 10] =       [String::from("nilli"), String::from("un"), String::from("duo"), String::from("tre"), String::from("quattor"), 
                                     String::from("quinqua"), String::from("se"), String::from("septen"), String::from("octo"), String::from("novem")];
    let tens: [String; 10] =        [String::from("nilli"), String::from("deci"), String::from("viginti"), String::from("triginta"), String::from("quadraginta"), 
                                     String::from("quinquaginta"), String::from("sexaginta"), String::from("septuaginta"), String::from("octoginta"), String::from("nonaginta")];
    let hundreds: [String; 10] =    [String::from("nilli"), String::from("centi"), String::from("ducenti"), String::from("trecenti"), String::from("quadringenti"), 
                                     String::from("quingenti"), String::from("sescenti"), String::from("septingenti"), String::from("octingenti"), String::from("nongenti")];

    if input.len() % 3 == 0 {
        output.push_str(units[num[2]].as_str());
        output.push_str(tens[num[1]].as_str());
        output.push_str(hundreds[num[0]].as_str());
        output.push_str("lli");
        for i in (3..num.len()).step_by(3) {
            output.push_str(units[num[i+2]].as_str());
            output.push_str(tens[num[i+1]].as_str());
            output.push_str(hundreds[num[i]].as_str()); 
            output.push_str("lli"); 
        }
    }
    if input.len() % 3 == 2 {
        output.push_str(units[num[0]].as_str());
        output.push_str(tens[num[1]].as_str());
        output.push_str("lli");
        for i in (2..num.len()).step_by(3) {
            output.push_str(units[num[i+2]].as_str());
            output.push_str(tens[num[i+1]].as_str());
            output.push_str(hundreds[num[i]].as_str());
            output.push_str("lli");
        }
    }
    if input.len() % 3 == 1 {
        output.push_str(conv_units[num[0]].as_str());
        output.push_str("lli");
        for i in (1..num.len()).step_by(3) {
            output.push_str(units[num[i+2]].as_str());
            output.push_str(tens[num[i+1]].as_str());
            output.push_str(hundreds[num[i]].as_str());
            output.push_str("lli");
        }
    }
    output.push_str("z"); // intermediate value for replacing

        output = output.replace("treviginti", "tresviginti");
        output = output.replace("tretriginta", "trestriginta");
        output = output.replace("trequadraginta", "tresquadraginta");
        output = output.replace("trequinqaginta", "tresquinquagainta");
        output = output.replace("treoctaginta", "tresoctaginta");

        output = output.replace("seviginti", "sesviginti");
        output = output.replace("setriginta", "sestriginta");
        output = output.replace("sequadraginta", "sesquadraginta");
        output = output.replace("sequinqaginta", "sesquinquagainta");
        output = output.replace("seoctaginta", "sesoctaginta");

        output = output.replace("septennonaginta", "septenonaginta");

        output = output.replace("novemnonaginta", "novenonaginta");

        output = output.replace("illiz", "illion");
        output = output.replace("z", "llion");

        // these strings are prefixes of one another, so they need to be differentiated to avoid cascading replacements
        output = output.replace("nonilli", "no[illi");

        output = output.replace("nillinillinilli", "n}lli");
        output = output.replace("nillinilli", "n{lli");
        output = output.replace("nilli", "n~lli");
        output = output.replace("n}lli", "nilli");
        output = output.replace("n{lli", "");
        output = output.replace("n~lli", "");

        output = output.replace("ellion", "illion");
        output = output.replace("allion", "illion");

        output = output.replace("no[illi", "nonilli");

        output = output.replace("illilli", "illi");
        
    println!("10^{} = {}", &args[1], output);  
}
