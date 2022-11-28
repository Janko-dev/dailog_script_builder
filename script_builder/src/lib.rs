use std::iter::Peekable;
use std::str::{Lines};
use std::{error::Error};
use json::object;
use json::JsonValue;

mod error;

use error::DailogError;

#[derive(Debug)]
pub struct Script {
    pause: f32,
    index: usize,
    images: Vec<(String, usize, usize)>,
    pub result: JsonValue,
}

impl Default for Script {
    fn default() -> Self {
        Self { 
            pause: 0.6,
            index: 0, 
            images: Vec::new(),
            result: object! {
                titles: [],
                script: []
            }
        }
    }
}

pub fn parse_script(input: &mut String) -> Result<Script, Box<dyn Error>> {
    let mut input = input.lines().peekable();
    
    let mut script = Script::default();

    if let Some(&"[start]") = input.peek() {
        input.next();
        parse_content_section(&mut input, &mut script)?;
    }

    for (img, index, count) in &script.images {
        // println!("{} {} {}", img, index, count);
        for i in *index..(*index + *count) {
            script.result["script"][i]["img"] = JsonValue::String(img.clone());
        }
    }

    Ok(script)
}

fn parse_content_section(input: &mut Peekable<Lines>, script: &mut Script) -> Result<(), Box<dyn Error>> {

    loop {
        match input.peek() {
            Some(&"") => {},
            Some(line) if line.starts_with("[video") => {
                let video_name = parse_string(*line)?;
                script.result["script"][script.index-1]["video"] = JsonValue::String(video_name.to_string());
                // println!("{}", video_name);
            },
            Some(line) if line.starts_with("[image") => {
                let image_name = parse_string(*line)?;
                let count = parse_image_count(*line)?;
                script.images.push((image_name.to_string(), script.index-1, count));
                // println!("{}, {}", image_name, count);
            },
            Some(line) if line.starts_with("[title") => {
                let title_name = parse_string(*line)?;
                script.result["titles"].push(make_title_object(title_name, script.index))?;
                // println!("{}", title_name);
            },
            Some(line) => {
                let Some((name, text)) = line.split_once(':') else {
                    return Err(Box::new(DailogError::ParseError("Could not match name and text delimited by ':' in script")));
                };
                script.result["script"].push(make_script_object(name, text, script.pause))?;
                script.index += 1;
            }
            None => {
                break;
            }
        }
        input.next();
    }

    Ok(())
}

fn make_script_object(name: &str, sentence: &str, pause: f32) -> JsonValue {
    object! {
        "name": name,
        "pause": pause,
        "img": "",
        "video": "",
        "text": format!("<speak>{}</speak>", sentence)
    }
}

fn make_title_object(title: &str, index: usize) -> JsonValue {
    object! {
        "title": title,
        "at_index": index
    }
}

fn parse_image_count(line: &str) -> Result<usize, Box<dyn Error>> {
    let mut line = line.split(',');
    line.next();
    Ok(line
        .collect::<String>()
        .strip_suffix(']')
        .unwrap()
        .trim()
        .parse::<usize>()?)
}

fn parse_string(line: &str) -> Result<&str, Box<dyn Error>> {
    
    let mut chars = line.chars().peekable();
    let mut start: usize = 1;

    while chars.next() != Some('"') {
        start += 1;
    }

    let mut end: usize = start;
    while chars.next() != Some('"') {
        end += 1;
    }

    Ok(&line[start..end])
}