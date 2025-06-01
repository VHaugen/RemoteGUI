use std::io::Write;
use iced::widget::{button, column, text, Column,progress_bar};
use hound;
use std::env;
use pitch_detection::detector::mcleod::McLeodDetector;
use pitch_detection::detector::PitchDetector;



#[derive(Default)]
struct Counter {
    value:f32,
    warning_text:f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl Counter {

    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout

        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),
            progress_bar(0.0..=100.0, self.value),
            // We show the value of the counter here
            text(self.value).size(50),
            text(self.warning_text).size(50),

            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1.0;
            }
            Message::Decrement => {
                self.value -= 1.0;
            }
        }
    }
}
fn compute_snippet()
{

}



fn main() -> iced::Result {
    let fname = env::args().nth(1).expect("no file given");
    let mut reader = hound::WavReader::open(&fname).unwrap();
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    
    let size_of_samples:i32 = samples.len() as i32;
    println!("this is t he size: {size_of_samples}");
    let samples_hz:i32 = size_of_samples/88200;
    let testvec: Vec<i16> = Vec::new();

    const SAMPLE_RATE: usize = 88200;
    const SIZE: usize = 1024;
    const PADDING: usize = SIZE / 2;
    const POWER_THRESHOLD: f64 = 5.0;
    const CLARITY_THRESHOLD: f64 = 0.7;


    let dt = 1.0 / SAMPLE_RATE as f64;
    let freq = 300.0;
    let signal: Vec<f64> = (0..SIZE)
        .map(|x| (2.0 * std::f64::consts::PI * x as f64 * dt * freq).sin())
        .collect();


    let mut detector = McLeodDetector::new(SIZE, PADDING);



    let pitch = detector
        .get_pitch(&signal, SAMPLE_RATE, POWER_THRESHOLD, CLARITY_THRESHOLD)
        .unwrap();
    
        println!("Frequency: {}, Clarity: {}", pitch.frequency, pitch.clarity);


    for value in 0..(samples_hz) {
        println!("sample {value}")
    }

    iced::run("A cool counter", Counter::update, Counter::view)
}