use quick_xml::Writer;
use quick_xml::events::{BytesEnd, BytesStart, Event};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

struct ScanParams {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    steps: usize,
}

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
                // 1. Get the parameters from the user
                if let Some(params) = get_user_coords() {
                    // 2. Generate the mathematical points
                    let points = generate_points(&params);
                    // 3. Create the XML using those points
                    create_xml(points);
                    running = false;
                    println!("Quitting...");
                } else {
                    println!("Invalid input. Please use the format: x1, y1, x2, y2, steps");
                    println!("Press enter to try again. HINT: Press up arrow to get last input.");
                    // Loop will continue so user can try again
                }
            }
            _ => {
                println!("Unknown input: {}", user_input);
                println!("Press enter to continue... or type q to quit.");
            }
        }
    }

    println!("Press Enter to close this window...");

    let mut exit_wait = String::new();
    io::stdin()
        .read_line(&mut exit_wait)
        .expect("Failed to read line");
}

fn welcome_message() {
    println!(
        r"     _                                                                     _   
 ___| |_ __ _  __ _  ___     _ __ ___   _____   _____ _ __ ___   ___ _ __ | |_ 
/ __| __/ _` |/ _` |/ _ \   | '_ ` _ \ / _ \ \ / / _ \ '_ ` _ \ / _ \ '_ \| __|
\__ \ || (_| | (_| |  __/   | | | | | | (_) \ V /  __/ | | | | |  __/ | | | |_ 
|___/\__\__,_|\__, |\___/   |_| |_| |_|\___/ \_/ \___|_| |_| |_|\___|_| |_|\__|
              |___/                                                        
XML Generator
Ben 2026      
[]->[]->[]->[]->[]->[]->[]->[]->[]->[]->[]->[]->[]->[]->[]->[]->[]->[]->[]->[]->[]->[]
"
    );
    println!("This tool generates XML files which give a list of coordinates for ND2 Acquisition.");
    println!("Each time you run the program, an XML file will be created in the 'output' folder.");
    println!(
        "It may be useful to have a notepad open with your desired coordinates for copy pasting. 
        
Example: x1, y1, x2, y2, steps"
    );

    println!("The coordinates MUST be in millimeters (mm).");

    println!(
        "
Continue (press Enter) or quit (type 'q' and press Enter):"
    );
}

fn get_user_coords() -> Option<ScanParams> {
    println!("Enter coordinates: x1, y1, x2, y2, steps:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;

    // Split by comma, collect into a Vec of trimmed strings
    let parts: Vec<&str> = input.split(',').map(|s| s.trim()).collect();

    if parts.len() != 5 {
        println!("Error: Expected 5 values, got {}.", parts.len());
        return None;
    }

    // Parse each part, handling potential errors gracefully
    let x1 = parts[0].parse::<f64>().ok()?;
    let y1 = parts[1].parse::<f64>().ok()?;
    let x2 = parts[2].parse::<f64>().ok()?;
    let y2 = parts[3].parse::<f64>().ok()?;
    let steps = parts[4].parse::<usize>().ok()?;

    Some(ScanParams {
        x1,
        y1,
        x2,
        y2,
        steps,
    })
}

fn generate_points(params: &ScanParams) -> Vec<(f64, f64)> {
    let mut points = Vec::new();

    if params.steps <= 1 {
        points.push((params.x1, params.y1));
        return points;
    }

    let dx = (params.x2 - params.x1) / (params.steps - 1) as f64;
    let dy = (params.y2 - params.y1) / (params.steps - 1) as f64;

    for i in 0..params.steps {
        // Multiply by 1000.0 to convert mm to microns (if that's the discrepancy)
        let x = (params.x1 + (dx * i as f64)) * 1000.0;
        let y = (params.y1 + (dy * i as f64)) * 1000.0;
        points.push((x, y));
    }
    points
}

fn time_code_filename(dir: &str) -> String {
    let mut counter = 0;
    let mut filename = String::from("script.xml");

    while Path::new(dir).join(&filename).exists() {
        counter += 1;
        filename = format!("script_{}.xml", counter);
    }
    filename
}

fn create_xml(points: Vec<(f64, f64)>) {
    println!("Generating XML file...");
    let dir = "output";

    let filename = time_code_filename(&dir);

    // 1. Create the "output" folder if it doesn't exist
    if let Err(e) = fs::create_dir_all(dir) {
        eprintln!("Error creating directory: {}", e);
        return;
    }

    // 2. Combine the directory and filename
    let path = Path::new(dir).join(filename);

    // Convert path to a string to pass to your function
    let path_str = path.to_str().unwrap_or("output/script.xml");

    println!("Generating XML at {}...", path_str);

    match create_script(path_str, points) {
        Ok(_) => println!("Successfully created XML in the output folder!"),
        Err(e) => eprintln!("Error creating XML: {}", e),
    }
}

fn create_script(
    output_filename: &str,
    coordinates: Vec<(f64, f64)>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create the XML string in memory first (Rust strings are UTF-8)
    let mut buffer = Vec::new();
    let mut writer = Writer::new(&mut buffer);

    // No Declaration here, we'll add it manually to match your Python logic
    let mut variant = BytesStart::new("variant");
    variant.push_attribute(("version", "1.0"));
    writer.write_event(Event::Start(variant))?;

    let mut no_name = BytesStart::new("no_name");
    no_name.push_attribute(("runtype", "CLxListVariant"));
    writer.write_event(Event::Start(no_name))?;

    write_sub_element(&mut writer, "bIncludeZ", "bool", "false")?;
    write_sub_element(&mut writer, "bPFSEnabled", "bool", "false")?;

    for (index, (x, y)) in coordinates.iter().enumerate() {
        let tag = format!("Point{:05}", index);
        let name = format!("A{}", index + 1);

        let mut point = BytesStart::new(&tag);
        point.push_attribute(("runtype", "NDSetupMultipointListItem"));
        writer.write_event(Event::Start(point))?;

        write_sub_element(&mut writer, "bChecked", "bool", "true")?;
        write_sub_element(&mut writer, "strName", "CLxStringW", &name)?;
        write_sub_element(&mut writer, "dXPosition", "double", &format!("{:.15}", x))?;
        write_sub_element(&mut writer, "dYPosition", "double", &format!("{:.15}", y))?;
        write_sub_element(&mut writer, "dZPosition", "double", &format!("{:.15}", 0.0))?;
        write_sub_element(&mut writer, "dPFSOffset", "double", "-1.000000000000000")?;
        write_sub_element(&mut writer, "baUserData", "CLxByteArray", "")?;

        writer.write_event(Event::End(BytesEnd::new(&tag)))?;
    }

    writer.write_event(Event::End(BytesEnd::new("no_name")))?;
    writer.write_event(Event::End(BytesEnd::new("variant")))?;

    // 2. Prepare the final string with the Header
    let xml_content = String::from_utf8(buffer)?;
    let final_xml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-16\"?>\n{}",
        xml_content
    );

    // 3. Convert to UTF-16 Little Endian
    let utf16_encoded: Vec<u16> = final_xml.encode_utf16().collect();

    // 4. Write to file with the BOM (0xFF 0xFE)
    let mut file = File::create(output_filename)?;
    file.write_all(&[0xFF, 0xFE])?; // THE CRITICAL PART: BOM

    // Convert u16 vector to u8 bytes (Little Endian)
    for &u in &utf16_encoded {
        file.write_all(&u.to_le_bytes())?;
    }

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

// STRATEGY:
// - User provides start and end coordinates + steps
// - Calculate stepsize for each axis
// - Generate list of coordinates
// - Write XML file with those coordinates

// COOL FEATURE IDEA:
// - Remembner last used coordinates and steps in a config file
// - Reverse direction option (start -> end or end -> start)
