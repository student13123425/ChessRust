use raylib::prelude::*;

pub struct AudioPlayer<'a> {
    tracks: Vec<Music<'a>>,
    current_track_index: Option<usize>,
    is_active: bool,
    volume: f32,
}

impl<'a> AudioPlayer<'a> {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            current_track_index: None,
            is_active: false,
            volume: 1.0,
        }
    }

    pub fn new_with_file(audio: &mut RaylibAudio, thread: &'a RaylibThread, file_path: &str) -> Self {
        let mut player = Self::new();
        player.add_track(audio, thread, file_path);
        player.current_track_index = Some(0);
        player
    }

    pub fn add_track(&mut self, audio: &mut RaylibAudio, thread: &'a RaylibThread, file_path: &str) {
        match audio.load_music_stream(thread, file_path) {
            Ok(mut new_track) => {
                new_track.looping = false;
                self.tracks.push(new_track);

                if self.current_track_index.is_none() {
                    self.current_track_index = Some(0);
                }
            }
            Err(e) => eprintln!("Failed to load music stream: {}", e),
        }
    }

    pub fn play_track(&mut self, audio: &mut RaylibAudio, track_index: usize, loop_track: bool) {
        if track_index >= self.tracks.len() {
            return;
        }

        if self.is_active {
            if let Some(current_index) = self.current_track_index {
                audio.stop_music_stream(&mut self.tracks[current_index]);
            }
        }

        self.current_track_index = Some(track_index);

        if let Some(track) = self.tracks.get_mut(track_index) {
            track.looping = loop_track;
            audio.set_music_volume(track, self.volume);
            audio.play_music_stream(track);
            self.is_active = true;
        }
    }

    pub fn play(&mut self, audio: &mut RaylibAudio, loop_track: bool) {
        if let Some(index) = self.current_track_index {
            self.play_track(audio, index, loop_track);
        }
    }

    pub fn stop(&mut self, audio: &mut RaylibAudio) {
        if self.is_active {
            if let Some(index) = self.current_track_index {
                if let Some(track) = self.tracks.get_mut(index) {
                    audio.stop_music_stream(track);
                }
                self.is_active = false;
            }
        }
    }

    pub fn update(&mut self, audio: &mut RaylibAudio) {
        if self.is_active {
            if let Some(index) = self.current_track_index {
                if let Some(track) = self.tracks.get_mut(index) {
                    audio.update_music_stream(track);

                    if !track.looping && !audio.is_music_stream_playing(track) {
                        self.is_active = false;
                    }
                }
            }
        }
    }

    pub fn set_volume(&mut self, audio: &mut RaylibAudio, new_volume: f32) {
        self.volume = new_volume;
        if let Some(index) = self.current_track_index {
            if let Some(track) = self.tracks.get_mut(index) {
                audio.set_music_volume(track, new_volume);
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        !self.is_active
    }

    pub fn unload(&mut self) {
        self.tracks.clear();
        self.current_track_index = None;
        self.is_active = false;
    }
}