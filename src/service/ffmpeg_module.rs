use std::io::{BufRead, BufReader};
use std::process::{Command, Output, Stdio};

static SUPPORTED_FORMATS: [&str; 74] = ["mp4", "webm", "flac", "apng", "asf", "ea","mp3","wav", "mov","a64","aac","ac3","adts","adx","afc","aiff","apm", "aptx","ast", "au","avi","avif","binka","bit","caf","dds_pipe","dfpwm","dvd","eac3","f4v","fits","flv","g722","genh","gif","h264","hevc", "ircam","ismv","ivf","latm","loas","m4v","mjpeg","moflex","m4a","mp2","mp3","mpeg","mtv","mulaw","mxf","nut","obu","oga","ogg","ogv","opus","psp","sbc","sox","spdif","spx","svs","tta","vag","vob","voc","w64","wav","webm","webp", "wtv", "wv"];

pub fn ffmpeg_convert(input: &str , output: &str ) -> Result<String, String> {
    if is_ffmpeg_installed(){
        run_ffmpeg_command(input, output);

        Ok("converted".to_string())
    }else {
        Err("ffmpeg not installed".to_string())
    }
}
pub fn ffmpeg_is_supported_format(name:&str) -> bool {
    return SUPPORTED_FORMATS.contains(&name)
}

fn is_ffmpeg_installed() -> bool{
    match Command::new("ffmpeg").arg("-version").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn run_ffmpeg_command(input: &str, output: &str){
    let total_frames = match get_video_frame_count(input) {
        Ok(frames) => {
            println!("total frames: {}", frames);
            frames
        }
        Err(_) => {
            println!("error getting frame count");
            return;
        }
    };

    let process = Command::new("ffmpeg")
        .arg("-i")
        .arg(input.to_string())
        .arg(output.to_string())
        .arg("-progress")
        .arg("-")
        .arg("-nostats")
        .arg("-y")
        .arg("-threads")
        .arg("0")
        .stderr(Stdio::piped()) // Capture stderr (ffmpeg usually writes to stderr)
        .stdout(Stdio::piped()) // Capture stdout
        .spawn()             // Spawn the process
        .expect("Failed to start ffmpeg");

    let stdout = BufReader::new(process.stdout.expect("Failed to capture ffmpeg output"));

    for line in stdout.lines() {
        match line {
            Ok(line) => {
                if line.starts_with("frame="){
                    println!("{}%", get_percentage(&*line, &total_frames));
                }
            },
            Err(e) => eprintln!("Error reading ffmpeg output: {}", e),
        }
    }
    let stderr = BufReader::new(process.stderr.expect("Failed to capture ffmpeg error"));

    for line in stderr.lines() {
        match line {
            Ok(line) => {
                if line.starts_with("error"){
                    print!("Error: {}", line);
                }
            }
            Err(e) => eprintln!("Error reading ffmpeg output: {}", e),
        }
    }
}
fn get_percentage(line: &str, total_frames: &u64)-> f64{
    let current_frame = line.split('=').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
    (current_frame as f64 / *total_frames as f64)* 100.0
}

fn get_video_frame_count(path: &str) -> Result<u64, String> {
    let output = Command::new("ffprobe")
        .args(&["-v", "error", "-select_streams", "v:0", "-count_packets", "-show_entries", "stream=nb_read_packets", "-of", "csv=p=0", path])
        .output()
        .map_err(|e| format!("Failed to execute ffmpeg command: {}", e))?;

    if !output.status.success() {
        return Err(format!("ffmpeg error: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let frame_count_str = String::from_utf8_lossy(&output.stdout);
    frame_count_str.trim().parse::<u64>()
        .map_err(|e| format!("Failed to parse frame count: {}", e))
}
