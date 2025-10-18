//! Performance monitoring module
//! Basic performance tracking without external dependencies

use bevy::prelude::*;
use std::time::Instant;

#[derive(Resource)]
pub struct PerformanceMonitor {
    frame_count: u32,
    last_fps_time: Instant,
    fps: f32,
    frame_times: Vec<f32>,
    max_frame_times: usize,
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self {
            frame_count: 0,
            last_fps_time: Instant::now(),
            fps: 0.0,
            frame_times: Vec::new(),
            max_frame_times: 60, // Keep last 60 frames
        }
    }
}

impl PerformanceMonitor {
    pub fn update(&mut self, delta_time: f32) {
        self.frame_count += 1;
        self.frame_times.push(delta_time);
        
        // Keep only the last N frame times
        if self.frame_times.len() > self.max_frame_times {
            self.frame_times.remove(0);
        }
        
        // Calculate FPS every second
        let now = Instant::now();
        if now.duration_since(self.last_fps_time).as_secs() >= 1 {
            self.fps = self.frame_count as f32 / now.duration_since(self.last_fps_time).as_secs_f32();
            self.frame_count = 0;
            self.last_fps_time = now;
            
            // Log performance stats
            let avg_frame_time = self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
            let max_frame_time = self.frame_times.iter().fold(0.0f32, |a, &b| a.max(b));
            let min_frame_time = self.frame_times.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            
            // Use println! for immediate console output
            println!("📊 Performance: FPS={:.1}, Avg={:.2}ms, Min={:.2}ms, Max={:.2}ms", 
                  self.fps, avg_frame_time * 1000.0, min_frame_time * 1000.0, max_frame_time * 1000.0);
        }
    }
    
    pub fn get_fps(&self) -> f32 {
        self.fps
    }
    
    pub fn get_avg_frame_time(&self) -> f32 {
        if self.frame_times.is_empty() {
            0.0
        } else {
            self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32
        }
    }
}

/// System to monitor performance
pub fn monitor_performance(
    time: Res<Time>,
    mut monitor: ResMut<PerformanceMonitor>,
) {
    monitor.update(time.delta_secs());
}

/// System to log system execution times (basic profiling)
pub fn log_system_performance(
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F2) {
        tracing::info!("🔍 System performance logging toggled");
    }
}
