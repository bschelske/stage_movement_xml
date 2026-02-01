use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    welcome_message();

    let mut running: bool = true;
    while running {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input = user_input.trim();

        match user_input {
            "q" => {
                running = false;
                println!("Quitting...");
            }
            "" => {
                create_xml();
                running = false;
                println!("Quitting...");
            }
            _ => {
                println!("Unknown input: {}", user_input);
            }
        }
    }
}

fn welcome_message() {
    println!(
        r"
     _                                                                     _   
 ___| |_ __ _  __ _  ___     _ __ ___   _____   _____ _ __ ___   ___ _ __ | |_ 
/ __| __/ _` |/ _` |/ _ \   | '_ ` _ \ / _ \ \ / / _ \ '_ ` _ \ / _ \ '_ \| __|
\__ \ || (_| | (_| |  __/   | | | | | | (_) \ V /  __/ | | | | |  __/ | | | |_ 
|___/\__\__,_|\__, |\___/   |_| |_| |_|\___/ \_/ \___|_| |_| |_|\___|_| |_|\__|
              |___/                                                        
XML Generator
Ben 2026
-------------------------------------------------------------------------------------------
"
    );
    println!("Instructions:");
    println!("Follow the prompts and press enter to continue.");
    println!("Enter 'q' to quit.");
}

fn calculate_stepsize(distance: f64, steps: u32) -> f64 {
    distance / steps as f64
}

fn time_code_filename() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    return format!("script_{}.xml", since_the_epoch.as_secs());
}

fn create_xml() {
    println!("Generating XML file...");
    let dir = "output";

    let filename = time_code_filename();

    // 1. Create the "output" folder if it doesn't exist
    if let Err(e) = fs::create_dir_all(dir) {
        eprintln!("Error creating directory: {}", e);
        return;
    }

    // 2. Combine the directory and filename
    // Path::new(dir).join(filename) handles slashes correctly for Windows or Mac/Linux
    let path = Path::new(dir).join(filename);

    // Convert path to a string to pass to your function
    let path_str = path.to_str().unwrap_or("output/script.xml");

    println!("Generating XML at {}...", path_str);

    match create_script(path_str) {
        Ok(_) => println!("Successfully created XML in the output folder!"),
        Err(e) => eprintln!("Error creating XML: {}", e),
    }
}

fn create_script(output_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup the writer (using a BufWriter for performance)
    let file = File::create(output_filename)?;
    let mut writer = Writer::new_with_indent(BufWriter::new(file), b' ', 2);

    // 2. Write XML Declaration (UTF-16 is specified in your Python, but
    // note that Rust strings are UTF-8. quick-xml handles the header tag here.)
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-16"), None)))?;

    // 3. Create Root: <variant version="1.0">
    let mut variant = BytesStart::new("variant");
    variant.push_attribute(("version", "1.0"));
    writer.write_event(Event::Start(variant))?;

    // 4. Create <no_name runtype="CLxListVariant">
    let mut no_name = BytesStart::new("no_name");
    no_name.push_attribute(("runtype", "CLxListVariant"));
    writer.write_event(Event::Start(no_name))?;

    write_sub_element(&mut writer, "bIncludeZ", "bool", "false")?;
    write_sub_element(&mut writer, "bPFSEnabled", "bool", "false")?;

    // 5. Logic Loop (Simplified coordinate mock)
    let coordinates = vec![(106464.830134385, 25188.343777459, 57159.144); 3]; // Mocking your generator

    for (index, (x, y, z)) in coordinates.iter().enumerate() {
        let tag = format!("Point{:05}", index);
        let name = format!("A{}", index + 1);

        let mut point = BytesStart::new(&tag);
        point.push_attribute(("runtype", "NDSetupMultipointListItem"));
        writer.write_event(Event::Start(point))?;

        write_sub_element(&mut writer, "bChecked", "bool", "true")?;
        write_sub_element(&mut writer, "strName", "CLxStringW", &name)?;
        write_sub_element(&mut writer, "dXPosition", "double", &format!("{:.15}", x))?;
        write_sub_element(&mut writer, "dYPosition", "double", &format!("{:.15}", y))?;
        write_sub_element(&mut writer, "dZPosition", "double", &format!("{:.15}", z))?;
        write_sub_element(&mut writer, "dPFSOffset", "double", "-1.000000000000000")?;
        write_sub_element(&mut writer, "baUserData", "CLxByteArray", "")?;

        writer.write_event(Event::End(BytesEnd::new(&tag)))?;
    }

    // 6. Close Tags
    writer.write_event(Event::End(BytesEnd::new("no_name")))?;
    writer.write_event(Event::End(BytesEnd::new("variant")))?;

    Ok(())
}

fn write_sub_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    name: &str,
    runtype: &str,
    value: &str,
) -> Result<(), quick_xml::Error> {
    let mut elem = BytesStart::new(name);
    elem.push_attribute(("runtype", runtype));
    elem.push_attribute(("value", value));
    writer.write_event(Event::Empty(elem))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_stepsize() {
        let distance = 10.0;
        let steps = 5;
        let expected_stepsize = 2.0;
        let result = calculate_stepsize(distance, steps);
        assert_eq!(result, expected_stepsize);
    }
}
