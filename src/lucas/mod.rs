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

#[acmd_script( agent = "lucas", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn lucas_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("handr"), 2.5, 366, 100, 30, 0, 5.5, 4.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 2.5, 366, 100, 30, 0, 3.5, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 5.0, 73, 180, 0, 40, 6.5, 3.0, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 5.0, 73, 180, 0, 40, 3.5, -3.0, 0.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 0.85);
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "lucas", script = "effect_attackairhi", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("handr"), 1.5, 5.5, 0, 0, -80, -105, 0.7, true);
        //macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1.5, 7.5, 0, 0, -80, -105, 0.7, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true, 0.7);
    }
}

// DASH ATTACK


#[acmd_script( agent = "lucas", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn lucas_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 0.9);
    frame(fighter.lua_state_agent, 17.0);

    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 75, 76, 0, 60, 2.5, 0.0, 5.5, 1.2, Some(0.0), Some(5.5), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 46, 81, 0, 60, 5.8, 0.0, 5.5, 12.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}


// DOWN AIR

#[acmd_script( agent = "lucas", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn lucas_attackairlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 10.0);
    for _ in 0..3 {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 367, 10, 0, 42, 4.2, 0.0, -4.0, 0.3, None, None, None, 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.5, 367, 100, 20, 0, 3.2, 0.0, 1.0, 0.3, None, None, None, 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 5.0);
}
if macros::is_excute(fighter) {
    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 270, 110, 0, 10, 5.5, 0.0, -4.0, 0.3, None, None, None, 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
    macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 130, 0, 10, 3.5, 0.0, 1.0, 0.3, None, None, None, 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
}
wait(fighter.lua_state_agent, 2.0);
if macros::is_excute(fighter) {
    AttackModule::clear_all(fighter.module_accessor);
}
frame(fighter.lua_state_agent, 47.0);
if macros::is_excute(fighter) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
}
}

// DOWN SPECIAL

#[acmd_script( agent = "lucas", script = "game_specialairlwstart", category = ACMD_GAME, low_priority )]
unsafe fn lucas_specialairlwstart(fighter: &mut L2CAgentBase) {
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 25, 98, 0, 40, 3.5, 0.0, 6.3, 7.0, Some(0.0), Some(6.3), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    }
}

#[acmd_script( agent = "lucas", script = "game_speciallwstart", category = ACMD_GAME, low_priority )]
unsafe fn lucas_speciallwstart(fighter: &mut L2CAgentBase) {
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 25, 98, 0, 40, 3.5, 0.0, 6.3, 7.0, Some(0.0), Some(6.3), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
    }
}

// DOWN THROW

#[acmd_script( agent = "lucas", script = "game_throwlw", category = ACMD_GAME, low_priority )]
unsafe fn lucas_throwlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::FT_LEAVE_NEAR_OTTOTTO(fighter, -2.5, 2.5);
        macros::FT_LEAVE_NEAR_OTTOTTO(fighter, 1.5, 1.5);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 70, 80, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 6.0);
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 361, 100, 0, 0, 4.3, 2.0, 1.8, 0.0, Some(-2.0), Some(1.8), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
            AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        }
        wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }

    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 100, 0, 0, 6.0, 2.0, 2.2, 0.0, Some(-2.0), Some(2.2), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_PSI);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 2, 0);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
}

#[acmd_script( agent = "lucas", script = "effect_throwlw", category = ACMD_EFFECT, low_priority )]
unsafe fn effect_throwlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    for _ in 0..3 {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("lucas_psi_atK"), Hash40::new("top"), 7, 4, 0, 0, 0, 0, 1, 6, 4, 13, 0, 0, 0, true);
    }
    wait(fighter.lua_state_agent, 5.0);
}
frame(fighter.lua_state_agent, 26.0);
if macros::is_excute(fighter) {
    macros::EFFECT(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0, 0.5, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
}
frame(fighter.lua_state_agent, 27.0);
if macros::is_excute(fighter) {
    macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
    macros::LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
}
}


#[acmd_script( agent = "lucas", script = "expression_throwlw", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_throwlw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackss"), 5);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 0);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 6);
    }
    frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
    }
}


#[acmd_script( agent = "lucas", script = "sound_throwlw", category = ACMD_SOUND, low_priority )]
unsafe fn sound_throwlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackair_f02"));
        macros::PLAY_SE(fighter, Hash40::new("14"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackair_f01"));
        }
}


// OPFF

#[fighter_frame( agent = FIGHTER_KIND_LUCAS )]
fn lucas_frame(fighter: &mut L2CFighterCommon) {
    unsafe{
        let status = StatusModule::status_kind(fighter.module_accessor);
        if status == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_HIT || status == *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_END {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
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
        lucas_attackairhi,
        lucas_attackdash,
        lucas_attackairlw,
        lucas_specialairlwstart,
        lucas_speciallwstart,

        effect_attackairhi,
        effect_throwlw,

        lucas_throwlw,

        expression_throwlw,

        sound_throwlw
    );

 
    smashline::install_agent_frames!(
        lucas_frame
    );

}
