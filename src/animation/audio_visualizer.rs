use std::sync::{Arc, Mutex};
use std::thread;
use crate::{Animation, Strip};
use std::sync::mpsc::channel;
use std::thread::JoinHandle;
use spectrum_analyzer::{Frequency, FrequencyLimit, FrequencyValue, samples_fft_to_spectrum};
use spectrum_analyzer::scaling::divide_by_N;
use spectrum_analyzer::windows::hann_window;
use speedy2d::color::Color;

pub struct AudioVisualizer{
    buffer: Arc<Mutex<Vec<(Frequency, FrequencyValue)>>>,
    thread: Option<JoinHandle<()>>,
    last_sums: Vec<f32>,
    last_sums_count: usize,
    stop_stream: Arc<Mutex<bool>>,
    last_factor: f32,
}

impl Animation for AudioVisualizer{
    fn initialize(&mut self, strip: Arc<Mutex<Strip>>) {
        println!("Initializing AudioVisualizer");
        {
            strip.lock().unwrap().reset();
        }
        self.start();
    }

    fn update(&mut self, strip: Arc<Mutex<Strip>>) {
        let color = Color::GREEN;
        let scale_factor = 2.0;
        let local_buffer;
        {
            local_buffer = self.buffer.lock().unwrap().clone();
        }
        let mut sum = 0.0;
        for (_, value) in local_buffer.iter() {
            sum += value.val();
        }
        self.last_sums.push(sum);
        while self.last_sums.len() > self.last_sums_count {
            self.last_sums.remove(0);
        }
        let factor = scale_factor * sum / self.last_sums.iter().sum::<f32>();
        let decrease_factor = 0.95;
        let actual_factor = f32::min(1.0, f32::max(factor, self.last_factor*decrease_factor)- (1.0 / self.last_sums_count as f32) / 2.0);
        {
            let mut strip = strip.lock().unwrap();
            strip.reset();
            for i in 0..(strip.get_pixel_length() as f32*actual_factor).ceil() as usize {
                strip.set_pixel(i, color);
            }
        }
    }

    fn terminate(&mut self) {
        self.stop();
    }
}

impl AudioVisualizer{
    pub fn new() -> AudioVisualizer{
        AudioVisualizer{
            buffer: Arc::new(Mutex::new(Vec::new())),
            thread: None,
            last_sums: vec![0.0; 5],
            last_sums_count: 5,
            stop_stream: Arc::new(Mutex::new(false)),
            last_factor: 0.0,
        }
    }

    fn stop(&mut self){
        {
            let mut stop_stream = self.stop_stream.lock().unwrap();
            *stop_stream = true;
        }
        if let Some(thread) = self.thread.take(){
            thread.join().unwrap();
        }
        self.thread = None;
    }

    fn start(&mut self){
        if let Some(_) = self.thread{
            self.stop();
        }
        {
            let mut stop_stream = self.stop_stream.lock().unwrap();
            *stop_stream = false;
        }
        let stopper_copy = self.stop_stream.clone();
        let buffer_copy = self.buffer.clone();
        self.thread = Some(thread::spawn(move || {
            // region Audio Initialization
            let pa = portaudio::PortAudio::new().expect("Unable to init PortAudio");
            // Collect information about the default microphone
            let mic_index = pa.default_input_device().expect("Unable to get default device");
            let mic = pa.device_info(mic_index).expect("unable to get mic info");

            // Set parameters for the stream settings.
            // We pass which mic should be used, how many channels are used,
            // whether all the values of all the channels should be passed in a
            // single audiobuffer and the latency that should be considered
            let input_params = portaudio::StreamParameters::<f32>::new( mic_index, 1, true, mic.default_low_input_latency);

            // Settings for an inputstream.
            // Here we pass the stream parameters we set before,
            // the sample rate of the mic and the amount values we want to receive
            let input_settings = portaudio::InputStreamSettings::new(input_params, mic.default_sample_rate, 256);

            // Creating a channel so we can receive audio values asynchronously
            let (sender, receiver) = channel();

            // A callback function that should be as short as possible so we send all the info to a different thread
            let callback = move |portaudio::InputStreamCallbackArgs {buffer, .. }| {
                match sender.send(buffer) {
                    Ok(_) => portaudio::Continue,
                    Err(_) => portaudio::Complete
                }
            };

            // Creating & starting the input stream with our settings & callback
            let mut stream = pa.open_non_blocking_stream(input_settings, callback).expect("Unable to create stream");
            stream.start().expect("Unable to start stream");
            // endregion
            let mut mybuffer: Vec<f32> = vec![];

            'outer: while stream.is_active().unwrap(){
                while let Ok(buffer) = receiver.try_recv(){
                    let stop: bool;
                    {
                        stop = *stopper_copy.lock().unwrap();
                    }
                    if stop{
                        break 'outer;
                    }
                    mybuffer.extend_from_slice(buffer);
                    if mybuffer.len() > 4096{
                        mybuffer.truncate(4096);
                        // apply hann window for smoothing; length must be a power of 2 for the FFT
                        // 2048 is a good starting point with 44100 kHz
                        let hann_window = hann_window(mybuffer.as_slice());
                        // calc spectrum
                        let spectrum_hann_window = samples_fft_to_spectrum(
                            // (windowed) samples
                            &hann_window,
                            // sampling rate
                            mic.default_sample_rate.floor() as u32,
                            // optional frequency limit: e.g. only interested in frequencies 50 <= f <= 150?
                            FrequencyLimit::Range(20.0, 10000.0),
                            // optional scale
                            Some(&divide_by_N),
                        )
                            .unwrap();
                        {
                            let mut lock = buffer_copy.lock().unwrap();
                            *lock = spectrum_hann_window.data().to_vec();
                        }
                        mybuffer.clear();
                    }
                }
            }
            stream.stop().expect("Unable to stop stream");
        }));

    }
}
