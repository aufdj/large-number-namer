use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = (((&args[1].parse::<u128>().unwrap()) - 3) / 3).to_string();
    let mut num: Vec<usize> = vec![];

    // Convert input string to byte array
    let input = input.into_bytes();
    // Map number ascii codes to numbers i.e. "0" (48) -> 0
    for i in input.iter() {
        num.push((i - 48) as usize);
    }

    let conv_units: [&str; 10] = 
    ["", "mi", "bi", "tri", "quadri", "quinti", "sexti", "septi", "octi", "noni"];
    let units: [&str; 10] = 
    ["nilli", "un", "duo", "tre", "quattor", "quinqua", "se", "septen", "octo", "novem"];
    let tens: [&str; 10] =  
    ["nilli", "deci", "viginti", "triginta", "quadraginta", "quinquaginta", "sexaginta", "septuaginta", "octoginta", "nonaginta"];
    let hundreds: [&str; 10] =    
    ["nilli", "centi", "ducenti", "trecenti", "quadringenti", "quingenti", "sescenti", "septingenti", "octingenti", "nongenti"];

    let mut output = String::new();
    if input.len() % 3 == 0 {
        output.push_str(units[num[2]]);
        output.push_str(tens[num[1]]);
        output.push_str(hundreds[num[0]]);
        output.push_str("lli");
        for i in (3..num.len()).step_by(3) {
            output.push_str(units[num[i+2]]);
            output.push_str(tens[num[i+1]]);
            output.push_str(hundreds[num[i]]); 
            output.push_str("lli"); 
        }
    }
    if input.len() % 3 == 2 {
        output.push_str(units[num[0]]);
        output.push_str(tens[num[1]]);
        output.push_str("lli");
        for i in (2..num.len()).step_by(3) {
            output.push_str(units[num[i+2]]);
            output.push_str(tens[num[i+1]]);
            output.push_str(hundreds[num[i]]);
            output.push_str("lli");
        }
    }
    if input.len() % 3 == 1 {
        output.push_str(conv_units[num[0]]);
        output.push_str("lli");
        for i in (1..num.len()).step_by(3) {
            output.push_str(units[num[i+2]]);
            output.push_str(tens[num[i+1]]);
            output.push_str(hundreds[num[i]]);
            output.push_str("lli");
        }
    }
    output.push_str("z"); // Intermediate value for replacing

    output = output
    .replace("treviginti", "tresviginti")
    .replace("tretriginta", "trestriginta")
    .replace("trequadraginta", "tresquadraginta")
    .replace("trequinqaginta", "tresquinquagainta")
    .replace("treoctaginta", "tresoctaginta")

    .replace("seviginti", "sesviginti")
    .replace("setriginta", "sestriginta")
    .replace("sequadraginta", "sesquadraginta")
    .replace("sequinqaginta", "sesquinquagainta")
    .replace("seoctaginta", "sesoctaginta")

    .replace("septennonaginta", "septenonaginta")

    .replace("novemnonaginta", "novenonaginta")

    .replace("illiz", "illion")
    .replace("z", "llion")

    // These strings are prefixes of one another, so they need 
    // to be differentiated to avoid cascading replacements
    .replace("nonilli", "no[illi")
    .replace("nillinillinilli", "n}lli")
    .replace("nillinilli", "n{lli")
    .replace("nilli", "n~lli")
    .replace("n}lli", "nilli")
    .replace("n{lli", "")
    .replace("n~lli", "")

    .replace("ellion", "illion")
    .replace("allion", "illion")

    .replace("no[illi", "nonilli")

    .replace("illilli", "illi");
        
    println!("10^{} = {}", &args[1], output);   
}
