use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
        
    },
    smash_script::*,
    smashline::*,
    smash::app::utility

};

// SHEIK

// FORWARD SMASH

#[acmd_script( agent = "sheik", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn sheik_attacks4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 5.0, 35, 100, 47, 0, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 5.0, 45, 100, 47, 0, 3.0, -0.7, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 5.0, 80, 100, 30, 0, 4.0, 3.2, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }

    for n in 1..8{
        wait(fighter.lua_state_agent, 1.0);
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD){{
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                CancelModule::enable_cancel(fighter.module_accessor);
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_CATCH, true);
                }
            }
        }
    }

    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 8.0, 361, 109, 0, 60, 4.8, -1.6, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 8.0, 361, 109, 0, 60, 4.6, 5.6, -0.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}


// JOKER DTILT PORT

#[acmd_script( agent = "sheik", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn sheik_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        macros::FT_MOTION_RATE(fighter, 0.7);
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 75, 66, 0, 75, 2.6, -4.0, 0.0, 1.5, Some(5.0), Some(0.0), Some(1.5), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }

    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 82, 66, 0, 75, 2.6, -4.0, 0.0, 1.5, Some(5.0), Some(0.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }

    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.0, 85, 66, 0, 75, 2.6, -4.0, 0.0, 1.5, Some(5.0), Some(0.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }

    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        JostleModule::set_status(fighter.module_accessor, true);
    }
}


#[acmd_script( agent = "sheik", script = "expression_attacklw3", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_attacklw3(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 5);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

#[acmd_script( agent = "sheik", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -1.5, 0, 0, 0, 0.8, 0, 0, 1.5, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 1);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 6, 0, -2.5, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, -2.5, 0, 0, 0, 0.45, 0, 0, 1.5, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7, 0, -2.5, 0, 0, 0, 0.35, 0, 0, 1.5, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 5, 0, -2.5, 0, 0, 0, 0.35, 0, 0, 1.5, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "sheik", script = "sound_attacklw3", category = ACMD_SOUND, low_priority )]
unsafe fn sound_attacklw3(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::SET_PLAY_INHIVIT(fighter, Hash40::new("se_jack_squat"), 60);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_jack_rnd_attack_m"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_jack_attackhard_l01"));
    }
}



// OPFF

#[fighter_frame( agent = FIGHTER_KIND_SHEIK )]
fn sheik_frame(fighter: &mut L2CFighterCommon) {
    unsafe{

        let status = StatusModule::status_kind(fighter.module_accessor);

        if status == *FIGHTER_STATUS_KIND_ATTACK_LW3 {            
            GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sheik_attacklw3,
        sheik_attacks4,

        expression_attacklw3,
        
        effect_attacklw3,

        sound_attacklw3
    );

 
    smashline::install_agent_frames!(
        sheik_frame
    );

}
