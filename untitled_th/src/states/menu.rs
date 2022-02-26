//! Using lua to write menu logic
//! Load 'script/menu/main.lua' as the main logic
//! the loaded function should be inserted to th.menu.<name>
//! the script should have function
//! * render
//! * update
//! * * return tran, LoopState

use std::convert::TryInto;

use mlua::{Lua, Table};
use mlua::prelude::LuaFunction;
use wgpu_glyph::Text;

use pth_render_lib::*;

use crate::LoopState;
use crate::render::GlobalState;
use crate::render::texture2d::{Texture2DObject, Texture2DVertexData};
use crate::states::{GameState, StateData, StateEvent, Trans};

const BUTTON_COUNT: usize = 9;
const BUTTON_NAME: [&str; BUTTON_COUNT] = ["Singleplayer", "Multiplayer", "Extra", "Profile", "Replay", "Music Room", "Option", "Cloud", "Exit"];


pub struct MainMenu {
    /// menu scripts (name, render, update)
    scripts: Vec<(String, LuaFunction<'static>, LuaFunction<'static>)>,
}

impl MainMenu {
    pub(crate) fn new(lua: &'static Lua, script: String) -> Self {
        let table: Table = lua.load_from_function("main", lua.load(&script).into_function().unwrap()).unwrap();
        Self {
            scripts: vec![("main".into(), table.get("render").unwrap(), table.get("update").unwrap())]
        }
    }
}

impl GameState for MainMenu {
    fn start(&mut self, data: &mut StateData) {
        if let Some(al) = &mut data.global_state.al {
            al.play_bgm(data.global_state.handles.bgm_map.read().unwrap()["title"].clone());
        }
    }

    fn update(&mut self, data: &mut StateData) -> (Trans, LoopState) {
        if let Some((name, _, update)) = self.scripts.last() {
            match update.call::<_, (i8, LoopState)>(()) {
                Ok((tran, loop_state)) => {
                    if tran == -1 {
                        (Trans::Exit, loop_state)
                    } else {
                        (Trans::None, loop_state)
                    }
                }
                Err(e) => {
                    log::warn!("Script failed in {} for {:?}", name, e);
                    self.scripts.pop();
                    (Trans::None, LoopState::WAIT)
                }
            }
        } else {
            return (Trans::Pop, LoopState::WAIT);
        }
        // let mut loop_state = LoopState::WAIT_ALL;
        // const EXIT_IDX: u8 = (BUTTON_COUNT - 1) as u8;
        //
        // let now = std::time::SystemTime::now();
        // let input = &data.inputs.cur_frame_game_input;
        //
        // //make sure the screen is right
        // //check enter / shoot first
        // if input.shoot > 0 || input.enter > 0 {
        //     match self.select {
        //         0 => {
        //
        //             // return (LoadState::switch_wait_load(Trans::Push(Box::new(Gaming::default())), Duration::from_secs(0)), LoopState::WAIT);
        //         }
        //         EXIT_IDX => {
        //             return (Trans::Exit, loop_state);
        //         }
        //         _ => {}
        //     }
        // }
        // if input.bomb == 1 {
        //     loop_state = LoopState::WAIT;
        //     self.select = EXIT_IDX;
        // }
        //
        // let just_change = input.up == 1 || input.down == 1;
        // if input.up == 1 || input.down == 1 || now.duration_since(self.time).unwrap().as_secs_f32() > if self.con { 1. / 6. } else { 0.5 } {
        //     match input.direction.1 {
        //         x if x > 0 => {
        //             self.time = now;
        //             self.con = !just_change;
        //             log::debug!("Select previous button");
        //             self.select = get_previous(self.select, BUTTON_COUNT as _);
        //             loop_state = LoopState::WAIT;
        //         }
        //         x if x < 0 => {
        //             self.time = now;
        //             self.con = !just_change;
        //             log::debug!("Select next button");
        //             self.select = get_next(self.select, BUTTON_COUNT as _);
        //             loop_state = LoopState::WAIT;
        //         }
        //         _ => {
        //             self.con = false;
        //         }
        //     }
        // }
        //
        // for (i, s) in self.texts.iter_mut().enumerate() {
        //     if i as u8 == self.select {
        //         s.text[0].extra.color = [1., 1., 1., 1.];
        //     } else {
        //         s.text[0].extra.color = [0.5, 0.5, 0.5, 1.];
        //     }
        // }
        // loop_state.render |= self.dirty;
        // self.dirty = false;
        // (Trans::None, loop_state)
    }


    fn render(&mut self, data: &mut StateData) -> Trans {
        // let screen = &data.render.views.get_screen().view;
        // if let Some(ref obj) = self.background {
        //     data.render.render2d.render(data.global_state, screen, std::slice::from_ref(obj));
        // }
        // {
        //     let mut encoder = data.global_state.device
        //         .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Menu Text Encoder") });
        //     let mut tran = self.texts[self.select as usize].screen_position;
        //     tran.0 += 3.0;
        //     tran.1 += 3.0;
        //     let shadow = wgpu_glyph::Section {
        //         screen_position: tran,
        //         bounds: (9961.0, 9961.0),
        //         layout: Default::default(),
        //         text: vec![Text::new(BUTTON_NAME[self.select as usize])
        //             .with_color([136.0 / 256.0, 136.0 / 256.0, 136.0 / 256.0, 1.0])
        //             .with_scale(36.0 * data.global_state.size_scale[0])],
        //     };
        //     data.render.glyph_brush.queue(shadow);
        //
        //     for s in &self.texts {
        //         data.render.glyph_brush.queue(s);
        //     }
        //
        //     if let Err(e) = data.render.glyph_brush
        //         .draw_queued(&data.global_state.device, &mut data.render.staging_belt, &mut encoder, screen,
        //                      data.global_state.surface_cfg.width,
        //                      data.global_state.surface_cfg.height) {
        //         log::warn!("Render menu text failed for {}", e);
        //     }
        //     data.render.staging_belt.finish();
        //     data.global_state.queue.submit(Some(encoder.finish()));
        // }
        Trans::None
    }

    fn on_event(&mut self, e: &StateEvent) {
        // match e {
        //     StateEvent::Resize { width, height } => {
        //         let width = *width as _;
        //         let height = *height as _;
        //         if let Some(background) = &mut self.background {
        //             background.vertex = (0..4).map(|x| {
        //                 Texture2DVertexData {
        //                     pos: match x {
        //                         0 => [0.0, height],
        //                         1 => [width, height],
        //                         2 => [0.0, 0.0],
        //                         3 => [width, 0.0],
        //                         _ => unreachable!()
        //                     },
        //                     coord: match x {
        //                         0 => [0.0, 0.0],
        //                         1 => [1.0, 0.0],
        //                         2 => [0.0, 1.0],
        //                         3 => [1.0, 1.0],
        //                         _ => unreachable!()
        //                     },
        //                 }
        //             }).collect::<Vec<_>>().try_into().unwrap();
        //             self.texts.clear();
        //             for (i, text) in BUTTON_NAME.iter().enumerate() {
        //                 let color = if i == 0 { 1.0 } else { 0.5 };
        //                 self.texts.push(wgpu_glyph::Section {
        //                     screen_position: (60.0 * width as f32 / 1600.0, (380.0 + i as f32 * 55.0) * height as f32 / 900.0),
        //                     bounds: (9961.0, 9961.0),
        //                     layout: Default::default(),
        //                     text: vec![Text::new(text).with_color([color, color, color, 1.0])
        //                         .with_scale(36.0 * width as f32 / 1600.0)],
        //                 })
        //             }
        //         }
        //     }
        //     _ => {}
        // }
    }
}

#[inline]
pub fn get_previous(cur_idx: u8, max_len: u8) -> u8 {
    if cur_idx == 0 {
        max_len - 1
    } else {
        cur_idx - 1
    }
}

#[inline]
pub fn get_next(cur_idx: u8, max_len: u8) -> u8 {
    let cur_idx = cur_idx + 1;
    if cur_idx == max_len {
        0
    } else {
        cur_idx
    }
}