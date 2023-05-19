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


// PUMMEL

#[acmd_script( agent = "pichu", script = "game_catchattack", category = ACMD_GAME, low_priority )]
unsafe fn game_catchattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 0.1);
        macros::FT_MOTION_RATE(fighter, 0.4);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.4, 80, 100, 30, 0, 4.7, 0.0, 8.2, 8.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// UP AIR

#[acmd_script( agent = "pichu", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn pichu_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("tail1"), 4.0, 80, 70, 0, 70, 3.4, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("tail3"), 4.0, 80, 70, 0, 70, 3.8, 1.5, -0.2, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        macros::ATTACK(fighter, 1, 0, Hash40::new("tail3"), 4.0, 348, 12, 0, 50, 3.8, 1.5, -0.2, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);

    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


// UP SPECIAL

#[acmd_script( agent = "pichu", script = "game_specialhi1", category = ACMD_GAME, low_priority )]
unsafe fn pichu_specialhi1(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 363, 100, 0, 20, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "pichu", script = "game_specialairhi1", category = ACMD_GAME, low_priority )]
unsafe fn pichu_specialairhi1(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 363, 100, 0, 20, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        JostleModule::set_status(fighter.module_accessor, false);
    }
}


#[acmd_script( agent = "pichu", script = "game_specialhi2", category = ACMD_GAME, low_priority )]
unsafe fn pichu_specialhi2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 363, 100, 0, 20, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "pichu", script = "game_specialairhi2", category = ACMD_GAME, low_priority )]
unsafe fn pichu_specialairhi2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 363, 100, 0, 20, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 10, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

// SHINE

#[acmd_script( agent = "pichu", script = "game_specialairn", category = ACMD_GAME, low_priority )]
unsafe fn pichu_specialairn(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.2);
        ControlModule::clear_command(fighter.module_accessor, true);
    }

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 88, 112, 0, 30, 8.0, 0.0, 4.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        MotionModule::set_frame(fighter.module_accessor, 32.0,true);
        macros::FT_MOTION_RATE(fighter, 0.8);
    }

    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pichu", script = "effect_specialairn", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialairn(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);

     if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_kaminari_hit2"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 1.15, true);
        macros::LAST_EFFECT_SET_SCALE_W(fighter, 0.7, 0.7, 0.7);
    }
    
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 0, 0, 0, 0);
        macros::BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::BURN_COLOR_NORMAL(fighter);
        macros::COL_NORMAL(fighter);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_kaminari_hit2"), false, true);
    }

}


#[acmd_script( agent = "pichu", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn pichu_specialn(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.2);
        ControlModule::clear_command(fighter.module_accessor, true);
    }

    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 88, 112, 0, 30, 8.0, 0.0, 4.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        MotionModule::set_frame(fighter.module_accessor, 32.0,true);
        macros::FT_MOTION_RATE(fighter, 0.8);
    }

    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter){
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pichu", script = "effect_specialn", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_specialn(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 1.0);

     if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_kaminari_hit2"), Hash40::new("top"), 0, 0, 0, 0, 90, 0, 1.15, true);
        macros::LAST_EFFECT_SET_SCALE_W(fighter, 0.7, 0.7, 0.7);
    }
    
    if macros::is_excute(fighter) {
        macros::FLASH(fighter, 0, 0, 0, 0);
        macros::BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
        macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::BURN_COLOR_NORMAL(fighter);
        macros::COL_NORMAL(fighter);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_kaminari_hit2"), false, true);
    }

}

#[acmd_script( agent = "pichu", script = "expression_specialairn", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_specialairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "pichu", script = "expression_specialn", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_specialn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

// OPFF SCRIPT

#[fighter_frame( agent = FIGHTER_KIND_PICHU )]
fn pichu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);

        if status == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK ||
        status == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END {
            GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }

        if status == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                }; 
            
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                };
            }
        }
    }  
}


pub fn install() {
    smashline::install_acmd_scripts!(
        pichu_specialhi1,
        pichu_specialairhi1,
        pichu_specialhi2,
        pichu_specialairhi2,
        pichu_attackairhi,

        pichu_specialairn,
        pichu_specialn,

        effect_specialairn,
        effect_specialn,

        expression_specialairn,
        expression_specialn
    );

 
    smashline::install_agent_frames!(
        pichu_frame
    );

}
