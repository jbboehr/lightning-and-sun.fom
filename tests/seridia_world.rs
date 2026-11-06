use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path, process::Command};
fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn rgba(n: u32) -> [u8; 4] {
    [(n >> 16) as u8, (n >> 8) as u8, n as u8, 255]
}
fn inventory() -> Vec<(&'static str, u32, &'static str)> {
    vec![
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_action_east.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_action_north.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_action_south.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_blink_east.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_blink_south.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_drink_east.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_drink_north.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_drink_south.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_eat_east.png",
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_eat_north.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_eat_south.png",
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_idle_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_idle_north.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_idle_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_kiss_east.png",
            4,
            "[0.15,0.15,0.8,0.15]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_sit_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_sit_north.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_sit_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_sleep_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_walk_east.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_walk_north.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_walk_south.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_meditate_end_south.png",
            3,
            "[0.125,0.125,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_meditate_loop_south.png",
            8,
            "[0.125,0.2,0.125,0.2,0.125,0.2,0.125,0.2]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_meditate_start_south.png",
            3,
            "[0.1,0.125,0.125]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_read_sit_end_south.png",
            3,
            "0.1",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_read_sit_loop_south.png",
            4,
            "[3.0,0.1,3.0,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_read_sit_start_south.png",
            3,
            "0.1",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_spell_cast_end_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_spell_cast_loop_south.png",
            4,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_spell_cast_start_south.png",
            2,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_write_end_south.png",
            2,
            "[0.125,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_write_loop_south.png",
            4,
            "[0.1,0.125,0.1,0.3]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_write_sit_end_south.png",
            2,
            "[0.125,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_write_sit_loop_south.png",
            4,
            "[0.1,0.125,0.1,0.3]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_write_sit_start_south.png",
            2,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_write_start_south.png",
            2,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_action_east.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_action_north.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_action_south.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_bath_swim_east.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_bath_swim_south.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_blink_east.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_blink_south.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_idle_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_idle_north.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_idle_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_kiss_east.png",
            4,
            "[0.15,0.15,0.8,0.15]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_sit_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_sit_north.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_sit_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_walk_east.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_walk_north.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_walk_south.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_blink_east.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_blink_south.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_idle_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_idle_north.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_idle_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_walk_east.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_walk_north.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_walk_south.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_meditate_end_south.png",
            3,
            "[0.125,0.125,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_meditate_loop_south.png",
            8,
            "[0.125,0.2,0.125,0.2,0.125,0.2,0.125,0.2]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_meditate_start_south.png",
            3,
            "[0.1,0.125,0.125]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_read_sit_end_south.png",
            3,
            "0.1",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_read_sit_loop_south.png",
            4,
            "[3.0,0.1,3.0,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_read_sit_start_south.png",
            3,
            "0.1",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_spell_cast_end_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_spell_cast_loop_south.png",
            4,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_spell_cast_start_south.png",
            2,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_write_end_south.png",
            2,
            "[0.125,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_write_loop_south.png",
            4,
            "[0.1,0.125,0.1,0.3]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_write_sit_end_south.png",
            2,
            "[0.125,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_write_sit_loop_south.png",
            4,
            "[0.1,0.125,0.1,0.3]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_write_sit_start_south.png",
            2,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_write_start_south.png",
            2,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_action_east.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_action_north.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_action_south.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_blink_east.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_blink_south.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_drink_east.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_drink_north.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_drink_south.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_eat_east.png",
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_eat_north.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_eat_south.png",
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_idle_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_idle_north.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_idle_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_kiss_east.png",
            4,
            "[0.15,0.15,0.8,0.15]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_sit_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_sit_north.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_sit_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_sleep_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_walk_east.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_walk_north.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_walk_south.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_meditate_end_south.png",
            3,
            "[0.125,0.125,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_meditate_loop_south.png",
            8,
            "[0.125,0.2,0.125,0.2,0.125,0.2,0.125,0.2]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_meditate_start_south.png",
            3,
            "[0.1,0.125,0.125]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_read_sit_end_south.png",
            3,
            "0.1",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_read_sit_loop_south.png",
            4,
            "[3.0,0.1,3.0,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_read_sit_start_south.png",
            3,
            "0.1",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_spell_cast_end_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_spell_cast_loop_south.png",
            4,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_spell_cast_start_south.png",
            2,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_write_end_south.png",
            2,
            "[0.125,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_write_loop_south.png",
            4,
            "[0.1,0.125,0.1,0.3]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_write_sit_end_south.png",
            2,
            "[0.125,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_write_sit_loop_south.png",
            4,
            "[0.1,0.125,0.1,0.3]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_write_sit_start_south.png",
            2,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_write_start_south.png",
            2,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_action_east.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_action_north.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_action_south.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_blink_east.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_blink_south.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_drink_east.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_drink_north.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_drink_south.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_eat_east.png",
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_eat_north.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_eat_south.png",
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_idle_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_idle_north.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_idle_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_kiss_east.png",
            4,
            "[0.15,0.15,0.8,0.15]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_sit_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_sit_north.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_sit_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_sleep_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_walk_east.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_walk_north.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_walk_south.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_action_east.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_action_north.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_action_south.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_blink_east.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_blink_south.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_idle_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_idle_north.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_idle_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_kiss_east.png",
            4,
            "[0.15,0.15,0.8,0.15]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_sit_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_sit_north.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_sit_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_walk_east.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_walk_north.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_walk_south.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_meditate_end_south.png",
            3,
            "[0.125,0.125,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_meditate_loop_south.png",
            8,
            "[0.125,0.2,0.125,0.2,0.125,0.2,0.125,0.2]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_meditate_start_south.png",
            3,
            "[0.1,0.125,0.125]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_read_sit_end_south.png",
            3,
            "0.1",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_read_sit_loop_south.png",
            4,
            "[3.0,0.1,3.0,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_read_sit_start_south.png",
            3,
            "0.1",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_spell_cast_end_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_spell_cast_loop_south.png",
            4,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_spell_cast_start_south.png",
            2,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_write_end_south.png",
            2,
            "[0.125,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_write_loop_south.png",
            4,
            "[0.1,0.125,0.1,0.3]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_write_sit_end_south.png",
            2,
            "[0.125,0.1]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_write_sit_loop_south.png",
            4,
            "[0.1,0.125,0.1,0.3]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_write_sit_start_south.png",
            2,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_write_start_south.png",
            2,
            "0.125",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_action_east.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_action_north.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_action_south.png",
            7,
            "[0.1,0.25,0.25,0.25,0.25,0.1,0.4]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_blink_east.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_blink_south.png",
            3,
            "[0.075,0.125,0.075]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_drink_east.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_drink_north.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_drink_south.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_eat_east.png",
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_eat_north.png",
            3,
            "1.0",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_eat_south.png",
            5,
            "[0.125,0.15,0.175,0.125,0.6]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_idle_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_idle_north.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_idle_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_kiss_east.png",
            4,
            "[0.15,0.15,0.8,0.15]",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_sit_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_sit_north.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_sit_south.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_sleep_east.png",
            1,
            "",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_walk_east.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_walk_north.png",
            4,
            "0.15",
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_walk_south.png",
            4,
            "0.15",
        ),
    ]
}
const WORLD_ROLES: &[(u32, usize, [i32; 3])] = &[
    (0x594240, 3, [0, 0, 0]),
    (0x624A48, 3, [0, 0, 0]),
    (0x64484E, 3, [11, 6, 14]),
    (0x724F62, 3, [25, 13, 34]),
    (0x7C556F, 3, [35, 19, 47]),
    (0x7F5673, 3, [38, 20, 51]),
    (0x835979, 3, [42, 23, 57]),
    (0x8A716A, 2, [0, 0, 0]),
    (0x8E746D, 2, [0, 0, 0]),
    (0x917375, 2, [7, 2, 11]),
    (0x947678, 2, [6, 2, 11]),
    (0x95726A, 16, [0, 0, 0]),
    (0x9A7684, 2, [16, 5, 26]),
    (0x9D7887, 2, [15, 4, 26]),
    (0xA1788F, 2, [23, 7, 37]),
    (0xA27992, 2, [24, 8, 40]),
    (0xA47A91, 2, [22, 6, 36]),
    (0xA57A96, 2, [27, 9, 44]),
    (0xA57B94, 2, [23, 7, 39]),
    (0xA69084, 1, [0, 0, 0]),
    (0xA87C99, 2, [26, 8, 44]),
    (0xAB908D, 1, [5, 0, 9]),
    (0xB1909A, 1, [11, 0, 22]),
    (0xB690A2, 1, [16, 0, 30]),
    (0xB790A5, 1, [17, 0, 33]),
    (0xB990A9, 1, [19, 0, 37]),
    (0xC1AFA5, 0, [0, 0, 0]),
    (0xC3AA9B, 15, [0, 0, 0]),
    (0xC3ADAC, 0, [2, -2, 7]),
    (0xC7AAB5, 0, [6, -5, 16]),
    (0xCAA7BB, 0, [9, -8, 22]),
    (0xCBA7BD, 0, [10, -8, 24]),
    (0xCCA6C0, 0, [11, -9, 27]),
    (0xD37A57, 9, [0, 0, 0]),
    (0xF0B988, 8, [0, 0, 0]),
    (0xFCD9B3, 7, [0, 0, 0]),
];
fn target(color: u32, choice: &Value) -> [u8; 4] {
    let (_, role, offset) = WORLD_ROLES.iter().find(|(c, _, _)| *c == color).unwrap();
    let s = choice["colors"][*role].as_str().unwrap();
    let t = rgba(u32::from_str_radix(&s[1..], 16).unwrap());
    [
        (i32::from(t[0]) + offset[0]).clamp(0, 255) as u8,
        (i32::from(t[1]) + offset[1]).clamp(0, 255) as u8,
        (i32::from(t[2]) + offset[2]).clamp(0, 255) as u8,
        255,
    ]
}
#[test]
fn world_profile_preserves_all_portraits_palette_roles_and_spectral_exclusion() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let old = read_json(root.join("palettes/profiles/seridia-portraits.json"));
    let new = read_json(root.join("palettes/profiles/seridia-world-trial.json"));
    assert_eq!(
        &new["regions"].as_array().unwrap()[..164],
        old["regions"].as_array().unwrap()
    );
    assert_eq!(
        &new["source_colors"].as_array().unwrap()[..20],
        old["source_colors"].as_array().unwrap()
    );
    assert_eq!(
        &new["color_groups"].as_array().unwrap()[..20],
        old["color_groups"].as_array().unwrap()
    );
    assert_eq!(new["regions"].as_array().unwrap().len(), 352);
    assert_eq!(new["source_colors"].as_array().unwrap().len(), 50);
    let actual: BTreeSet<_> = new["regions"].as_array().unwrap()[164..]
        .iter()
        .map(|r| r["asset"].as_str().unwrap())
        .collect();
    let expected: BTreeSet<_> = inventory().into_iter().map(|(a, _, _)| a).collect();
    assert_eq!(actual, expected);
    assert_eq!(actual.len(), 188);
    assert!(actual.iter().all(|a| !a.contains("/Priestess/")));
    let before = read_json(root.join("palettes/sets/seridia-portraits-trial.json"));
    let after = read_json(root.join("palettes/sets/seridia-world-trial.json"));
    assert_eq!(before["presets"].as_array().unwrap().len(), 4);
    assert_eq!(after["presets"].as_array().unwrap().len(), 4);
    for (old, new) in before["presets"]
        .as_array()
        .unwrap()
        .iter()
        .zip(after["presets"].as_array().unwrap())
    {
        assert_eq!(old["id"], new["id"]);
        assert_eq!(old["label"], new["label"]);
        assert_eq!(
            &new["colors"].as_array().unwrap()[..20],
            old["colors"].as_array().unwrap()
        );
        for (c, mapping) in new["colors"]
            .as_array()
            .unwrap()
            .iter()
            .zip(after_source(root))
        {
            let n = u32::from_str_radix(&mapping[1..], 16).unwrap();
            if WORLD_ROLES.iter().any(|(s, _, _)| *s == n) {
                assert_eq!(
                    rgba(u32::from_str_radix(&c.as_str().unwrap()[1..], 16).unwrap()),
                    target(n, old)
                );
            }
        }
    }
}
fn after_source(root: &Path) -> Vec<String> {
    read_json(root.join("palettes/profiles/seridia-world-trial.json"))["source_colors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect()
}
#[test]
fn all_seven_form_packages_preserve_native_frames_timing_offsets_and_atlas() {
    let temp = tempfile::tempdir().unwrap();
    let original = temp.path().join("original");
    let modified = temp.path().join("modified");
    fs::create_dir(&original).unwrap();
    fs::create_dir(&modified).unwrap();
    for (name, frames, duration) in inventory() {
        let stem = Path::new(name)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let timing = if frames == 1 {
            String::new()
        } else {
            format!("frame_len={frames}\nduration={duration}\n")
        };
        let metadata = format!(
            "[meta_properties]\nid='synthetic'\nasset_kind='Animation'\n[asset_properties]\nframe_size=[80,80]\n{timing}atlas='Default'\n[asset_properties.offset]\nhorizontal=40.0\nvertical=54.0\n"
        );
        for (directory, color) in [
            (&original, [10, 20, 30, 255]),
            (&modified, [40, 50, 60, 255]),
        ] {
            RgbaImage::from_pixel(frames * 80, 80, Rgba(color))
                .save(directory.join(format!("{stem}.png")))
                .unwrap();
            fs::write(directory.join(format!("{stem}.meta.toml")), &metadata).unwrap();
        }
    }
    let output = temp.path().join("package");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["package-toggle", "--original"])
        .arg(&original)
        .arg("--modified")
        .arg(&modified)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let script = fs::read_to_string(output.join("gml/palette_assets.gml")).unwrap();
    let table: Value = serde_json::from_str(
        script
            .split_once("return ")
            .unwrap()
            .1
            .split_once("; }")
            .unwrap()
            .0,
    )
    .unwrap();
    assert_eq!(table.as_array().unwrap().len(), 1);
    assert_eq!(table[0][0], "seridia");
    assert_eq!(table[0][4].as_array().unwrap().len(), 188);
    let mut frames = 0;
    for row in table[0][4].as_array().unwrap() {
        let stem = row[0].as_str().unwrap();
        assert!(!stem.contains("west"));
        let target = row[1].as_str().unwrap();
        let before: toml::Value = toml::from_str(
            &fs::read_to_string(original.join(format!("{stem}.meta.toml"))).unwrap(),
        )
        .unwrap();
        let after: toml::Value = toml::from_str(
            &fs::read_to_string(
                output.join(format!("animations/LightningAndSun/{target}.meta.toml")),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            before["asset_properties"], after["asset_properties"],
            "{stem}"
        );
        frames += before["asset_properties"]
            .get("frame_len")
            .and_then(toml::Value::as_integer)
            .unwrap_or(1);
        assert_eq!(
            fs::read(modified.join(format!("{stem}.png"))).unwrap(),
            fs::read(output.join(format!("animations/LightningAndSun/{target}.png"))).unwrap()
        );
    }
    assert_eq!(frames, 603);
}

#[test]
#[ignore = "requires Seridia complete source corpus; independent world materials and prior portrait output"]
fn every_world_pixel_matches_materials_and_retains_all_portraits() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original = root.join("extracted/test-corpus/seridia");
    let set = std::env::var_os("FOM_SERIDIA_WORLD_PRESETS")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.join("palettes/sets/seridia-world-trial.json"));
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("bundle");
    let run = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["build-presets", "--original"])
        .arg(&original)
        .arg("--presets")
        .arg(set)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let original_set = read_json(root.join("palettes/sets/seridia-portraits-trial.json"));
    let cases: &[(&str, &[usize], &[u32])] = &[
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_action_east.png",
            &[59, 70, 67, 70, 67, 59, 72],
            &[
                15802, 15804, 15805, 15807, 15882, 15884, 15885, 15887, 15962, 15964, 15965, 15967,
                16042, 16044, 16045, 16047, 16199, 16201, 16202, 16204, 16280, 16282, 16283, 16285,
                16359, 16360, 16368, 16439, 16440, 16448, 16519, 16520, 16528, 16599, 16600, 16608,
                16680, 16682, 16683, 16685, 16756, 16757, 16765, 16837, 16838, 16846, 16919, 16920,
                16928, 16999, 17000, 17008, 17079, 17080, 17088, 17159, 17160, 17168, 17237, 17238,
                17246, 17316, 17317, 17325, 17397, 17398, 17406, 17797, 17798, 17806, 19168, 19248,
                19328, 19408, 20847, 20927, 21007, 21087, 23721, 23881, 24190, 24270, 24350, 24430,
                24669, 25069, 25148, 28603, 29003, 29162, 29163, 29562, 29563,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_action_north.png",
            &[14, 18, 18, 18, 18, 14, 14],
            &[
                20208, 20608, 20688, 20849, 20929, 21009, 21089, 21890, 22290, 22370, 22530, 22610,
                22690, 22770, 24686, 24688, 24766, 24768, 24846, 24848, 24926, 24928, 25006, 25008,
                25086, 25088, 25166, 25168, 25243, 25244, 25246, 25320, 25321, 25322, 25323, 25324,
                25326, 25400, 25401, 25402, 25403, 25404, 25406, 25480, 25481, 25482, 25483, 25484,
                25486, 25560, 25561, 25562, 25563, 25564, 25566, 25643, 25644, 25646, 25723, 25724,
                25726, 25800, 25801, 25802, 25803, 25804, 25805, 25806, 25881, 25882, 25883, 25884,
                25885, 25886, 25961, 25962, 25963, 25964, 25965, 25966, 26041, 26042, 26043, 26044,
                26045, 26046, 26121, 26122, 26123, 26124, 26125, 26126, 26200, 26201, 26202, 26203,
                26204, 26205, 26206, 26280, 26281, 26282, 26283, 26284, 26285, 26286, 26361, 26362,
                26363, 26364, 26761, 26762, 26763, 26764, 26841, 26842, 26843, 26844,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_action_south.png",
            &[81, 85, 81, 85, 81, 81, 89],
            &[
                15797, 15802, 15877, 15882, 15957, 15962, 16037, 16042, 16197, 16202, 16277, 16282,
                16355, 16364, 16435, 16444, 16515, 16524, 16595, 16604, 16677, 16682, 16755, 16764,
                16835, 16844, 16915, 16916, 16923, 16924, 16995, 16996, 17003, 17004, 17075, 17076,
                17083, 17084, 17155, 17156, 17163, 17164, 17235, 17244, 17315, 17316, 17323, 17324,
                17395, 17396, 17403, 17404, 17795, 17796, 17803, 17804, 20270, 20350, 20430, 20510,
                21948, 22028, 22108, 22188, 24204, 24205, 24284, 24285, 24364, 24365, 24444, 24445,
                24685, 24686, 24749, 24829, 24909, 24989, 25085, 25086, 25149, 25229, 25629,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_blink_east.png",
            &[72, 76, 72],
            &[
                6759, 6761, 6762, 6764, 6839, 6841, 6842, 6844, 6919, 6921, 6922, 6924, 6996, 6997,
                7005, 7076, 7077, 7085, 7156, 7157, 7165, 7236, 7237, 7245, 7316, 7317, 7325, 7396,
                7397, 7405, 10588, 10668, 10748,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_blink_south.png",
            &[89, 93, 89],
            &[
                6757, 6762, 6837, 6842, 6917, 6922, 6995, 7004, 7075, 7084, 7155, 7164, 7235, 7236,
                7243, 7244, 7315, 7316, 7323, 7324, 7395, 7396, 7403, 7404, 10589, 10669, 10749,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_drink_east.png",
            &[44, 45, 44],
            &[
                7479, 7481, 7482, 7484, 7561, 7639, 7641, 7642, 7644, 7716, 7717, 7725, 7796, 7802,
                7876, 7877, 7885, 7956, 7957, 7965, 8034, 8042, 8116, 8117, 8125, 8274, 8523, 9229,
                11319, 11320, 11399, 11400, 11479, 11480, 11561, 11641, 11721, 11801, 11802, 11881,
                11882, 11961, 11962, 12041, 12042, 12121, 12122, 12201, 12202, 12281, 12361, 12441,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_drink_north.png",
            &[0, 0, 0],
            &[
                8928, 9088, 9489, 9650, 9810, 10210, 10846, 10848, 11006, 11008, 11083, 11084,
                11086, 11166, 11243, 11244, 11246, 11320, 11321, 11322, 11323, 11324, 11325, 11326,
                11400, 11401, 11402, 11403, 11404, 11406, 11408, 11480, 11481, 11482, 11483, 11484,
                11485, 11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_drink_south.png",
            &[59, 57, 59],
            &[
                7477, 7482, 7637, 7642, 7715, 7724, 7797, 7802, 7875, 7884, 7955, 7956, 7963, 7964,
                8035, 8044, 8115, 8116, 8123, 8124, 8275, 8276, 8283, 8284, 8756, 8763, 9390, 9550,
                9709, 11557, 11562, 11637, 11642, 11717, 11722, 11797, 11802, 11877, 11882, 11957,
                11962, 12037, 12038, 12041, 12042, 12117, 12118, 12121, 12122, 12197, 12198, 12201,
                12202, 12277, 12278, 12281, 12282, 12357, 12358, 12361, 12362, 12437, 12438, 12441,
                12442, 12517, 12522, 12597, 12602, 12677, 12682,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_eat_east.png",
            &[40, 37, 31, 41, 45],
            &[
                12199, 12201, 12202, 12204, 12439, 12441, 12442, 12444, 12520, 12522, 12523, 12525,
                12596, 12597, 12605, 12759, 12761, 12762, 12764, 12836, 12837, 12845, 12917, 12918,
                12926, 12996, 12997, 13005, 13079, 13081, 13082, 13084, 13156, 13157, 13165, 13236,
                13237, 13245, 13317, 13318, 13326, 13476, 13477, 13485, 13556, 13557, 13565, 13804,
                13876, 13877, 13885, 14125, 14684, 15469, 17721, 17800, 17801, 18839, 18840, 18919,
                18920, 18999, 19000, 19079, 19080, 19159, 19160, 19241, 19321, 19401, 19481, 19561,
                19641, 19642, 19721, 19722, 19801, 19802, 19881, 19882, 19961, 19962, 20041, 20042,
                20121, 20122, 20201, 20202, 20281, 20282, 20361, 20362, 20441, 20521, 20601, 20681,
                20761,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_eat_north.png",
            &[0, 0, 0],
            &[
                8928, 9088, 9489, 9650, 9810, 10450, 10846, 10848, 11006, 11008, 11083, 11084,
                11086, 11166, 11243, 11244, 11246, 11320, 11321, 11322, 11323, 11324, 11325, 11326,
                11400, 11401, 11402, 11403, 11404, 11406, 11408, 11480, 11481, 11482, 11483, 11484,
                11485, 11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_eat_south.png",
            &[59, 55, 37, 60, 60],
            &[
                12199, 12200, 12437, 12442, 12597, 12602, 12757, 12762, 12835, 12844, 12917, 12922,
                12995, 13004, 13077, 13082, 13155, 13164, 13235, 13236, 13243, 13244, 13315, 13324,
                13395, 13396, 13403, 13404, 13475, 13484, 13555, 13556, 13563, 13564, 13715, 13716,
                13723, 13724, 13875, 13876, 13883, 13884, 14516, 14523, 14676, 14683, 15390, 15630,
                15870, 15950, 19242, 19322, 19397, 19402, 19477, 19482, 19557, 19562, 19637, 19642,
                19722, 19797, 19802, 19877, 19882, 19957, 19962, 20037, 20038, 20041, 20042, 20121,
                20122, 20197, 20198, 20201, 20202, 20277, 20278, 20281, 20282, 20357, 20358, 20361,
                20362, 20437, 20438, 20441, 20442, 20517, 20518, 20521, 20522, 20597, 20598, 20601,
                20602, 20677, 20678, 20681, 20682, 20757, 20758, 20761, 20762, 20837, 20842, 20917,
                20922, 20997, 21002, 21077, 21082, 21157, 21162,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_idle_east.png",
            &[72],
            &[
                2279, 2281, 2282, 2284, 2356, 2357, 2365, 2436, 2437, 2445, 3548,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_idle_north.png",
            &[14],
            &[
                2928, 3170, 3566, 3568, 3643, 3644, 3646, 3720, 3721, 3722, 3723, 3724, 3725, 3726,
                3801, 3802, 3803, 3804,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_idle_south.png",
            &[89],
            &[2277, 2282, 2355, 2364, 2435, 2436, 2443, 2444, 3549],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_kiss_east.png",
            &[66, 57, 77, 57],
            &[
                9162, 9164, 9165, 9167, 9318, 9323, 9400, 9402, 9403, 9405, 9479, 9480, 9488, 9560,
                9562, 9563, 9565, 9635, 9636, 9644, 9717, 9718, 9726, 9799, 9800, 9808, 9877, 9878,
                9886, 9955, 9956, 9964, 10037, 10038, 10046, 10197, 10198, 10206, 11088, 11709,
                11949, 12109, 12908, 13068, 14428, 14509, 14669, 14919, 15799, 15959, 16362, 16438,
                16598, 16681, 16682, 16757, 16758, 16761, 16762, 16917, 16918, 16921, 16922, 17077,
                17237,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_sit_east.png",
            &[51],
            &[
                2519, 2521, 2522, 2524, 2596, 2597, 2605, 2676, 2677, 2685, 3799, 3800, 3881, 3961,
                3962, 4041, 4042, 4121,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_sit_north.png",
            &[0],
            &[
                3008, 3250, 3646, 3648, 3723, 3724, 3726, 3800, 3801, 3802, 3803, 3804, 3805, 3806,
                3881, 3882, 3883, 3884,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_sit_south.png",
            &[60],
            &[
                2517, 2522, 2595, 2604, 2675, 2676, 2683, 2684, 3150, 3877, 3882, 3957, 3962, 4037,
                4038, 4041, 4042, 4117, 4118, 4121, 4122, 4197, 4202,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_sleep_east.png",
            &[50],
            &[
                2359, 2361, 2362, 2364, 2436, 2437, 2445, 2516, 2517, 2525, 3958, 4038, 4118, 4119,
                4122, 4198, 4199, 4202, 4278,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_walk_east.png",
            &[72, 72, 72, 76],
            &[
                8999, 9001, 9002, 9004, 9159, 9161, 9162, 9164, 9316, 9317, 9325, 9399, 9401, 9402,
                9404, 9476, 9477, 9485, 9559, 9561, 9562, 9564, 9636, 9637, 9645, 9716, 9717, 9725,
                9796, 9797, 9805, 9876, 9877, 9885, 10036, 10037, 10045, 10196, 10197, 10205,
                14108, 14268, 14667, 14829, 16437, 16438,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_walk_north.png",
            &[14, 6, 14, 6],
            &[
                11568, 11728, 11967, 12129, 12530, 12690, 12929, 13090, 14126, 14128, 14286, 14288,
                14443, 14444, 14446, 14526, 14528, 14603, 14604, 14606, 14686, 14688, 14760, 14761,
                14762, 14763, 14764, 14765, 14766, 14843, 14844, 14846, 14920, 14921, 14922, 14923,
                14924, 14925, 14926, 15003, 15004, 15006, 15081, 15082, 15083, 15084, 15160, 15161,
                15162, 15163, 15164, 15165, 15241, 15242, 15243, 15244, 15320, 15321, 15322, 15323,
                15324, 15325, 15481, 15482, 15483, 15641, 15642, 15643, 16117, 16282, 16437, 16438,
                16601, 16602, 16757, 16758, 16921, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_autumn_walk_south.png",
            &[89, 81, 89, 81],
            &[
                8997, 9002, 9157, 9162, 9315, 9324, 9397, 9402, 9475, 9484, 9557, 9562, 9635, 9636,
                9643, 9644, 9715, 9724, 9795, 9796, 9803, 9804, 9875, 9884, 10035, 10036, 10043,
                10044, 10195, 10196, 10203, 10204, 14109, 14205, 14206, 14269, 14353, 14354, 14510,
                15796,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_meditate_end_south.png",
            &[77, 73, 77],
            &[
                7237, 7242, 7397, 7402, 7475, 7484, 7557, 7562, 7635, 7644, 7716, 7723, 7795, 7804,
                7875, 7876, 7883, 7884, 8036, 8043, 8669, 8990, 9071, 11476,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_meditate_loop_south.png",
            &[77, 77, 77, 77, 77, 77, 77, 77],
            &[
                18277, 18282, 18357, 18362, 18437, 18442, 18517, 18522, 18597, 18602, 18677, 18682,
                18757, 18762, 18837, 18842, 18915, 18924, 18995, 19004, 19075, 19084, 19155, 19164,
                19235, 19244, 19315, 19324, 19395, 19404, 19475, 19484, 19556, 19563, 19636, 19643,
                19716, 19723, 19796, 19803, 19876, 19883, 19956, 19963, 20036, 20043, 20116, 20123,
                21469, 21550, 21789, 21870, 23388, 23468, 23708, 23788, 25396, 25556, 25716, 25876,
                26034, 26045, 26194, 26205, 26354, 26365, 26514, 26525, 26673, 26686, 26829, 26833,
                26846, 26993, 27006, 27149, 27153, 27166, 27230, 27390, 27550, 27710,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_meditate_start_south.png",
            &[77, 73, 77],
            &[
                7237, 7242, 7397, 7402, 7475, 7484, 7557, 7562, 7635, 7644, 7715, 7716, 7723, 7724,
                7795, 7804, 7876, 7883, 8036, 8043, 8829, 8911, 8990, 11316,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_read_sit_end_south.png",
            &[46, 40, 56],
            &[
                7637, 7642, 7717, 7722, 7797, 7802, 7875, 7884, 7955, 7964, 8035, 8044, 8115, 8116,
                8123, 8124, 8195, 8196, 8203, 8204, 8275, 8276, 8283, 8284, 9550, 9630, 9710,
                11244, 11717, 11722, 11957, 11962, 12197, 12198, 12201, 12202, 12437, 12438, 12441,
                12442, 12677, 12682,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_read_sit_loop_south.png",
            &[30, 42, 30, 42],
            &[
                9959, 9961, 9962, 9964, 10115, 10117, 10118, 10120, 10276, 10277, 10285, 10357,
                10362, 10434, 10442, 10443, 10517, 10522, 10596, 10597, 10605, 10675, 10684, 10754,
                10762, 10763, 10835, 10844, 10995, 10996, 11003, 11004, 11155, 11156, 11163, 11164,
                12509, 12910, 13070, 16358, 16361, 16362, 16517, 16518, 16521, 16677, 16682, 16757,
                16762, 16837, 16842, 16917, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_read_sit_start_south.png",
            &[56, 44, 42],
            &[
                7477, 7482, 7715, 7724, 7797, 7802, 7877, 7882, 7955, 7956, 7963, 7964, 8035, 8044,
                8115, 8124, 8275, 8276, 8283, 8284, 8355, 8356, 8363, 8364, 9390, 9710, 9790,
                11084, 11557, 11562, 11797, 11802, 12037, 12038, 12041, 12042, 12277, 12278, 12281,
                12282, 12517, 12522,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_spell_cast_end_south.png",
            &[89],
            &[2357, 2362, 2435, 2444, 2515, 2516, 2523, 2524, 3629],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_spell_cast_loop_south.png",
            &[101, 101, 100, 100],
            &[
                8679, 8680, 8759, 8760, 8839, 8840, 8919, 8920, 8997, 9002, 9077, 9082, 9157, 9162,
                9237, 9242, 9315, 9324, 9395, 9404, 9475, 9484, 9555, 9564, 9635, 9636, 9643, 9644,
                9715, 9716, 9723, 9724, 9795, 9796, 9803, 9804, 9875, 9876, 9883, 9884, 12588,
                14109, 14189, 14269, 15476, 15556,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_spell_cast_start_south.png",
            &[89, 100],
            &[
                4597, 4602, 4677, 4682, 4755, 4764, 4835, 4844, 4915, 4916, 4923, 4924, 4995, 4996,
                5003, 5004, 7149, 7229,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_write_end_south.png",
            &[78, 82],
            &[
                4519, 4521, 4522, 4524, 4676, 4677, 4685, 4757, 4762, 4836, 4837, 4845, 4915, 4924,
                5075, 5076, 5083, 5084, 7309,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_write_loop_south.png",
            &[67, 68, 73, 70],
            &[
                8999, 9001, 9002, 9004, 9079, 9081, 9082, 9084, 9159, 9161, 9162, 9164, 9239, 9241,
                9242, 9244, 9316, 9317, 9325, 9396, 9397, 9405, 9476, 9477, 9485, 9556, 9557, 9565,
                9636, 9637, 9645, 9716, 9717, 9725, 9796, 9797, 9805, 9876, 9877, 9885,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_write_sit_end_south.png",
            &[55, 61],
            &[
                4999, 5001, 5002, 5004, 5156, 5157, 5165, 5237, 5242, 5316, 5317, 5325, 5395, 5404,
                5555, 5556, 5563, 5564, 6510, 7396, 7636, 7796, 7797, 7957, 8037, 8038, 8041, 8117,
                8118, 8121, 8122, 8197, 8198, 8201, 8202, 8277, 8278, 8281, 8282, 8357, 8362, 8437,
                8442,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_write_sit_loop_south.png",
            &[45, 46, 52, 48],
            &[
                9959, 9961, 9962, 9964, 10039, 10041, 10042, 10044, 10119, 10121, 10122, 10124,
                10199, 10201, 10202, 10204, 10276, 10277, 10285, 10356, 10357, 10365, 10436, 10437,
                10445, 10516, 10517, 10525, 10596, 10597, 10605, 10676, 10677, 10685, 10756, 10757,
                10765, 10836, 10837, 10845, 15396, 15717, 15797, 15957, 16037, 16038, 16041, 16042,
                16117, 16118, 16121, 16122, 16197, 16198, 16277, 16278, 16281, 16282, 16357, 16358,
                16361, 16362, 16437, 16438, 16441, 16442, 16517, 16518, 16521, 16522, 16597, 16598,
                16601, 16602, 16677, 16682, 16757, 16762, 16837, 16842, 16917, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_write_sit_start_south.png",
            &[61, 55],
            &[
                5079, 5081, 5082, 5084, 5157, 5162, 5236, 5237, 5245, 5315, 5324, 5396, 5397, 5405,
                5475, 5476, 5483, 5484, 6430, 7476, 7556, 7716, 7717, 7877, 8037, 8038, 8041, 8042,
                8117, 8118, 8121, 8197, 8198, 8201, 8202, 8277, 8278, 8281, 8282, 8357, 8362, 8437,
                8442,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Autumn/spr_npc_seridia_specialanimation_autumn_write_start_south.png",
            &[82, 78],
            &[
                4599, 4601, 4602, 4604, 4677, 4682, 4756, 4757, 4765, 4835, 4844, 4916, 4917, 4925,
                4995, 4996, 5003, 5004, 7229,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_action_east.png",
            &[79, 87, 84, 87, 84, 79, 86],
            &[
                15802, 15804, 15805, 15807, 15882, 15884, 15885, 15887, 15962, 15964, 15965, 15967,
                16042, 16044, 16045, 16047, 16199, 16201, 16202, 16204, 16280, 16282, 16283, 16285,
                16359, 16360, 16368, 16439, 16440, 16448, 16519, 16520, 16528, 16599, 16600, 16608,
                16680, 16682, 16683, 16685, 16756, 16757, 16765, 16837, 16838, 16846, 16919, 16920,
                16928, 16999, 17000, 17008, 17079, 17080, 17088, 17159, 17160, 17168, 17237, 17238,
                17246, 17316, 17317, 17325, 17397, 17398, 17406, 17797, 17798, 17806, 19168, 19248,
                19328, 19408, 19712, 19792, 19872, 19952, 20191, 20591, 20670, 21390, 21470, 21550,
                21630, 21869, 21960, 22040, 22120, 22200, 22269, 22348, 22523, 22601, 22683, 22761,
                23162, 23322, 23477, 24190, 24270, 24350, 24430, 24669, 24751, 24752, 24753, 24831,
                24832, 24833, 24911, 24912, 24913, 24991, 24992, 24993, 25069, 25148, 25231, 25232,
                25233, 25312, 25313, 25314, 25315, 25316, 25317, 25318, 25392, 25393, 25394, 25395,
                25396, 25397, 25398, 25472, 25473, 25474, 25475, 25476, 25477, 25478, 25552, 25553,
                25554, 25555, 25556, 25557, 25558, 25631, 25632, 25633, 25710, 25711, 25712, 25718,
                25792, 25793, 25794, 25795, 25796, 25797, 25874, 25875, 25876, 25877, 25954, 25955,
                25956, 25957, 26034, 26035, 26036, 26037, 26114, 26115, 26116, 26117, 26192, 26193,
                26194, 26195, 26196, 26197, 26271, 26272, 26273, 26274, 26275, 26276, 26278, 26279,
                26355, 26356, 26357, 26359, 26755, 26756, 26757, 26759, 26834, 26835, 26836,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_action_north.png",
            &[24, 23, 23, 23, 23, 23, 27],
            &[
                20208, 20608, 20688, 20849, 20929, 21009, 21089, 21890, 22290, 22370, 22530, 22610,
                22690, 22770, 24686, 24688, 24766, 24768, 24846, 24848, 24926, 24928, 25006, 25008,
                25086, 25088, 25166, 25168, 25243, 25244, 25246, 25320, 25321, 25322, 25323, 25324,
                25326, 25400, 25401, 25402, 25403, 25404, 25406, 25480, 25481, 25482, 25483, 25484,
                25486, 25560, 25561, 25562, 25563, 25564, 25566, 25643, 25644, 25646, 25723, 25724,
                25726, 25800, 25801, 25802, 25803, 25804, 25805, 25806, 25881, 25882, 25883, 25884,
                25885, 25886, 25961, 25962, 25963, 25964, 25965, 25966, 26041, 26042, 26043, 26044,
                26045, 26046, 26121, 26122, 26123, 26124, 26125, 26126, 26200, 26201, 26202, 26203,
                26204, 26205, 26206, 26280, 26281, 26282, 26283, 26284, 26285, 26286, 26361, 26362,
                26363, 26364, 26761, 26762, 26763, 26764, 26841, 26842, 26843, 26844,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_action_south.png",
            &[111, 104, 100, 104, 100, 111, 114],
            &[
                15797, 15802, 15877, 15882, 15957, 15962, 16037, 16042, 16197, 16202, 16277, 16282,
                16355, 16364, 16435, 16444, 16515, 16524, 16595, 16604, 16677, 16682, 16755, 16764,
                16835, 16844, 16915, 16916, 16923, 16924, 16995, 16996, 17003, 17004, 17075, 17076,
                17083, 17084, 17155, 17156, 17163, 17164, 17235, 17244, 17315, 17316, 17323, 17324,
                17395, 17396, 17403, 17404, 17795, 17796, 17803, 17804, 20270, 20350, 20430, 20510,
                20671, 20751, 21151, 21948, 21963, 22028, 22043, 22108, 22123, 22188, 22203, 22349,
                22429, 22829, 22914, 22925, 23643, 23644, 23645, 23723, 23724, 23725, 23803, 23804,
                23805, 23883, 23884, 23885, 24204, 24205, 24284, 24285, 24364, 24365, 24444, 24445,
                24749, 24829, 24909, 24989, 25149, 25229, 25311, 25391, 25471, 25551, 25629, 25711,
                25791, 26191,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_bath_swim_east.png",
            &[37, 37, 33, 33],
            &[
                14119, 14121, 14122, 14124, 14199, 14201, 14202, 14204, 14436, 14437, 14445, 14516,
                14517, 14525, 14599, 14601, 14602, 14604, 14679, 14681, 14682, 14684, 14756, 14757,
                14765, 14836, 14837, 14845, 14916, 14917, 14925, 14996, 14997, 15005, 15236, 15237,
                15245, 15316, 15317, 15325,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_bath_swim_south.png",
            &[42, 42, 38, 38],
            &[
                14117, 14122, 14197, 14202, 14435, 14444, 14515, 14524, 14597, 14602, 14677, 14682,
                14755, 14756, 14763, 14764, 14835, 14836, 14843, 14844, 14915, 14924, 14995, 15004,
                15235, 15236, 15243, 15244, 15315, 15316, 15323, 15324,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_blink_east.png",
            &[86, 90, 86],
            &[
                6759, 6761, 6762, 6764, 6839, 6841, 6842, 6844, 6919, 6921, 6922, 6924, 6996, 6997,
                7005, 7076, 7077, 7085, 7156, 7157, 7165, 7236, 7237, 7245, 7316, 7317, 7325, 7396,
                7397, 7405, 8670, 8750, 8830, 9388, 9468, 9548, 9877, 9957, 10037, 10588, 10668,
                10748, 10830, 10831, 10832, 10838, 10910, 10911, 10912, 10918, 10990, 10991, 10992,
                10998, 11071, 11072, 11073, 11074, 11075, 11076, 11078, 11079, 11151, 11152, 11153,
                11154, 11155, 11156, 11158, 11159, 11231, 11232, 11233, 11234, 11235, 11236, 11238,
                11239, 11314, 11315, 11316, 11394, 11395, 11396, 11474, 11475, 11476,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_blink_south.png",
            &[114, 118, 114],
            &[
                6757, 6762, 6837, 6842, 6917, 6922, 6995, 7004, 7075, 7084, 7155, 7164, 7235, 7236,
                7243, 7244, 7315, 7316, 7323, 7324, 7395, 7396, 7403, 7404, 8671, 8751, 8831, 9389,
                9469, 9549, 9634, 9645, 9714, 9725, 9794, 9805, 10589, 10669, 10749, 10831, 10911,
                10991,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_idle_east.png",
            &[86],
            &[
                2279, 2281, 2282, 2284, 2356, 2357, 2365, 2436, 2437, 2445, 2910, 3148, 3317, 3548,
                3630, 3631, 3632, 3638, 3711, 3712, 3713, 3714, 3715, 3716, 3718, 3719, 3794, 3795,
                3796,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_idle_north.png",
            &[27],
            &[
                2928, 3170, 3566, 3568, 3643, 3644, 3646, 3720, 3721, 3722, 3723, 3724, 3725, 3726,
                3801, 3802, 3803, 3804,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_idle_south.png",
            &[114],
            &[
                2277, 2282, 2355, 2364, 2435, 2436, 2443, 2444, 2911, 3149, 3234, 3245, 3549, 3631,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_kiss_east.png",
            &[84, 83, 99, 83],
            &[
                9162, 9164, 9165, 9167, 9318, 9323, 9400, 9402, 9403, 9405, 9479, 9480, 9488, 9560,
                9562, 9563, 9565, 9635, 9636, 9644, 9717, 9718, 9726, 9799, 9800, 9808, 9877, 9878,
                9886, 9955, 9956, 9964, 10037, 10038, 10046, 10197, 10198, 10206, 11088, 11709,
                11869, 11949, 12109, 12669, 12678, 12827, 12908, 13068, 14270, 14428, 14509, 14592,
                14593, 14594, 14669, 14750, 14751, 14752, 14757, 14831, 14832, 14833, 14913, 14914,
                14915, 14916, 14991, 14992, 14993, 15071, 15072, 15073, 15074, 15075, 15077, 15078,
                15152, 15153, 15154, 15155, 15156, 15159, 15236, 15312, 15313, 15314, 15315, 15316,
                15319, 15394, 15395, 15475, 15476, 15480, 15635, 15636, 15640,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_sit_east.png",
            &[62],
            &[
                2519, 2521, 2522, 2524, 2596, 2597, 2605, 2676, 2677, 2685, 3149, 3476, 3549, 3557,
                3631, 3632, 3633, 3712, 3800,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_sit_north.png",
            &[0],
            &[
                3008, 3250, 3646, 3648, 3723, 3724, 3726, 3800, 3801, 3802, 3803, 3804, 3805, 3806,
                3881, 3882, 3883, 3884,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_sit_south.png",
            &[78],
            &[
                2517, 2522, 2595, 2604, 2675, 2676, 2683, 2684, 3150, 3475, 3484, 3550, 3632, 3876,
                3877, 3878, 3957, 3958, 4037, 4038, 4117, 4118, 4197,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_walk_east.png",
            &[86, 85, 86, 92],
            &[
                8999, 9001, 9002, 9004, 9159, 9161, 9162, 9164, 9316, 9317, 9325, 9399, 9401, 9402,
                9404, 9476, 9477, 9485, 9559, 9561, 9562, 9564, 9636, 9637, 9645, 9716, 9717, 9725,
                9796, 9797, 9805, 9876, 9877, 9885, 10036, 10037, 10045, 10196, 10197, 10205,
                11550, 11710, 12108, 12271, 12508, 12668, 13067, 13157, 13229, 13237, 13317, 13556,
                14108, 14268, 14430, 14431, 14432, 14438, 14590, 14591, 14592, 14598, 14667, 14751,
                14752, 14753, 14754, 14755, 14756, 14758, 14759, 14829, 14836, 14911, 14912, 14913,
                14914, 14915, 14916, 14918, 14919, 14989, 14990, 14991, 14998, 15074, 15075, 15076,
                15151, 15152, 15153, 15154, 15155, 15156, 15234, 15235, 15236, 15310, 15311, 15312,
                15313, 15314, 15315, 15316, 15319, 15472, 15473, 15474, 15475, 15476, 15482, 15633,
                15634, 15635, 16437, 16438,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_walk_north.png",
            &[27, 10, 27, 12],
            &[
                11568, 11728, 11967, 12129, 12530, 12690, 12929, 13090, 13873, 13884, 13885, 13886,
                14033, 14126, 14128, 14205, 14286, 14288, 14353, 14354, 14443, 14444, 14446, 14526,
                14528, 14603, 14604, 14606, 14686, 14688, 14760, 14761, 14762, 14763, 14764, 14765,
                14766, 14843, 14844, 14846, 14920, 14921, 14922, 14923, 14924, 14925, 14926, 15003,
                15004, 15006, 15081, 15082, 15083, 15084, 15160, 15161, 15162, 15163, 15164, 15165,
                15241, 15242, 15243, 15244, 15320, 15321, 15322, 15323, 15324, 15325, 15477, 15481,
                15482, 15483, 15641, 15642, 15643, 15797, 15798, 16117, 16118, 16281, 16282, 16437,
                16438, 16601, 16602, 16757, 16758, 16921, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Beach/spr_npc_seridia_beach_walk_south.png",
            &[114, 104, 114, 106],
            &[
                8997, 9002, 9157, 9162, 9315, 9324, 9397, 9402, 9475, 9484, 9557, 9562, 9635, 9636,
                9643, 9644, 9715, 9724, 9795, 9796, 9803, 9804, 9875, 9884, 10035, 10036, 10043,
                10044, 10195, 10196, 10203, 10204, 11551, 11711, 12430, 12509, 12669, 12834, 12845,
                12910, 12994, 13005, 13388, 14109, 14269, 14431, 14510, 14591, 14832, 14836, 14990,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_blink_east.png",
            &[29, 33, 29],
            &[],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_blink_south.png",
            &[31, 35, 31],
            &[],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_idle_east.png",
            &[26],
            &[],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_idle_north.png",
            &[0],
            &[],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_idle_south.png",
            &[28],
            &[],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_walk_east.png",
            &[26, 26, 26, 26],
            &[],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_walk_north.png",
            &[0, 2, 0, 2],
            &[],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Flashback Priestess/spr_npc_seridia_flashback_priestess_walk_south.png",
            &[28, 26, 28, 26],
            &[],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_meditate_end_south.png",
            &[68, 64, 67],
            &[
                7237, 7242, 7397, 7402, 7475, 7484, 7557, 7562, 7635, 7644, 7716, 7723, 7795, 7804,
                7875, 7876, 7883, 7884, 8036, 8043, 8669, 8990, 10836, 10996, 11156, 11236, 11315,
                11316, 11476, 11555, 11556, 11557, 11635, 11636, 11875, 11876, 11877, 11956, 12196,
                12197, 12437, 12438, 12678, 12918,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_meditate_loop_south.png",
            &[68, 68, 68, 68, 68, 68, 68, 68],
            &[
                18277, 18282, 18357, 18362, 18437, 18442, 18517, 18522, 18597, 18602, 18677, 18682,
                18757, 18762, 18837, 18842, 18915, 18924, 18995, 19004, 19075, 19084, 19155, 19164,
                19235, 19244, 19315, 19324, 19395, 19404, 19475, 19484, 19556, 19563, 19636, 19643,
                19716, 19723, 19796, 19803, 19876, 19883, 19956, 19963, 20036, 20043, 20116, 20123,
                21469, 21550, 21789, 21870, 23388, 23468, 23708, 23788, 24114, 24125, 24274, 24285,
                24434, 24445, 24594, 24605, 26679, 26680, 26829, 26839, 26840, 26999, 27000, 27149,
                27159, 27160, 27230, 27390, 27550, 27710, 27876, 27956, 28036, 28116, 28196, 28276,
                28356, 28436, 29155, 29156, 29235, 29236, 29315, 29316, 29395, 29396, 29475, 29476,
                29555, 29556, 29635, 29636, 29715, 29716, 29795, 29796, 29797, 29875, 29876, 29877,
                29955, 29956, 29957, 30035, 30036, 30037, 30115, 30116, 30117, 30195, 30196, 30197,
                30275, 30276, 30277, 30355, 30356, 30357,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_meditate_start_south.png",
            &[67, 64, 68],
            &[
                7237, 7242, 7397, 7402, 7475, 7484, 7557, 7562, 7635, 7644, 7715, 7716, 7723, 7724,
                7795, 7804, 7876, 7883, 8036, 8043, 8829, 8990, 10836, 10996, 11076, 11156, 11316,
                11475, 11476, 11635, 11636, 11715, 11716, 11717, 11796, 11875, 11876, 11877, 12036,
                12037, 12277, 12278, 12518, 12758,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_read_sit_end_south.png",
            &[46, 40, 51],
            &[
                7637, 7642, 7717, 7722, 7797, 7802, 7875, 7884, 7955, 7964, 8035, 8044, 8115, 8116,
                8123, 8124, 8195, 8196, 8203, 8204, 8275, 8276, 8283, 8284, 9550, 9630, 9710,
                11244, 11957, 11962, 12197, 12198, 12201, 12202, 12437, 12438, 12441, 12442, 12677,
                12682,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_read_sit_loop_south.png",
            &[28, 42, 28, 42],
            &[
                9959, 9961, 9962, 9964, 10115, 10117, 10118, 10120, 10276, 10277, 10285, 10357,
                10362, 10434, 10442, 10443, 10517, 10522, 10596, 10597, 10605, 10675, 10684, 10754,
                10762, 10763, 10835, 10844, 10995, 10996, 11003, 11004, 11155, 11156, 11163, 11164,
                12509, 12910, 13070, 16358, 16361, 16362, 16517, 16518, 16521, 16677, 16682, 16757,
                16762, 16837, 16842, 16917, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_read_sit_start_south.png",
            &[51, 44, 42],
            &[
                7477, 7482, 7715, 7724, 7797, 7802, 7877, 7882, 7955, 7956, 7963, 7964, 8035, 8044,
                8115, 8124, 8275, 8276, 8283, 8284, 8355, 8356, 8363, 8364, 9390, 9710, 9790,
                11084, 11797, 11802, 12037, 12038, 12041, 12042, 12277, 12278, 12281, 12282, 12517,
                12522,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_spell_cast_end_south.png",
            &[79],
            &[
                2357, 2362, 2435, 2444, 2515, 2516, 2523, 2524, 3629, 3636, 3716, 3796, 3956,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_spell_cast_loop_south.png",
            &[88, 96, 92, 91],
            &[
                8679, 8680, 8759, 8760, 8839, 8840, 8919, 8920, 8997, 9002, 9077, 9082, 9157, 9162,
                9237, 9242, 9315, 9324, 9395, 9404, 9475, 9484, 9555, 9564, 9635, 9636, 9643, 9644,
                9715, 9716, 9723, 9724, 9795, 9796, 9803, 9804, 9875, 9876, 9883, 9884, 12588,
                14109, 14116, 14123, 14189, 14196, 14269, 14276, 14356, 14363, 14436, 14443, 14596,
                14676, 14756, 14763, 14916, 14996, 15476, 15643,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_spell_cast_start_south.png",
            &[79, 91],
            &[
                4597, 4602, 4677, 4682, 4755, 4764, 4835, 4844, 4915, 4916, 4923, 4924, 4995, 4996,
                5003, 5004, 7149, 7156, 7229, 7236, 7316, 7396, 7476, 7556, 7796, 7876,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_write_end_south.png",
            &[62, 67],
            &[
                4519, 4521, 4522, 4524, 4676, 4677, 4685, 4757, 4762, 4836, 4837, 4845, 4915, 4924,
                5075, 5076, 5083, 5084, 6757, 6758, 6918, 6997, 6998, 7309, 7316, 7396, 7476, 7636,
                7716, 7723, 7956,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_write_loop_south.png",
            &[61, 60, 62, 62],
            &[
                8999, 9001, 9002, 9004, 9079, 9081, 9082, 9084, 9159, 9161, 9162, 9164, 9239, 9241,
                9242, 9244, 9316, 9317, 9325, 9396, 9397, 9405, 9476, 9477, 9485, 9556, 9557, 9565,
                9636, 9637, 9645, 9716, 9717, 9725, 9796, 9797, 9805, 9876, 9877, 9885, 14436,
                14756, 14836, 14916, 14996, 15396, 15476, 15556, 15563, 15636,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_write_sit_end_south.png",
            &[48, 53],
            &[
                4999, 5001, 5002, 5004, 5156, 5157, 5165, 5237, 5242, 5316, 5317, 5325, 5395, 5404,
                5555, 5556, 5563, 5564, 6510, 7396, 7636, 7796, 7957, 8037, 8038, 8041, 8117, 8118,
                8121, 8122, 8197, 8198, 8201, 8202, 8277, 8278, 8281, 8282, 8357, 8362, 8437, 8442,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_write_sit_loop_south.png",
            &[43, 42, 46, 44],
            &[
                9959, 9961, 9962, 9964, 10039, 10041, 10042, 10044, 10119, 10121, 10122, 10124,
                10199, 10201, 10202, 10204, 10276, 10277, 10285, 10356, 10357, 10365, 10436, 10437,
                10445, 10516, 10517, 10525, 10596, 10597, 10605, 10676, 10677, 10685, 10756, 10757,
                10765, 10836, 10837, 10845, 15396, 15717, 15797, 15957, 16037, 16038, 16041, 16042,
                16117, 16118, 16121, 16122, 16197, 16198, 16277, 16278, 16281, 16282, 16357, 16358,
                16361, 16362, 16437, 16438, 16441, 16442, 16517, 16518, 16521, 16522, 16597, 16598,
                16601, 16602, 16677, 16682, 16757, 16762, 16837, 16842, 16917, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_write_sit_start_south.png",
            &[53, 48],
            &[
                5079, 5081, 5082, 5084, 5157, 5162, 5236, 5237, 5245, 5315, 5324, 5396, 5397, 5405,
                5475, 5476, 5483, 5484, 6430, 7476, 7556, 7716, 7877, 8037, 8038, 8041, 8042, 8117,
                8118, 8121, 8197, 8198, 8201, 8202, 8277, 8278, 8281, 8282, 8357, 8362, 8437, 8442,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_specialanimation_spring_write_start_south.png",
            &[67, 62],
            &[
                4599, 4601, 4602, 4604, 4677, 4682, 4756, 4757, 4765, 4835, 4844, 4916, 4917, 4925,
                4995, 4996, 5003, 5004, 6837, 6838, 6917, 6918, 6998, 7229, 7236, 7396, 7476, 7556,
                7796, 7803, 7876,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_action_east.png",
            &[59, 61, 63, 61, 63, 59, 58],
            &[
                15802, 15804, 15805, 15807, 15882, 15884, 15885, 15887, 15962, 15964, 15965, 15967,
                16042, 16044, 16045, 16047, 16199, 16201, 16202, 16204, 16280, 16282, 16283, 16285,
                16359, 16360, 16368, 16439, 16440, 16448, 16519, 16520, 16528, 16599, 16600, 16608,
                16680, 16682, 16683, 16685, 16756, 16757, 16765, 16837, 16838, 16846, 16919, 16920,
                16928, 16999, 17000, 17008, 17079, 17080, 17088, 17159, 17160, 17168, 17237, 17238,
                17246, 17316, 17317, 17325, 17397, 17398, 17406, 17797, 17798, 17806, 23640, 23641,
                23644, 23720, 23721, 23800, 23801, 23804, 23880, 23881, 24038, 24039, 24042, 24123,
                24190, 24201, 24202, 24203, 24204, 24270, 24281, 24282, 24350, 24361, 24362, 24363,
                24364, 24430, 24441, 24442, 24523, 24599, 24600, 24601, 24602, 24669, 24760, 24840,
                24920, 25000, 25069, 25148, 25158, 25320, 25400, 25480, 25560, 25718, 25798, 25879,
                25880, 25959, 25960, 26039, 26040, 26119, 26120, 26198, 26278, 26358, 26758,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_action_north.png",
            &[14, 18, 18, 18, 18, 14, 14],
            &[
                20208, 20608, 20688, 20849, 20929, 21009, 21089, 21890, 22290, 22370, 22530, 22610,
                22690, 22770, 24686, 24688, 24766, 24768, 24846, 24848, 24926, 24928, 25006, 25008,
                25086, 25088, 25166, 25168, 25243, 25244, 25246, 25320, 25321, 25322, 25323, 25324,
                25326, 25400, 25401, 25402, 25403, 25404, 25406, 25480, 25481, 25482, 25483, 25484,
                25486, 25560, 25561, 25562, 25563, 25564, 25566, 25643, 25644, 25646, 25723, 25724,
                25726, 25800, 25801, 25802, 25803, 25804, 25805, 25806, 25881, 25882, 25883, 25884,
                25885, 25886, 25961, 25962, 25963, 25964, 25965, 25966, 26041, 26042, 26043, 26044,
                26045, 26046, 26121, 26122, 26123, 26124, 26125, 26126, 26200, 26201, 26202, 26203,
                26204, 26205, 26206, 26280, 26281, 26282, 26283, 26284, 26285, 26286, 26361, 26362,
                26363, 26364, 26761, 26762, 26763, 26764, 26841, 26842, 26843, 26844,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_action_south.png",
            &[72, 75, 68, 75, 68, 72, 79],
            &[
                15797, 15802, 15877, 15882, 15957, 15962, 16037, 16042, 16197, 16202, 16277, 16282,
                16355, 16364, 16435, 16444, 16515, 16524, 16595, 16604, 16677, 16682, 16755, 16764,
                16835, 16844, 16915, 16916, 16923, 16924, 16995, 16996, 17003, 17004, 17075, 17076,
                17083, 17084, 17155, 17156, 17163, 17164, 17235, 17244, 17315, 17316, 17323, 17324,
                17395, 17396, 17403, 17404, 17795, 17796, 17803, 17804, 20270, 20350, 20430, 20510,
                21948, 22028, 22108, 22188, 23645, 23721, 23722, 23725, 23805, 23881, 23882, 23885,
                24204, 24205, 24281, 24284, 24285, 24364, 24365, 24441, 24444, 24445, 24685, 24686,
                24749, 24829, 24909, 24989, 25085, 25086, 25149, 25156, 25229, 25396, 25556, 25629,
                25716, 25796, 25876, 25956, 26036, 26116, 26196, 26276, 26356, 26756, 26996, 27076,
                27156, 27236, 27396, 27476, 27876,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_blink_east.png",
            &[58, 62, 58],
            &[
                6759, 6761, 6762, 6764, 6839, 6841, 6842, 6844, 6919, 6921, 6922, 6924, 6996, 6997,
                7005, 7076, 7077, 7085, 7156, 7157, 7165, 7236, 7237, 7245, 7316, 7317, 7325, 7396,
                7397, 7405, 10118, 10119, 10122, 10198, 10199, 10202, 10278, 10279, 10282, 10359,
                10360, 10361, 10362, 10439, 10440, 10441, 10442, 10519, 10520, 10521, 10522, 10588,
                10598, 10668, 10678, 10748, 10758, 10838, 10918, 10998, 11078, 11158, 11238,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_blink_south.png",
            &[79, 83, 79],
            &[
                6757, 6762, 6837, 6842, 6917, 6922, 6995, 7004, 7075, 7084, 7155, 7164, 7235, 7236,
                7243, 7244, 7315, 7316, 7323, 7324, 7395, 7396, 7403, 7404, 10589, 10596, 10669,
                10676, 10749, 10756, 10836, 10916, 10996, 11076, 11156, 11236, 11556, 11636, 11716,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_drink_east.png",
            &[36, 39, 36],
            &[
                7479, 7481, 7482, 7484, 7561, 7639, 7641, 7642, 7644, 7716, 7717, 7725, 7796, 7802,
                7876, 7877, 7885, 7956, 7957, 7965, 8034, 8042, 8116, 8117, 8125, 8274, 9229,
                11079, 11080, 11159, 11160, 11239, 11240, 11560, 11561, 11640, 11641, 11720, 11721,
                11801, 11802, 11804, 11881, 11882, 11884, 11961, 11962, 11964, 12041, 12042, 12121,
                12122, 12201, 12202, 12281, 12361, 12441,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_drink_north.png",
            &[0, 0, 0],
            &[
                8928, 9088, 9489, 9650, 9810, 10210, 10846, 10848, 11006, 11008, 11083, 11084,
                11086, 11166, 11243, 11244, 11246, 11320, 11321, 11322, 11323, 11324, 11325, 11326,
                11400, 11401, 11402, 11403, 11404, 11406, 11408, 11480, 11481, 11482, 11483, 11484,
                11485, 11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_drink_south.png",
            &[54, 52, 54],
            &[
                7477, 7482, 7637, 7642, 7715, 7724, 7797, 7802, 7875, 7884, 7955, 7956, 7963, 7964,
                8035, 8044, 8115, 8116, 8123, 8124, 8275, 8276, 8283, 8284, 8756, 8763, 9390, 9550,
                9709, 10676, 11797, 11802, 11877, 11882, 11957, 11962, 12037, 12038, 12041, 12042,
                12117, 12118, 12121, 12122, 12197, 12198, 12201, 12202, 12277, 12278, 12281, 12282,
                12357, 12358, 12361, 12362, 12437, 12438, 12441, 12442, 12517, 12522, 12597, 12602,
                12677, 12682,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_eat_east.png",
            &[34, 33, 27, 35, 40],
            &[
                12199, 12201, 12202, 12204, 12439, 12441, 12442, 12444, 12520, 12522, 12523, 12525,
                12596, 12597, 12605, 12759, 12761, 12762, 12764, 12836, 12837, 12845, 12917, 12918,
                12926, 12996, 12997, 13005, 13079, 13081, 13082, 13084, 13156, 13157, 13165, 13236,
                13237, 13245, 13317, 13318, 13326, 13476, 13477, 13485, 13556, 13557, 13565, 13804,
                13876, 13877, 13885, 14125, 14684, 15469, 18439, 18440, 18519, 18520, 18599, 18600,
                18679, 18680, 18759, 19240, 19241, 19320, 19321, 19400, 19401, 19480, 19481, 19560,
                19561, 19641, 19642, 19644, 19721, 19722, 19724, 19801, 19802, 19804, 19881, 19882,
                19884, 19961, 19962, 19964, 20041, 20042, 20121, 20122, 20201, 20202, 20281, 20282,
                20361, 20362, 20441, 20521, 20601, 20681, 20761,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_eat_north.png",
            &[0, 0, 0],
            &[
                8928, 9088, 9489, 9650, 9810, 10450, 10846, 10848, 11006, 11008, 11083, 11084,
                11086, 11166, 11243, 11244, 11246, 11320, 11321, 11322, 11323, 11324, 11325, 11326,
                11400, 11401, 11402, 11403, 11404, 11406, 11408, 11480, 11481, 11482, 11483, 11484,
                11485, 11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_eat_south.png",
            &[54, 48, 33, 55, 54],
            &[
                12199, 12200, 12437, 12442, 12597, 12602, 12757, 12762, 12835, 12844, 12917, 12922,
                12995, 13004, 13077, 13082, 13155, 13164, 13235, 13236, 13243, 13244, 13315, 13324,
                13395, 13396, 13403, 13404, 13475, 13484, 13555, 13556, 13563, 13564, 13715, 13716,
                13723, 13724, 13875, 13876, 13883, 13884, 14516, 14523, 14676, 14683, 15390, 15630,
                15870, 15950, 19637, 19642, 19722, 19797, 19802, 19877, 19882, 19957, 19962, 20037,
                20038, 20041, 20042, 20121, 20122, 20197, 20198, 20201, 20202, 20277, 20278, 20281,
                20282, 20357, 20358, 20361, 20362, 20437, 20438, 20441, 20442, 20517, 20518, 20521,
                20522, 20597, 20598, 20601, 20602, 20677, 20678, 20681, 20682, 20757, 20758, 20761,
                20762, 20837, 20842, 20917, 20922, 20997, 21002, 21077, 21082, 21157, 21162,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_idle_east.png",
            &[58],
            &[
                2279, 2281, 2282, 2284, 2356, 2357, 2365, 2436, 2437, 2445, 3398, 3399, 3402, 3479,
                3480, 3481, 3482, 3548, 3558, 3638, 3718,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_idle_north.png",
            &[14],
            &[
                2928, 3170, 3566, 3568, 3643, 3644, 3646, 3720, 3721, 3722, 3723, 3724, 3725, 3726,
                3801, 3802, 3803, 3804,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_idle_south.png",
            &[79],
            &[
                2277, 2282, 2355, 2364, 2435, 2436, 2443, 2444, 3549, 3556, 3636, 3716, 3876,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_kiss_east.png",
            &[58, 52, 72, 52],
            &[
                9162, 9164, 9165, 9167, 9318, 9323, 9400, 9402, 9403, 9405, 9479, 9480, 9488, 9560,
                9562, 9563, 9565, 9635, 9636, 9644, 9717, 9718, 9726, 9799, 9800, 9808, 9877, 9878,
                9886, 9955, 9956, 9964, 10037, 10038, 10046, 10197, 10198, 10206, 11709, 11949,
                12109, 12908, 13068, 13640, 13641, 13798, 13801, 13879, 13880, 13883, 13959, 13961,
                13962, 13963, 14039, 14040, 14043, 14118, 14119, 14120, 14121, 14200, 14201, 14202,
                14203, 14279, 14280, 14360, 14361, 14362, 14363, 14428, 14509, 14519, 14599, 14600,
                14669, 14679, 14757, 14839, 14919, 14920, 14999, 15077, 15159, 15319, 16441, 16442,
                16601, 16602, 16761, 16762, 16921, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_sit_east.png",
            &[42],
            &[
                2519, 2521, 2522, 2524, 2596, 2597, 2605, 2676, 2677, 2685, 3719, 3720, 3880, 3881,
                3961, 3962, 3964, 4041, 4042, 4121,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_sit_north.png",
            &[0],
            &[
                3008, 3250, 3646, 3648, 3723, 3724, 3726, 3800, 3801, 3802, 3803, 3804, 3805, 3806,
                3881, 3882, 3883, 3884,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_sit_south.png",
            &[54],
            &[
                2517, 2522, 2595, 2604, 2675, 2676, 2683, 2684, 3150, 3957, 3962, 4037, 4038, 4041,
                4042, 4117, 4118, 4121, 4122, 4197, 4202,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_sleep_east.png",
            &[47],
            &[
                2359, 2361, 2362, 2364, 2436, 2437, 2445, 2516, 2517, 2525, 3478, 3479, 3482, 3559,
                3560, 3561, 3562, 3638, 3718, 4121, 4122, 4202,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_walk_east.png",
            &[58, 55, 58, 62],
            &[
                8999, 9001, 9002, 9004, 9159, 9161, 9162, 9164, 9316, 9317, 9325, 9399, 9401, 9402,
                9404, 9476, 9477, 9485, 9559, 9561, 9562, 9564, 9636, 9637, 9645, 9716, 9717, 9725,
                9796, 9797, 9805, 9876, 9877, 9885, 10036, 10037, 10045, 10196, 10197, 10205,
                13478, 13479, 13482, 13638, 13639, 13642, 13799, 13800, 13801, 13802, 13878, 13879,
                13882, 13959, 13960, 13961, 13962, 14039, 14042, 14108, 14118, 14199, 14200, 14201,
                14202, 14268, 14278, 14359, 14360, 14361, 14362, 14438, 14518, 14598, 14667, 14758,
                14829, 14838, 14918, 14998, 15158, 15318, 16437, 16438,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_walk_north.png",
            &[14, 6, 14, 6],
            &[
                11568, 11728, 11967, 12129, 12530, 12690, 12929, 13090, 14126, 14128, 14286, 14288,
                14443, 14444, 14446, 14526, 14528, 14603, 14604, 14606, 14686, 14688, 14760, 14761,
                14762, 14763, 14764, 14765, 14766, 14843, 14844, 14846, 14920, 14921, 14922, 14923,
                14924, 14925, 14926, 15003, 15004, 15006, 15081, 15082, 15083, 15084, 15160, 15161,
                15162, 15163, 15164, 15165, 15241, 15242, 15243, 15244, 15320, 15321, 15322, 15323,
                15324, 15325, 15481, 15482, 15483, 15641, 15642, 15643, 15797, 16117, 16118, 16281,
                16282, 16437, 16438, 16601, 16602, 16757, 16758, 16921, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Spring/spr_npc_seridia_spring_walk_south.png",
            &[79, 73, 79, 73],
            &[
                8997, 9002, 9157, 9162, 9315, 9324, 9397, 9402, 9475, 9484, 9557, 9562, 9635, 9636,
                9643, 9644, 9715, 9724, 9795, 9796, 9803, 9804, 9875, 9884, 10035, 10036, 10043,
                10044, 10195, 10196, 10203, 10204, 14109, 14116, 14205, 14206, 14269, 14276, 14353,
                14354, 14436, 14510, 14596, 14756, 14836, 14916, 15003, 15396, 15556,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_meditate_end_south.png",
            &[72, 68, 76],
            &[
                7237, 7242, 7397, 7402, 7475, 7484, 7557, 7562, 7635, 7644, 7716, 7723, 7795, 7804,
                7875, 7876, 7883, 7884, 8036, 8043, 8669, 8990, 9071, 10274, 10285, 10359, 10360,
                10679, 10680, 11476,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_meditate_loop_south.png",
            &[72, 72, 72, 72, 72, 72, 72, 72],
            &[
                18277, 18282, 18357, 18362, 18437, 18442, 18517, 18522, 18597, 18602, 18677, 18682,
                18757, 18762, 18837, 18842, 18915, 18924, 18995, 19004, 19075, 19084, 19155, 19164,
                19235, 19244, 19315, 19324, 19395, 19404, 19475, 19484, 19556, 19563, 19636, 19643,
                19716, 19723, 19796, 19803, 19876, 19883, 19956, 19963, 20036, 20043, 20116, 20123,
                21469, 21550, 21789, 21870, 23388, 23468, 23708, 23788, 26599, 26600, 26679, 26680,
                26759, 26760, 26829, 26839, 26840, 26919, 26920, 26999, 27000, 27079, 27080, 27149,
                27159, 27160, 27230, 27390, 27550, 27710,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_meditate_start_south.png",
            &[76, 68, 72],
            &[
                7237, 7242, 7397, 7402, 7475, 7484, 7557, 7562, 7635, 7644, 7715, 7716, 7723, 7724,
                7795, 7804, 7876, 7883, 8036, 8043, 8829, 8911, 8990, 10114, 10125, 10519, 10520,
                10679, 10680, 11316,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_read_sit_end_south.png",
            &[46, 40, 51],
            &[
                7637, 7642, 7717, 7722, 7797, 7802, 7875, 7884, 7955, 7964, 8035, 8044, 8115, 8116,
                8123, 8124, 8195, 8196, 8203, 8204, 8275, 8276, 8283, 8284, 9550, 9630, 9710,
                10515, 10524, 11244, 11717, 11722, 11957, 11958, 11961, 11962, 12197, 12198, 12201,
                12202, 12437, 12438, 12441, 12442, 12677, 12682,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_read_sit_loop_south.png",
            &[30, 42, 30, 42],
            &[
                9959, 9961, 9962, 9964, 10115, 10117, 10118, 10120, 10276, 10277, 10285, 10357,
                10362, 10434, 10442, 10443, 10517, 10522, 10596, 10597, 10605, 10675, 10684, 10754,
                10762, 10763, 10835, 10844, 10995, 10996, 11003, 11004, 11155, 11156, 11163, 11164,
                12509, 12910, 13070, 16358, 16361, 16362, 16517, 16518, 16521, 16677, 16682, 16757,
                16762, 16837, 16842, 16917, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_read_sit_start_south.png",
            &[51, 44, 42],
            &[
                7477, 7482, 7715, 7724, 7797, 7802, 7877, 7882, 7955, 7956, 7963, 7964, 8035, 8044,
                8115, 8124, 8275, 8276, 8283, 8284, 8355, 8356, 8363, 8364, 9390, 9710, 9790,
                10355, 10364, 11084, 11557, 11562, 11797, 11798, 11801, 11802, 12037, 12038, 12041,
                12042, 12277, 12278, 12281, 12282, 12517, 12522,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_spell_cast_end_south.png",
            &[88],
            &[
                2357, 2362, 2435, 2444, 2515, 2516, 2523, 2524, 3314, 3325, 3629, 3716,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_spell_cast_loop_south.png",
            &[92, 91, 91, 89],
            &[
                8679, 8680, 8759, 8760, 8839, 8840, 8919, 8920, 8997, 9002, 9077, 9082, 9157, 9162,
                9237, 9242, 9315, 9324, 9395, 9404, 9475, 9484, 9555, 9564, 9635, 9636, 9643, 9644,
                9715, 9716, 9723, 9724, 9795, 9796, 9803, 9804, 9875, 9876, 9883, 9884, 12515,
                12524, 12588, 12595, 12604, 12675, 12684, 12755, 12764, 14109, 14189, 14269, 14436,
                14596, 14676, 15476, 15643,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_spell_cast_start_south.png",
            &[88, 92],
            &[
                4597, 4602, 4677, 4682, 4755, 4764, 4835, 4844, 4915, 4916, 4923, 4924, 4995, 4996,
                5003, 5004, 6355, 6364, 6594, 6605, 7149, 7229, 7316, 7396,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_write_end_south.png",
            &[74, 76],
            &[
                4519, 4521, 4522, 4524, 4676, 4677, 4685, 4757, 4762, 4836, 4837, 4845, 4915, 4924,
                5075, 5076, 5083, 5084, 6275, 6918, 7309, 7476,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_write_loop_south.png",
            &[69, 69, 74, 72],
            &[
                8999, 9001, 9002, 9004, 9079, 9081, 9082, 9084, 9159, 9161, 9162, 9164, 9239, 9241,
                9242, 9244, 9316, 9317, 9325, 9396, 9397, 9405, 9476, 9477, 9485, 9556, 9557, 9565,
                9636, 9637, 9645, 9716, 9717, 9725, 9796, 9797, 9805, 9876, 9877, 9885, 12835,
                14195, 14436,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_write_sit_end_south.png",
            &[51, 54],
            &[
                4999, 5001, 5002, 5004, 5156, 5157, 5165, 5237, 5242, 5316, 5317, 5325, 5395, 5404,
                5555, 5556, 5563, 5564, 6510, 6755, 7396, 7636, 7796, 7797, 7957, 7958, 8037, 8038,
                8041, 8117, 8118, 8121, 8122, 8197, 8198, 8201, 8202, 8277, 8278, 8281, 8282, 8357,
                8362, 8437, 8442,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_write_sit_loop_south.png",
            &[45, 45, 51, 48],
            &[
                9959, 9961, 9962, 9964, 10039, 10041, 10042, 10044, 10119, 10121, 10122, 10124,
                10199, 10201, 10202, 10204, 10276, 10277, 10285, 10356, 10357, 10365, 10436, 10437,
                10445, 10516, 10517, 10525, 10596, 10597, 10605, 10676, 10677, 10685, 10756, 10757,
                10765, 10836, 10837, 10845, 13795, 15155, 15396, 15717, 15718, 15797, 15798, 15957,
                15958, 16037, 16038, 16041, 16042, 16117, 16118, 16121, 16122, 16197, 16198, 16277,
                16278, 16281, 16282, 16357, 16358, 16361, 16362, 16437, 16438, 16441, 16442, 16517,
                16518, 16521, 16522, 16597, 16598, 16601, 16602, 16677, 16682, 16757, 16762, 16837,
                16842, 16917, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_write_sit_start_south.png",
            &[54, 51],
            &[
                5079, 5081, 5082, 5084, 5157, 5162, 5236, 5237, 5245, 5315, 5324, 5396, 5397, 5405,
                5475, 5476, 5483, 5484, 6430, 6835, 7476, 7556, 7716, 7717, 7877, 7878, 8037, 8038,
                8041, 8042, 8117, 8118, 8121, 8197, 8198, 8201, 8202, 8277, 8278, 8281, 8282, 8357,
                8362, 8437, 8442,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_specialanimation_summer_write_start_south.png",
            &[76, 74],
            &[
                4599, 4601, 4602, 4604, 4677, 4682, 4756, 4757, 4765, 4835, 4844, 4916, 4917, 4925,
                4995, 4996, 5003, 5004, 6355, 6998, 7229, 7396,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_action_east.png",
            &[59, 68, 69, 68, 69, 59, 69],
            &[
                15802, 15804, 15805, 15807, 15882, 15884, 15885, 15887, 15962, 15964, 15965, 15967,
                16042, 16044, 16045, 16047, 16199, 16201, 16202, 16204, 16280, 16282, 16283, 16285,
                16359, 16360, 16368, 16439, 16440, 16448, 16519, 16520, 16528, 16599, 16600, 16608,
                16680, 16682, 16683, 16685, 16756, 16757, 16765, 16837, 16838, 16846, 16919, 16920,
                16928, 16999, 17000, 17008, 17079, 17080, 17088, 17159, 17160, 17168, 17237, 17238,
                17246, 17316, 17317, 17325, 17397, 17398, 17406, 17797, 17798, 17806, 21962, 22042,
                22122, 22202, 22601, 22761, 22915, 22998, 23398, 24190, 24270, 24282, 24350, 24430,
                24442, 24669, 25069, 25148, 25718, 28602, 28603, 29002, 29003, 29162, 29163, 29562,
                29563,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_action_north.png",
            &[14, 18, 18, 18, 18, 14, 14],
            &[
                20208, 20608, 20688, 20849, 20929, 21009, 21089, 21890, 22290, 22370, 22530, 22610,
                22690, 22770, 24686, 24688, 24766, 24768, 24846, 24848, 24926, 24928, 25006, 25008,
                25086, 25088, 25166, 25168, 25243, 25244, 25246, 25320, 25321, 25322, 25323, 25324,
                25326, 25400, 25401, 25402, 25403, 25404, 25406, 25480, 25481, 25482, 25483, 25484,
                25486, 25560, 25561, 25562, 25563, 25564, 25566, 25643, 25644, 25646, 25723, 25724,
                25726, 25800, 25801, 25802, 25803, 25804, 25805, 25806, 25881, 25882, 25883, 25884,
                25885, 25886, 25961, 25962, 25963, 25964, 25965, 25966, 26041, 26042, 26043, 26044,
                26045, 26046, 26121, 26122, 26123, 26124, 26125, 26126, 26200, 26201, 26202, 26203,
                26204, 26205, 26206, 26280, 26281, 26282, 26283, 26284, 26285, 26286, 26361, 26362,
                26363, 26364, 26761, 26762, 26763, 26764, 26841, 26842, 26843, 26844,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_action_south.png",
            &[78, 82, 79, 82, 79, 78, 88],
            &[
                15797, 15802, 15877, 15882, 15957, 15962, 16037, 16042, 16197, 16202, 16277, 16282,
                16355, 16364, 16435, 16444, 16515, 16524, 16595, 16604, 16677, 16682, 16755, 16764,
                16835, 16844, 16915, 16916, 16923, 16924, 16995, 16996, 17003, 17004, 17075, 17076,
                17083, 17084, 17155, 17156, 17163, 17164, 17235, 17244, 17315, 17316, 17323, 17324,
                17395, 17396, 17403, 17404, 17795, 17796, 17803, 17804, 20270, 20350, 20430, 20510,
                21948, 22028, 22108, 22188, 22914, 22925, 22995, 23004, 23395, 23404, 23644, 23645,
                23724, 23725, 23804, 23805, 23884, 23885, 24125, 24126, 24204, 24205, 24281, 24284,
                24285, 24364, 24365, 24441, 24444, 24445, 24525, 24526, 24685, 24686, 24749, 24829,
                24909, 24989, 25085, 25086, 25149, 25229, 25396, 25556, 25629, 25716, 25796, 26196,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_blink_east.png",
            &[70, 74, 70],
            &[
                6759, 6761, 6762, 6764, 6839, 6841, 6842, 6844, 6919, 6921, 6922, 6924, 6996, 6997,
                7005, 7076, 7077, 7085, 7156, 7157, 7165, 7236, 7237, 7245, 7316, 7317, 7325, 7396,
                7397, 7405, 9635, 9715, 9795, 10588, 10668, 10748, 10838, 10918, 10998,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_blink_south.png",
            &[88, 92, 88],
            &[
                6757, 6762, 6837, 6842, 6917, 6922, 6995, 7004, 7075, 7084, 7155, 7164, 7235, 7236,
                7243, 7244, 7315, 7316, 7323, 7324, 7395, 7396, 7403, 7404, 9634, 9645, 9714, 9725,
                9794, 9805, 10589, 10669, 10749, 10836, 10916, 10996,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_drink_east.png",
            &[41, 44, 41],
            &[
                7479, 7481, 7482, 7484, 7561, 7639, 7641, 7642, 7644, 7716, 7717, 7725, 7796, 7802,
                7876, 7877, 7885, 7956, 7957, 7965, 8034, 8042, 8116, 8117, 8125, 8274, 9229,
                11319, 11399, 11479, 11561, 11641, 11721, 11801, 11802, 11804, 11881, 11882, 11884,
                11961, 11962, 11964, 12041, 12042, 12121, 12122, 12201, 12202, 12281, 12361, 12441,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_drink_north.png",
            &[0, 0, 0],
            &[
                8928, 9088, 9489, 9650, 9810, 10210, 10846, 10848, 11006, 11008, 11083, 11084,
                11086, 11166, 11243, 11244, 11246, 11320, 11321, 11322, 11323, 11324, 11325, 11326,
                11400, 11401, 11402, 11403, 11404, 11406, 11408, 11480, 11481, 11482, 11483, 11484,
                11485, 11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_drink_south.png",
            &[57, 57, 57],
            &[
                7477, 7482, 7637, 7642, 7715, 7724, 7797, 7802, 7875, 7884, 7955, 7956, 7963, 7964,
                8035, 8044, 8115, 8116, 8123, 8124, 8275, 8276, 8283, 8284, 8756, 8763, 9390, 9550,
                9709, 10364, 10524, 11557, 11562, 11637, 11642, 11717, 11722, 11797, 11798, 11801,
                11802, 11877, 11878, 11881, 11882, 11957, 11958, 11961, 11962, 12037, 12038, 12041,
                12042, 12117, 12118, 12121, 12122, 12197, 12198, 12201, 12202, 12277, 12278, 12281,
                12282, 12357, 12358, 12361, 12362, 12437, 12438, 12441, 12442, 12517, 12522, 12597,
                12602, 12677, 12682,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_eat_east.png",
            &[39, 39, 35, 40, 45],
            &[
                12199, 12201, 12202, 12204, 12439, 12441, 12442, 12444, 12520, 12522, 12523, 12525,
                12596, 12597, 12605, 12759, 12761, 12762, 12764, 12836, 12837, 12845, 12917, 12918,
                12926, 12996, 12997, 13005, 13079, 13081, 13082, 13084, 13156, 13157, 13165, 13236,
                13237, 13245, 13317, 13318, 13326, 13476, 13477, 13485, 13556, 13557, 13565, 13804,
                13876, 13877, 13885, 14125, 14684, 15469, 17722, 18839, 18919, 18999, 19079, 19159,
                19241, 19321, 19401, 19481, 19561, 19641, 19642, 19644, 19721, 19722, 19724, 19801,
                19802, 19804, 19881, 19882, 19884, 19961, 19962, 19964, 20041, 20042, 20121, 20122,
                20201, 20202, 20281, 20282, 20361, 20362, 20441, 20521, 20601, 20681, 20761,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_eat_north.png",
            &[0, 0, 0],
            &[
                8928, 9088, 9489, 9650, 9810, 10450, 10846, 10848, 11006, 11008, 11083, 11084,
                11086, 11166, 11243, 11244, 11246, 11320, 11321, 11322, 11323, 11324, 11325, 11326,
                11400, 11401, 11402, 11403, 11404, 11406, 11408, 11480, 11481, 11482, 11483, 11484,
                11485, 11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_eat_south.png",
            &[57, 52, 33, 57, 56],
            &[
                12199, 12200, 12437, 12442, 12597, 12602, 12757, 12762, 12835, 12844, 12917, 12922,
                12995, 13004, 13077, 13082, 13155, 13164, 13235, 13236, 13243, 13244, 13315, 13324,
                13395, 13396, 13403, 13404, 13475, 13484, 13555, 13556, 13563, 13564, 13715, 13716,
                13723, 13724, 13875, 13876, 13883, 13884, 14516, 14523, 14676, 14683, 15390, 15630,
                15870, 15950, 17244, 17404, 17555, 17564, 19242, 19322, 19397, 19402, 19477, 19482,
                19557, 19562, 19637, 19638, 19641, 19642, 19721, 19722, 19797, 19798, 19801, 19802,
                19877, 19878, 19881, 19882, 19957, 19958, 19961, 19962, 20037, 20038, 20041, 20042,
                20121, 20122, 20197, 20198, 20201, 20202, 20277, 20278, 20281, 20282, 20357, 20358,
                20361, 20362, 20437, 20438, 20441, 20442, 20517, 20518, 20521, 20522, 20597, 20598,
                20601, 20602, 20677, 20678, 20681, 20682, 20757, 20758, 20761, 20762, 20837, 20842,
                20917, 20922, 20997, 21002, 21077, 21082, 21157, 21162,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_idle_east.png",
            &[70],
            &[
                2279, 2281, 2282, 2284, 2356, 2357, 2365, 2436, 2437, 2445, 3235, 3548, 3638,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_idle_north.png",
            &[14],
            &[
                2928, 3170, 3566, 3568, 3643, 3644, 3646, 3720, 3721, 3722, 3723, 3724, 3725, 3726,
                3801, 3802, 3803, 3804,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_idle_south.png",
            &[88],
            &[
                2277, 2282, 2355, 2364, 2435, 2436, 2443, 2444, 3234, 3245, 3549, 3636,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_kiss_east.png",
            &[69, 63, 78, 63],
            &[
                9162, 9164, 9165, 9167, 9318, 9323, 9400, 9402, 9403, 9405, 9479, 9480, 9488, 9560,
                9562, 9563, 9565, 9635, 9636, 9644, 9717, 9718, 9726, 9799, 9800, 9808, 9877, 9878,
                9886, 9955, 9956, 9964, 10037, 10038, 10046, 10197, 10198, 10206, 11088, 11709,
                11949, 12109, 12677, 12908, 13068, 13154, 13236, 13396, 14279, 14428, 14509, 14599,
                14669, 14757, 16441, 16442, 16601, 16602, 16761, 16762, 16921, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_sit_east.png",
            &[47],
            &[
                2519, 2521, 2522, 2524, 2596, 2597, 2605, 2676, 2677, 2685, 3476, 3799, 3881, 3961,
                3962, 3964, 4041, 4042, 4121,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_sit_north.png",
            &[0],
            &[
                3008, 3250, 3646, 3648, 3723, 3724, 3726, 3800, 3801, 3802, 3803, 3804, 3805, 3806,
                3881, 3882, 3883, 3884,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_sit_south.png",
            &[56],
            &[
                2517, 2522, 2595, 2604, 2675, 2676, 2683, 2684, 3150, 3475, 3484, 3877, 3882, 3957,
                3958, 3961, 3962, 4037, 4038, 4041, 4042, 4117, 4118, 4121, 4122, 4197, 4202,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_sleep_east.png",
            &[57],
            &[
                2359, 2361, 2362, 2364, 2436, 2437, 2445, 2516, 2517, 2525, 3718, 4035, 4121, 4122,
                4202,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_walk_east.png",
            &[70, 71, 70, 71],
            &[
                8999, 9001, 9002, 9004, 9159, 9161, 9162, 9164, 9316, 9317, 9325, 9399, 9401, 9402,
                9404, 9476, 9477, 9485, 9559, 9561, 9562, 9564, 9636, 9637, 9645, 9716, 9717, 9725,
                9796, 9797, 9805, 9876, 9877, 9885, 10036, 10037, 10045, 10196, 10197, 10205,
                12835, 12995, 13235, 13395, 14108, 14268, 14364, 14438, 14598, 14667, 14829, 14838,
                14998, 15316, 16438,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_walk_north.png",
            &[14, 6, 14, 6],
            &[
                11568, 11728, 11967, 12129, 12530, 12690, 12929, 13090, 14126, 14128, 14286, 14288,
                14443, 14444, 14446, 14526, 14528, 14603, 14604, 14606, 14686, 14688, 14760, 14761,
                14762, 14763, 14764, 14765, 14766, 14843, 14844, 14846, 14920, 14921, 14922, 14923,
                14924, 14925, 14926, 15003, 15004, 15006, 15081, 15082, 15083, 15084, 15160, 15161,
                15162, 15163, 15164, 15165, 15241, 15242, 15243, 15244, 15320, 15321, 15322, 15323,
                15324, 15325, 15481, 15482, 15483, 15641, 15642, 15643, 16117, 16282, 16437, 16438,
                16601, 16602, 16757, 16758, 16921, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Summer/spr_npc_seridia_summer_walk_south.png",
            &[88, 74, 88, 75],
            &[
                8997, 9002, 9157, 9162, 9315, 9324, 9397, 9402, 9475, 9484, 9557, 9562, 9635, 9636,
                9643, 9644, 9715, 9724, 9795, 9796, 9803, 9804, 9875, 9884, 10035, 10036, 10043,
                10044, 10195, 10196, 10203, 10204, 12834, 12845, 12994, 13005, 13234, 13245, 13394,
                13405, 13885, 13886, 14033, 14034, 14109, 14205, 14206, 14269, 14353, 14354, 14436,
                14510, 14596, 14836, 14996, 15476,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_action_east.png",
            &[60, 64, 64, 64, 64, 60, 58],
            &[
                15802, 15804, 15805, 15807, 15882, 15884, 15885, 15887, 15962, 15964, 15965, 15967,
                16042, 16044, 16045, 16047, 16199, 16201, 16202, 16204, 16280, 16282, 16283, 16285,
                16359, 16360, 16368, 16439, 16440, 16448, 16519, 16520, 16528, 16599, 16600, 16608,
                16680, 16682, 16683, 16685, 16756, 16757, 16765, 16837, 16838, 16846, 16919, 16920,
                16928, 16999, 17000, 17008, 17079, 17080, 17088, 17159, 17160, 17168, 17237, 17238,
                17246, 17316, 17317, 17325, 17397, 17398, 17406, 17797, 17798, 17806, 19168, 19248,
                19328, 19408, 19727, 19807, 19887, 19967, 21961, 22041, 22121, 22201, 22523, 22601,
                22683, 22761, 23085, 23162, 23245, 23322, 23477, 23484, 23559, 23640, 23720, 23800,
                23880, 23959, 24038, 24042, 24123, 24523, 27958, 27959, 28518, 28519, 29078, 29079,
                29080, 29638, 29639, 30198,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_action_north.png",
            &[17, 21, 21, 21, 21, 17, 17],
            &[
                20208, 20608, 20688, 20849, 20929, 21009, 21089, 21890, 22290, 22370, 22530, 22610,
                22690, 22770, 24686, 24688, 24766, 24768, 24846, 24848, 24926, 24928, 25006, 25008,
                25086, 25088, 25166, 25168, 25243, 25244, 25246, 25320, 25321, 25322, 25323, 25324,
                25326, 25400, 25401, 25402, 25403, 25404, 25406, 25480, 25481, 25482, 25483, 25484,
                25486, 25560, 25561, 25562, 25563, 25564, 25566, 25643, 25644, 25646, 25723, 25724,
                25726, 25798, 25800, 25801, 25802, 25803, 25804, 25805, 25806, 25878, 25881, 25882,
                25883, 25884, 25885, 25886, 25958, 25961, 25962, 25963, 25964, 25965, 25966, 26038,
                26041, 26042, 26043, 26044, 26045, 26046, 26118, 26121, 26122, 26123, 26124, 26125,
                26126, 26198, 26200, 26201, 26202, 26203, 26204, 26205, 26206, 26278, 26280, 26281,
                26282, 26283, 26284, 26285, 26286, 26359, 26361, 26362, 26363, 26364, 26439, 26519,
                26599, 26679, 26759, 26761, 26762, 26763, 26764, 26839, 26841, 26842, 26843, 26844,
                26918, 26920, 27000, 27080, 27160, 27240, 27318, 27320, 27398, 27400,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_action_south.png",
            &[63, 67, 67, 67, 67, 63, 73],
            &[
                15797, 15802, 15877, 15882, 15957, 15962, 16037, 16042, 16197, 16202, 16277, 16282,
                16355, 16364, 16435, 16444, 16515, 16524, 16595, 16604, 16677, 16682, 16755, 16764,
                16835, 16844, 16915, 16916, 16923, 16924, 16995, 16996, 17003, 17004, 17075, 17076,
                17083, 17084, 17155, 17156, 17163, 17164, 17235, 17244, 17315, 17316, 17323, 17324,
                17395, 17396, 17403, 17404, 17795, 17796, 17803, 17804, 20270, 20350, 20430, 20510,
                21948, 22028, 22108, 22188, 23076, 23083, 23084, 23157, 23163, 23164, 23236, 23243,
                23244, 23317, 23323, 23324, 23476, 23483, 23556, 23563, 23641, 23642, 23645, 23721,
                23722, 23725, 23801, 23802, 23805, 23881, 23882, 23885, 23956, 23963, 24035, 24037,
                24041, 24042, 24044, 24117, 24121, 24122, 24124, 24126, 24204, 24205, 24284, 24285,
                24364, 24365, 24444, 24445, 24517, 24521, 24522, 24524, 24526, 24685, 24686, 25085,
                25086, 25396, 25556,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_blink_east.png",
            &[58, 62, 58],
            &[
                6759, 6761, 6762, 6764, 6839, 6841, 6842, 6844, 6919, 6921, 6922, 6924, 6996, 6997,
                7005, 7076, 7077, 7085, 7156, 7157, 7165, 7236, 7237, 7245, 7316, 7317, 7325, 7396,
                7397, 7405, 9877, 9884, 9957, 9964, 10037, 10044, 10118, 10122, 10198, 10202,
                10278, 10282, 11798, 11799, 11878, 11879, 11958, 11959, 12038, 12039, 12118, 12119,
                12198, 12199, 12278, 12279, 12280, 12358, 12359, 12360, 12438, 12439, 12440, 12518,
                12519, 12598, 12599, 12678, 12679, 12758, 12838, 12918,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_blink_south.png",
            &[72, 76, 72],
            &[
                6757, 6762, 6837, 6842, 6917, 6922, 6995, 7004, 7075, 7084, 7155, 7164, 7235, 7236,
                7243, 7244, 7315, 7316, 7323, 7324, 7395, 7396, 7403, 7404, 9876, 9883, 9956, 9963,
                10036, 10043, 10115, 10117, 10121, 10122, 10124, 10195, 10197, 10201, 10202, 10204,
                10275, 10277, 10281, 10282, 10284,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_idle_east.png",
            &[58],
            &[
                2279, 2281, 2282, 2284, 2356, 2357, 2365, 2436, 2437, 2445, 3317, 3324, 3398, 3402,
                3958, 3959, 4038, 4039, 4118, 4119, 4120, 4198, 4199, 4278,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_idle_north.png",
            &[17],
            &[
                2928, 3170, 3566, 3568, 3643, 3644, 3646, 3720, 3721, 3722, 3723, 3724, 3725, 3726,
                3801, 3802, 3803, 3804, 3878,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_idle_south.png",
            &[72],
            &[
                2277, 2282, 2355, 2364, 2435, 2436, 2443, 2444, 3316, 3323, 3395, 3397, 3401, 3402,
                3404,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_kiss_east.png",
            &[68, 47, 71, 47],
            &[
                9162, 9164, 9165, 9167, 9318, 9323, 9400, 9402, 9403, 9405, 9479, 9480, 9488, 9560,
                9562, 9563, 9565, 9635, 9636, 9644, 9717, 9718, 9726, 9799, 9800, 9808, 9877, 9878,
                9886, 9955, 9956, 9964, 10037, 10038, 10046, 10197, 10198, 10206, 11088, 11407,
                11709, 11949, 12109, 12908, 12997, 13068, 13319, 13558, 13718, 13801, 13879, 13883,
                14039, 14043, 15803, 15879, 15963, 16120, 16122, 16280, 16282, 16438, 16440, 16442,
                16598, 16600, 16602, 16757, 16758, 16761, 16917, 16918, 16921, 17077, 17237,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_sit_east.png",
            &[49],
            &[
                2519, 2521, 2522, 2524, 2596, 2597, 2605, 2676, 2677, 2685, 3557, 3884, 3964,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_sit_north.png",
            &[0],
            &[
                3008, 3250, 3646, 3648, 3723, 3724, 3726, 3800, 3801, 3802, 3803, 3804, 3805, 3806,
                3881, 3882, 3883, 3884,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_sit_south.png",
            &[42],
            &[
                2517, 2522, 2595, 2604, 2675, 2676, 2683, 2684, 3150, 3554, 3556, 3563, 3565, 3798,
                3801, 3878, 3880, 3881, 3957, 3958, 3961, 3962, 4037, 4038, 4041, 4042, 4117, 4118,
                4121, 4122, 4197, 4202,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_walk_east.png",
            &[58, 66, 58, 67],
            &[
                8999, 9001, 9002, 9004, 9159, 9161, 9162, 9164, 9316, 9317, 9325, 9399, 9401, 9402,
                9404, 9476, 9477, 9485, 9559, 9561, 9562, 9564, 9636, 9637, 9645, 9716, 9717, 9725,
                9796, 9797, 9805, 9876, 9877, 9885, 10036, 10037, 10045, 10196, 10197, 10205,
                13157, 13164, 13317, 13324, 13478, 13482, 13556, 13638, 13642, 13878, 13882, 14042,
                14044, 14364, 14667, 15718, 15719, 15878, 15879, 16038, 16039, 16198, 16199, 16280,
                16358, 16359, 16360, 16438, 16518, 16519, 16520, 16678, 16679, 16838, 16839, 16998,
                17158,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_walk_north.png",
            &[17, 6, 17, 6],
            &[
                11568, 11728, 11967, 12129, 12530, 12690, 12929, 13090, 14126, 14128, 14286, 14288,
                14443, 14444, 14446, 14526, 14528, 14603, 14604, 14606, 14686, 14688, 14760, 14761,
                14762, 14763, 14764, 14765, 14766, 14843, 14844, 14846, 14920, 14921, 14922, 14923,
                14924, 14925, 14926, 15003, 15004, 15006, 15081, 15082, 15083, 15084, 15160, 15161,
                15162, 15163, 15164, 15165, 15241, 15242, 15243, 15244, 15320, 15321, 15322, 15323,
                15324, 15325, 15398, 15481, 15482, 15483, 15558, 15641, 15642, 15643, 15798, 16117,
                16282, 16437, 16439, 16600, 16602, 16757, 16758, 16921, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Wedding/spr_npc_seridia_wedding_walk_south.png",
            &[72, 62, 72, 64],
            &[
                8997, 9002, 9157, 9162, 9315, 9324, 9397, 9402, 9475, 9484, 9557, 9562, 9635, 9636,
                9643, 9644, 9715, 9724, 9795, 9796, 9803, 9804, 9875, 9884, 10035, 10036, 10043,
                10044, 10195, 10196, 10203, 10204, 13156, 13163, 13316, 13323, 13475, 13477, 13481,
                13482, 13484, 13556, 13563, 13635, 13637, 13641, 13642, 13644, 13716, 13723, 13877,
                13881, 13882, 13884, 13886, 14033, 14035, 14037, 14041, 14042, 14205, 14206, 14353,
                14354,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_meditate_end_south.png",
            &[67, 63, 58],
            &[
                7237, 7242, 7397, 7402, 7475, 7484, 7557, 7562, 7635, 7644, 7716, 7723, 7795, 7804,
                7875, 7876, 7883, 7884, 8036, 8043, 8669, 8990, 9071, 10033, 10035, 10044, 10046,
                10602, 10834, 10838, 10842, 10922, 11076, 11154, 11158, 11162, 11235, 11238, 11244,
                11396, 11474, 11482, 12196, 12197, 12437, 12678, 12918,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_meditate_loop_south.png",
            &[67, 67, 67, 67, 67, 67, 67, 67],
            &[
                18277, 18282, 18357, 18362, 18437, 18442, 18517, 18522, 18597, 18602, 18677, 18682,
                18757, 18762, 18837, 18842, 18915, 18924, 18995, 19004, 19075, 19084, 19155, 19164,
                19235, 19244, 19315, 19324, 19395, 19404, 19475, 19484, 19556, 19563, 19636, 19643,
                19716, 19723, 19796, 19803, 19876, 19883, 19956, 19963, 20036, 20043, 20116, 20123,
                21469, 21550, 21789, 21870, 23388, 23468, 23708, 23788, 26829, 27149, 27230, 27242,
                27322, 27390, 27402, 27482, 27550, 27562, 27642, 27710, 27722, 27802, 27874, 27878,
                27882, 27958, 27962, 28034, 28038, 28042, 28114, 28118, 28122, 28194, 28198, 28202,
                28278, 28282, 28354, 28358, 28362, 28434, 28438, 28442, 28516, 28596, 28676, 28756,
                28836, 28916, 28996, 29076,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_meditate_start_south.png",
            &[58, 63, 67],
            &[
                7237, 7242, 7397, 7402, 7475, 7484, 7557, 7562, 7635, 7644, 7715, 7716, 7723, 7724,
                7795, 7804, 7876, 7883, 8036, 8043, 8829, 8911, 8990, 9873, 9875, 9884, 9886,
                10762, 10922, 10994, 10998, 11002, 11075, 11078, 11084, 11154, 11158, 11162, 11236,
                11314, 11322, 11396, 12036, 12037, 12277, 12518, 12758,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_read_sit_end_south.png",
            &[44, 40, 47],
            &[
                7637, 7642, 7717, 7722, 7797, 7802, 7875, 7884, 7955, 7964, 8035, 8044, 8115, 8116,
                8123, 8124, 8195, 8196, 8203, 8204, 8275, 8276, 8283, 8284, 9550, 9630, 9710,
                10513, 10515, 10524, 10526, 10673, 10686, 10995, 11237, 11242, 11476, 11717, 11722,
                11724, 11957, 11962, 12197, 12202, 12437, 12438, 12441, 12442, 12677, 12682,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_read_sit_loop_south.png",
            &[30, 42, 30, 42],
            &[
                9959, 9961, 9962, 9964, 10115, 10117, 10118, 10120, 10276, 10277, 10285, 10357,
                10362, 10434, 10442, 10443, 10517, 10522, 10596, 10597, 10605, 10675, 10684, 10754,
                10762, 10763, 10835, 10844, 10995, 10996, 11003, 11004, 11155, 11156, 11163, 11164,
                12509, 12910, 13070, 16358, 16361, 16362, 16517, 16518, 16521, 16677, 16682, 16757,
                16762, 16837, 16842, 16917, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_read_sit_start_south.png",
            &[47, 44, 40],
            &[
                7477, 7482, 7715, 7724, 7797, 7802, 7877, 7882, 7955, 7956, 7963, 7964, 8035, 8044,
                8115, 8124, 8275, 8276, 8283, 8284, 8355, 8356, 8363, 8364, 9390, 9710, 9790,
                10353, 10355, 10364, 10366, 10673, 10686, 10835, 11077, 11082, 11316, 11557, 11562,
                11564, 11797, 11802, 12037, 12042, 12277, 12278, 12281, 12282, 12517, 12522,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_spell_cast_end_south.png",
            &[66],
            &[
                2357, 2362, 2435, 2444, 2515, 2516, 2523, 2524, 3313, 3315, 3324, 3326, 3629, 3715,
                3718, 3724, 3802, 3874, 3956, 3964,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_spell_cast_loop_south.png",
            &[78, 79, 79, 79],
            &[
                8679, 8680, 8759, 8760, 8839, 8840, 8919, 8920, 8997, 9002, 9077, 9082, 9157, 9162,
                9237, 9242, 9315, 9324, 9395, 9404, 9475, 9484, 9555, 9564, 9635, 9636, 9643, 9644,
                9715, 9716, 9723, 9724, 9795, 9796, 9803, 9804, 9875, 9876, 9883, 9884, 12588,
                14109, 14189, 14269, 14435, 14444, 14515, 14518, 14524, 14597, 14604, 14675, 14678,
                14684, 14762, 14842, 14914, 15002, 15078, 15084, 15154, 15236, 15243, 15245, 15314,
                15397, 15476, 15484, 15636, 15644,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_spell_cast_start_south.png",
            &[66, 78],
            &[
                4597, 4602, 4677, 4682, 4755, 4764, 4835, 4844, 4915, 4916, 4923, 4924, 4995, 4996,
                5003, 5004, 6355, 6364, 6593, 6595, 6604, 6606, 7149, 7229, 7315, 7318, 7324, 7395,
                7398, 7404, 7482, 7562, 7634, 7714, 7796, 7804, 7876, 7884,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_write_end_south.png",
            &[63, 65],
            &[
                4519, 4521, 4522, 4524, 4676, 4677, 4685, 4757, 4762, 4836, 4837, 4845, 4915, 4924,
                5075, 5076, 5083, 5084, 6275, 7235, 7309, 7478, 7554, 7642, 7716, 7724, 7794, 7956,
                7964,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_write_loop_south.png",
            &[57, 57, 62, 59],
            &[
                8999, 9001, 9002, 9004, 9079, 9081, 9082, 9084, 9159, 9161, 9162, 9164, 9239, 9241,
                9242, 9244, 9316, 9317, 9325, 9396, 9397, 9405, 9476, 9477, 9485, 9556, 9557, 9565,
                9636, 9637, 9645, 9716, 9717, 9725, 9796, 9797, 9805, 9876, 9877, 9885, 12833,
                12835, 12913, 12993, 14435, 15074, 15154, 15234, 15314, 15396, 15404, 15476, 15484,
                15556, 15564, 15636, 15644,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_write_sit_end_south.png",
            &[50, 55],
            &[
                4999, 5001, 5002, 5004, 5156, 5157, 5165, 5237, 5242, 5316, 5317, 5325, 5395, 5404,
                5555, 5556, 5563, 5564, 6510, 6755, 7235, 7397, 7636, 7797, 7957, 8037, 8117, 8122,
                8197, 8198, 8201, 8202, 8277, 8278, 8281, 8282, 8357, 8362, 8437, 8442,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_write_sit_loop_south.png",
            &[45, 45, 50, 47],
            &[
                9959, 9961, 9962, 9964, 10039, 10041, 10042, 10044, 10119, 10121, 10122, 10124,
                10199, 10201, 10202, 10204, 10276, 10277, 10285, 10356, 10357, 10365, 10436, 10437,
                10445, 10516, 10517, 10525, 10596, 10597, 10605, 10676, 10677, 10685, 10756, 10757,
                10765, 10836, 10837, 10845, 13793, 13795, 13873, 13953, 15717, 15797, 15957, 16037,
                16042, 16117, 16122, 16197, 16277, 16282, 16357, 16358, 16361, 16362, 16437, 16438,
                16441, 16442, 16517, 16518, 16521, 16522, 16597, 16598, 16601, 16602, 16677, 16682,
                16757, 16762, 16837, 16842, 16917, 16922,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_write_sit_start_south.png",
            &[55, 50],
            &[
                5079, 5081, 5082, 5084, 5157, 5162, 5236, 5237, 5245, 5315, 5324, 5396, 5397, 5405,
                5475, 5476, 5483, 5484, 6430, 6835, 7315, 7477, 7556, 7717, 7877, 8037, 8042, 8117,
                8197, 8198, 8201, 8202, 8277, 8278, 8281, 8282, 8357, 8362, 8437, 8442,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_specialanimation_winter_write_start_south.png",
            &[65, 63],
            &[
                4599, 4601, 4602, 4604, 4677, 4682, 4756, 4757, 4765, 4835, 4844, 4916, 4917, 4925,
                4995, 4996, 5003, 5004, 6355, 7229, 7315, 7398, 7562, 7634, 7714, 7796, 7804, 7876,
                7884,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_action_east.png",
            &[51, 52, 54, 52, 54, 51, 52],
            &[
                15802, 15804, 15805, 15807, 15882, 15884, 15885, 15887, 15962, 15964, 15965, 15967,
                16042, 16044, 16045, 16047, 16199, 16201, 16202, 16204, 16280, 16282, 16283, 16285,
                16359, 16360, 16368, 16439, 16440, 16448, 16519, 16520, 16528, 16599, 16600, 16608,
                16680, 16682, 16683, 16685, 16756, 16757, 16765, 16837, 16838, 16846, 16919, 16920,
                16928, 16999, 17000, 17008, 17079, 17080, 17088, 17159, 17160, 17168, 17237, 17238,
                17246, 17316, 17317, 17325, 17397, 17398, 17406, 17797, 17798, 17806, 19168, 19248,
                19328, 19408, 21958, 21960, 22038, 22040, 22048, 22118, 22120, 22198, 22200, 22208,
                22365, 22446, 22608, 22768, 22846, 22914, 22916, 22918, 22926, 22996, 22998, 23006,
                23396, 23398, 23406, 23478, 24190, 24270, 24350, 24430, 24669, 24759, 24760, 24839,
                24840, 24919, 24920, 24999, 25000, 25069, 25148, 25318, 25321, 25325, 25398, 25401,
                25405, 25478, 25481, 25485, 25558, 25561, 25565, 25717, 25720, 25797, 26197, 26445,
                26525, 26605, 26685, 26844, 26924, 26999, 27079, 27159, 27239, 27324, 27398, 27478,
                27878, 29242, 29322, 29402, 29482,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_action_north.png",
            &[10, 14, 14, 14, 14, 10, 10],
            &[
                20208, 20608, 20688, 20849, 20929, 21009, 21089, 21890, 22290, 22370, 22530, 22610,
                22690, 22770, 24686, 24688, 24766, 24768, 24846, 24848, 24926, 24928, 25006, 25008,
                25086, 25088, 25166, 25168, 25243, 25244, 25246, 25320, 25321, 25322, 25323, 25324,
                25326, 25400, 25401, 25402, 25403, 25404, 25406, 25480, 25481, 25482, 25483, 25484,
                25486, 25560, 25561, 25562, 25563, 25564, 25566, 25643, 25644, 25646, 25723, 25724,
                25726, 25800, 25801, 25802, 25803, 25804, 25805, 25806, 25881, 25882, 25883, 25884,
                25885, 25886, 25961, 25962, 25963, 25964, 25965, 25966, 26041, 26042, 26043, 26044,
                26045, 26046, 26121, 26122, 26123, 26124, 26125, 26126, 26200, 26201, 26202, 26203,
                26204, 26205, 26206, 26280, 26281, 26282, 26283, 26284, 26285, 26286, 26361, 26362,
                26363, 26364, 26761, 26762, 26763, 26764, 26841, 26842, 26843, 26844,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_action_south.png",
            &[59, 62, 61, 62, 61, 59, 66],
            &[
                15797, 15802, 15877, 15882, 15957, 15962, 16037, 16042, 16197, 16202, 16277, 16282,
                16355, 16364, 16435, 16444, 16515, 16524, 16595, 16604, 16677, 16682, 16755, 16764,
                16835, 16844, 16915, 16916, 16923, 16924, 16995, 16996, 17003, 17004, 17075, 17076,
                17083, 17084, 17155, 17156, 17163, 17164, 17235, 17244, 17315, 17316, 17323, 17324,
                17395, 17396, 17403, 17404, 17795, 17796, 17803, 17804, 20270, 20350, 20430, 20510,
                21948, 22028, 22108, 22188, 22513, 22514, 22516, 22524, 22593, 22594, 22596, 22604,
                22673, 22674, 22676, 22684, 22753, 22754, 22756, 22764, 22913, 22915, 22924, 22926,
                22993, 22995, 23004, 23006, 23393, 23395, 23404, 23406, 23645, 23725, 23805, 23885,
                24126, 24204, 24205, 24284, 24285, 24364, 24365, 24444, 24445, 24526, 24685, 24686,
                24749, 24829, 24909, 24989, 25085, 25086, 25149, 25229, 25322, 25324, 25396, 25402,
                25404, 25482, 25484, 25556, 25562, 25564, 25629, 25715, 25718, 25724, 25798, 25804,
                25875, 25955, 26035, 26115, 26198, 26204, 26282, 26362, 26438, 26444, 26518, 26524,
                26598, 26604, 26678, 26684, 26762, 26834, 26914, 26997, 27077, 27157, 27237, 27314,
                27396, 27404, 27476, 27484, 27876, 27884,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_blink_east.png",
            &[52, 56, 52],
            &[
                6759, 6761, 6762, 6764, 6839, 6841, 6842, 6844, 6919, 6921, 6922, 6924, 6996, 6997,
                7005, 7076, 7077, 7085, 7156, 7157, 7165, 7236, 7237, 7245, 7316, 7317, 7325, 7396,
                7397, 7405, 9405, 9485, 9565, 9634, 9636, 9638, 9646, 9714, 9716, 9718, 9726, 9794,
                9796, 9798, 9806, 9878, 9958, 10038, 10588, 10668, 10748, 10837, 10840, 10917,
                10920, 10997, 11000, 11324, 11404, 11484, 11558, 11638, 11718,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_blink_south.png",
            &[66, 70, 66],
            &[
                6757, 6762, 6837, 6842, 6917, 6922, 6995, 7004, 7075, 7084, 7155, 7164, 7235, 7236,
                7243, 7244, 7315, 7316, 7323, 7324, 7395, 7396, 7403, 7404, 9633, 9635, 9644, 9646,
                9713, 9715, 9724, 9726, 9793, 9795, 9804, 9806, 10589, 10669, 10749, 10835, 10838,
                10844, 10915, 10918, 10924, 10995, 10998, 11004, 11082, 11162, 11242, 11314, 11394,
                11474, 11556, 11564, 11636, 11644, 11716, 11724,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_drink_east.png",
            &[45, 46, 45],
            &[
                7479, 7481, 7482, 7484, 7561, 7639, 7641, 7642, 7644, 7716, 7717, 7725, 7796, 7802,
                7876, 7877, 7885, 7956, 7957, 7965, 8034, 8042, 8116, 8117, 8125, 8274, 8523, 9229,
                10125, 10205, 10285, 10355, 10357, 10366, 10433, 10435, 10515, 10517, 10526, 10837,
                10997, 11079, 11159, 11239, 11318, 11398, 11478,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_drink_north.png",
            &[0, 0, 0],
            &[
                8928, 9088, 9489, 9650, 9810, 10210, 10846, 10848, 11006, 11008, 11083, 11084,
                11086, 11166, 11243, 11244, 11246, 11320, 11321, 11322, 11323, 11324, 11325, 11326,
                11400, 11401, 11402, 11403, 11404, 11406, 11408, 11480, 11481, 11482, 11483, 11484,
                11485, 11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_drink_south.png",
            &[51, 50, 51],
            &[
                7477, 7482, 7637, 7642, 7715, 7724, 7797, 7802, 7875, 7884, 7955, 7956, 7963, 7964,
                8035, 8044, 8115, 8116, 8123, 8124, 8275, 8276, 8283, 8284, 8756, 8763, 9390, 9550,
                9709, 10364, 10366, 10524, 10526, 10673, 10684, 10686, 10915, 11077, 11082, 11157,
                11162, 11237, 11242, 11316, 11396, 11476, 11557, 11562, 11564, 11637, 11642, 11644,
                11717, 11722, 11724, 11797, 11802, 11877, 11882, 11957, 11962, 12037, 12042, 12117,
                12122, 12197, 12202, 12277, 12278, 12281, 12282, 12357, 12358, 12361, 12362, 12437,
                12438, 12441, 12442, 12517, 12522, 12597, 12602, 12677, 12682,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_eat_east.png",
            &[44, 42, 35, 45, 48],
            &[
                12199, 12201, 12202, 12204, 12439, 12441, 12442, 12444, 12520, 12522, 12523, 12525,
                12596, 12597, 12605, 12759, 12761, 12762, 12764, 12836, 12837, 12845, 12917, 12918,
                12926, 12996, 12997, 13005, 13079, 13081, 13082, 13084, 13156, 13157, 13165, 13236,
                13237, 13245, 13317, 13318, 13326, 13476, 13477, 13485, 13556, 13557, 13565, 13804,
                13876, 13877, 13885, 14125, 14684, 15469, 16916, 16918, 16995, 16997, 17005, 17165,
                17235, 17237, 17246, 17327, 17406, 17555, 17557, 17566, 17875, 17877, 17886, 18439,
                18519, 18599, 18679, 18759, 18838, 18918, 18998, 19078, 19158,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_eat_north.png",
            &[0, 0, 0],
            &[
                8928, 9088, 9489, 9650, 9810, 10450, 10846, 10848, 11006, 11008, 11083, 11084,
                11086, 11166, 11243, 11244, 11246, 11320, 11321, 11322, 11323, 11324, 11325, 11326,
                11400, 11401, 11402, 11403, 11404, 11406, 11408, 11480, 11481, 11482, 11483, 11484,
                11485, 11486, 11561, 11562, 11563, 11564, 11641, 11642, 11643, 11644, 11645, 11646,
                11721, 11722, 11723, 11724,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_eat_south.png",
            &[51, 47, 31, 55, 49],
            &[
                12199, 12200, 12437, 12442, 12597, 12602, 12757, 12762, 12835, 12844, 12917, 12922,
                12995, 13004, 13077, 13082, 13155, 13164, 13235, 13236, 13243, 13244, 13315, 13324,
                13395, 13396, 13403, 13404, 13475, 13484, 13555, 13556, 13563, 13564, 13715, 13716,
                13723, 13724, 13875, 13876, 13883, 13884, 14516, 14523, 14676, 14683, 15390, 15630,
                15870, 15950, 16993, 17233, 17235, 17244, 17246, 17404, 17406, 17553, 17555, 17564,
                17566, 17637, 17714, 17716, 17724, 17726, 17873, 17884, 17886, 18195, 18355, 18442,
                18522, 18597, 18602, 18677, 18682, 18757, 18762, 18996, 19076, 19156, 19242, 19244,
                19322, 19324, 19397, 19402, 19404, 19477, 19482, 19484, 19557, 19562, 19564, 19637,
                19642, 19722, 19797, 19802, 19877, 19882, 19957, 19962, 20037, 20042, 20122, 20197,
                20202, 20277, 20282, 20357, 20362, 20437, 20438, 20441, 20442, 20517, 20518, 20521,
                20522, 20597, 20598, 20601, 20602, 20677, 20678, 20681, 20682, 20757, 20758, 20761,
                20762, 20837, 20842, 20917, 20922, 20997, 21002, 21077, 21082, 21157, 21162,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_idle_east.png",
            &[52],
            &[
                2279, 2281, 2282, 2284, 2356, 2357, 2365, 2436, 2437, 2445, 3165, 3234, 3236, 3238,
                3246, 3318, 3548, 3637, 3640, 3804, 3878,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_idle_north.png",
            &[10],
            &[
                2928, 3170, 3566, 3568, 3643, 3644, 3646, 3720, 3721, 3722, 3723, 3724, 3725, 3726,
                3801, 3802, 3803, 3804,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_idle_south.png",
            &[66],
            &[
                2277, 2282, 2355, 2364, 2435, 2436, 2443, 2444, 3233, 3235, 3244, 3246, 3549, 3635,
                3638, 3644, 3722, 3794, 3876, 3884,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_kiss_east.png",
            &[49, 44, 58, 44],
            &[
                9162, 9164, 9165, 9167, 9318, 9323, 9400, 9402, 9403, 9405, 9479, 9480, 9488, 9560,
                9562, 9563, 9565, 9635, 9636, 9644, 9717, 9718, 9726, 9799, 9800, 9808, 9877, 9878,
                9886, 9955, 9956, 9964, 10037, 10038, 10046, 10197, 10198, 10206, 11088, 11709,
                11949, 12109, 12677, 12679, 12844, 12908, 13001, 13008, 13068, 13153, 13155, 13157,
                13165, 13235, 13237, 13239, 13246, 13320, 13323, 13395, 13397, 13399, 13406, 13559,
                13719, 14428, 14434, 14435, 14436, 14509, 14599, 14602, 14604, 14669, 14759, 14838,
                14841, 14844, 14925, 14998, 15001, 15004, 15084, 15165, 15245, 15325, 15403, 15560,
                15565, 15717, 15799, 15959, 16438, 16598, 16757, 16758, 16917, 16918, 17077, 17237,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_sit_east.png",
            &[51],
            &[
                2519, 2521, 2522, 2524, 2596, 2597, 2605, 2676, 2677, 2685, 3405, 3474, 3476, 3478,
                3486, 3719, 3798,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_sit_north.png",
            &[0],
            &[
                3008, 3250, 3646, 3648, 3723, 3724, 3726, 3800, 3801, 3802, 3803, 3804, 3805, 3806,
                3881, 3882, 3883, 3884,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_sit_south.png",
            &[49],
            &[
                2517, 2522, 2595, 2604, 2675, 2676, 2683, 2684, 3150, 3473, 3475, 3484, 3486, 3635,
                3717, 3722, 3796, 3877, 3882, 3884, 3957, 3962, 4037, 4042, 4117, 4118, 4121, 4122,
                4197, 4202,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_sleep_east.png",
            &[47],
            &[
                2359, 2361, 2362, 2364, 2436, 2437, 2445, 2516, 2517, 2525, 3235, 3237, 3245, 3326,
                3398, 3717, 3720, 3884, 3958,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_walk_east.png",
            &[52, 59, 52, 57],
            &[
                8999, 9001, 9002, 9004, 9159, 9161, 9162, 9164, 9316, 9317, 9325, 9399, 9401, 9402,
                9404, 9476, 9477, 9485, 9559, 9561, 9562, 9564, 9636, 9637, 9645, 9716, 9717, 9725,
                9796, 9797, 9805, 9876, 9877, 9885, 10036, 10037, 10045, 10196, 10197, 10205,
                12525, 12685, 12834, 12836, 12838, 12846, 12925, 12994, 12996, 12998, 13006, 13085,
                13158, 13236, 13238, 13246, 13318, 13394, 13396, 13398, 13406, 13558, 14108, 14268,
                14437, 14440, 14520, 14597, 14600, 14667, 14829, 14837, 14998, 15003, 15084, 15164,
                15244, 15319, 15325, 15398, 15479, 15485, 15558, 15635, 15644, 15798, 15956, 16437,
                16438,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_walk_north.png",
            &[10, 6, 10, 6],
            &[
                11568, 11728, 11967, 12129, 12530, 12690, 12929, 13090, 14126, 14128, 14286, 14288,
                14443, 14444, 14446, 14526, 14528, 14603, 14604, 14606, 14686, 14688, 14760, 14761,
                14762, 14763, 14764, 14765, 14766, 14843, 14844, 14846, 14920, 14921, 14922, 14923,
                14924, 14925, 14926, 15003, 15004, 15006, 15081, 15082, 15083, 15084, 15160, 15161,
                15162, 15163, 15164, 15165, 15241, 15242, 15243, 15244, 15320, 15321, 15322, 15323,
                15324, 15325, 15481, 15482, 15483, 15641, 15642, 15643,
            ],
        ),
        (
            "assets/animations/NPCs/Seridia/Sprites/Winter/spr_npc_seridia_winter_walk_south.png",
            &[66, 60, 66, 58],
            &[
                8997, 9002, 9157, 9162, 9315, 9324, 9397, 9402, 9475, 9484, 9557, 9562, 9635, 9636,
                9643, 9644, 9715, 9724, 9795, 9796, 9803, 9804, 9875, 9884, 10035, 10036, 10043,
                10044, 10195, 10196, 10203, 10204, 12833, 12835, 12844, 12846, 12993, 12995, 13004,
                13006, 13233, 13235, 13244, 13246, 13393, 13395, 13404, 13406, 14109, 14205, 14206,
                14269, 14353, 14354, 14435, 14438, 14444, 14510, 14595, 14598, 14604, 14762, 14837,
                14922, 15002, 15074, 15162, 15234, 15315, 15396, 15404, 15483, 15485, 15556, 15564,
                15638, 15644, 15795, 15957,
            ],
        ),
    ];
    let mut selected = 0;
    let mut empty = 0;
    for (a, counts, protected) in cases {
        let src = image::open(original.join(a)).unwrap().to_rgba8();
        assert_eq!(src.dimensions(), (counts.len() as u32 * 80, 80));
        let mut observed = vec![0; counts.len()];
        for (x, y, p) in src.enumerate_pixels() {
            let c = (u32::from(p[0]) << 16) | (u32::from(p[1]) << 8) | u32::from(p[2]);
            if p[3] == 255
                && WORLD_ROLES.iter().any(|(s, _, _)| *s == c)
                && !protected.contains(&(y * src.width() + x))
            {
                observed[x as usize / 80] += 1;
                assert!((25..=53).contains(&(x % 80)) && (23..=63).contains(&y));
            }
        }
        assert_eq!(&observed, counts, "source materials {a}");
        selected += counts.iter().sum::<usize>();
        empty += counts.iter().filter(|n| **n == 0).count();
        for choice in original_set["presets"].as_array().unwrap() {
            let id = choice["id"].as_str().unwrap();
            let after = image::open(output.join("variants").join(id).join(a))
                .unwrap()
                .to_rgba8();
            assert_eq!(src.dimensions(), after.dimensions());
            let mut landmarks: Vec<(u32, u32, u32, bool)> = vec![];
            if a.ends_with("_idle_south.png") && !a.contains("Flashback") {
                landmarks.extend([
                    (39, 28, 0x594240, true),
                    (38, 31, 0x8A716A, true),
                    (39, 32, 0xA69084, true),
                    (39, 33, 0xC1AFA5, true),
                    (35, 34, 0x594240, true),
                    (32, 44, 0xC1AFA5, true),
                    (33, 44, 0x594240, true),
                    (39, 37, 0x8A716A, true),
                    (39, 38, 0xA69084, true),
                    (35, 30, 0xC3AA9B, false),
                    (36, 30, 0x8A716A, false),
                    (37, 28, 0x8A716A, false),
                    (37, 34, 0xECF0E9, false),
                    (36, 53, 0xC7496B, false),
                ]);
            }
            if a.ends_with("_idle_south.png") && !a.contains("Flashback") && !a.contains("Wedding")
            {
                landmarks.push((29, 44, 0x8A716A, false));
            }
            if a.ends_with("flashback_priestess_idle_south.png") {
                landmarks.extend([
                    (39, 33, 0xFCD9B3, true),
                    (39, 28, 0xFCD69E, false),
                    (35, 34, 0xFCD69E, false),
                    (39, 38, 0xF5F5F5, false),
                    (39, 37, 0xA1121D, false),
                    (39, 43, 0xBD2E4D, false),
                ]);
            }
            if a.ends_with("specialanimation_autumn_meditate_loop_south.png") {
                landmarks.extend([
                    (39, 33, 0xB990A9, true),
                    (39, 35, 0xCCA6C0, true),
                    (39, 29, 0x835979, true),
                    (35, 31, 0xCDA2B9, false),
                    (37, 29, 0xA87C99, false),
                ]);
            }
            for (x, y, c, skin) in landmarks {
                assert_eq!(
                    src.get_pixel(x, y).0,
                    rgba(c),
                    "source landmark {a} {x},{y}"
                );
                assert_eq!(
                    after.get_pixel(x, y).0,
                    if skin { target(c, choice) } else { rgba(c) },
                    "material landmark {a} {id} {x},{y}"
                );
            }
            for (x, y, p) in src.enumerate_pixels() {
                let c = (u32::from(p[0]) << 16) | (u32::from(p[1]) << 8) | u32::from(p[2]);
                let is_skin = p[3] == 255
                    && WORLD_ROLES.iter().any(|(s, _, _)| *s == c)
                    && !protected.contains(&(y * src.width() + x));
                let expected = if is_skin { target(c, choice) } else { p.0 };
                assert_eq!(
                    after.get_pixel(x, y).0,
                    expected,
                    "material pixel {a} {id} {x},{y}"
                );
            }
            assert_eq!(
                fs::read(original.join(a.replace(".png", ".meta.toml"))).unwrap(),
                fs::read(
                    output
                        .join("variants")
                        .join(id)
                        .join(a.replace(".png", ".meta.toml"))
                )
                .unwrap()
            );
        }
    }
    assert_eq!(selected, 32254);
    assert_eq!(empty, 33);
    let portraits = read_json(root.join("palettes/profiles/seridia-portraits.json"));
    let prior = temp.path().join("prior");
    fs::create_dir(&prior).unwrap();
    for region in portraits["regions"].as_array().unwrap() {
        let asset = region["asset"].as_str().unwrap();
        for asset in [asset.to_string(), asset.replace(".png", ".meta.toml")] {
            let to = prior.join(&asset);
            fs::create_dir_all(to.parent().unwrap()).unwrap();
            fs::copy(original.join(&asset), to).unwrap();
        }
    }
    let prior_output = temp.path().join("prior-bundle");
    let result = Command::new(env!("CARGO_BIN_EXE_mistria-palette"))
        .args(["build-presets", "--original"])
        .arg(prior)
        .arg("--presets")
        .arg(root.join("palettes/sets/seridia-portraits-trial.json"))
        .arg("--output")
        .arg(&prior_output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    for choice in ["blue", "npc_adeline", "npc_hayden", "npc_ryis"] {
        for region in portraits["regions"].as_array().unwrap() {
            let asset = region["asset"].as_str().unwrap();
            for asset in [asset.to_string(), asset.replace(".png", ".meta.toml")] {
                let path = Path::new("variants").join(choice).join(asset);
                assert_eq!(
                    fs::read(output.join(&path)).unwrap(),
                    fs::read(prior_output.join(path)).unwrap()
                );
            }
        }
    }
}
