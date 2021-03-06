use std::time::{Duration, Instant};

use futures::future::RemoteHandle;
use pollster::FutureExt;

use crate::LoopState;
// use game_api as root;
use crate::resource::{CounterProgress, Progress};
use crate::states::{GameState, StateData, Trans};
use crate::states::menu::MainMenu;

pub struct Loading {
    progress: CounterProgress,
    start: Instant,
    menu_script: Option<RemoteHandle<String>>,
    fst: bool,
}

impl Default for Loading {
    fn default() -> Self {
        Self {
            progress: Default::default(),
            start: Instant::now(),
            menu_script: None,
            fst: true,
        }
    }
}

impl GameState for Loading {
    fn start(&mut self, data: &mut StateData) {
        log::info!("loading state start");
        self.start = Instant::now();
        let graphics_state = &data.global_state;
        let handles = &graphics_state.handles;
        let pools = &data.pools;
        handles.load_texture_static("bullet", "bullet.png", graphics_state, self.progress.create_tracker());
        handles.load_texture_static("circle_red", "circle_red.png", graphics_state, self.progress.create_tracker());
        handles.load_texture_static("circle_blue", "circle_blue.png", graphics_state, self.progress.create_tracker());
        handles.load_texture_static("circle_green", "circle_green.png", graphics_state, self.progress.create_tracker());
        handles.load_texture_static("circle_yellow", "circle_yellow.png", graphics_state, self.progress.create_tracker());
        handles.load_texture_static("circle_purple", "circle_purple.png", graphics_state, self.progress.create_tracker());
        handles.load_texture_static("zzzz", "zzzz.png", graphics_state, self.progress.create_tracker());
        handles.load_texture_static("mainbg", "mainbg.png", graphics_state, self.progress.create_tracker());
        handles.load_texture_static("暗夜", "暗夜.png", graphics_state, self.progress.create_tracker());
        handles.load_texture_static("sheepBullet", "sheepBullet.png", graphics_state, self.progress.create_tracker());
        handles.load_texture_static("sheep", "sheep.png", graphics_state, self.progress.create_tracker());
        self.menu_script = Some(handles.read_all_string("script/menu/main.lua".into(), pools, self.progress.create_tracker()));
        if let Some(al) = &data.global_state.al {
            handles.load_bgm_static("title", "title.mp3", al.ctx.clone(), &data.pools.io_pool, self.progress.create_tracker());
        }
    }

    fn update(&mut self, s: &mut StateData) -> (Trans, LoopState) {
        if self.fst {
            self.fst = false;
            (Trans::None, LoopState::wait_until(Duration::from_millis(250), true))
        } else if self.progress.num_loading() == 0 {
            let script = self.menu_script.take().unwrap().block_on();
            log::info!("Loaded {} resources in {}ms", self.progress.num_finished(), std::time::Instant::now().duration_since(self.start).as_millis());
            (Trans::Push(Box::new(MainMenu::new(s.lua, script))), LoopState::WAIT)
        } else {
            (Trans::None, LoopState::wait_until(Duration::from_millis(50), false))
        }
    }

    fn shadow_tick(&mut self, _data: &StateData) {
        //todo: reload
    }
}