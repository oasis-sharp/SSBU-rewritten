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
        macros::ATTACK(fighter, 1, 0, Hash40::new("tail3"), 4.0, 348, 15, 0, 70, 3.8, 1.5, -0.2, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
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

// OPFF SCRIPT

#[fighter_frame( agent = FIGHTER_KIND_PICHU )]
fn pichu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);

    }  
}


pub fn install() {
    smashline::install_acmd_scripts!(
        pichu_specialhi1,
        pichu_specialairhi1,
        pichu_specialhi2,
        pichu_specialairhi2,
        pichu_attackairhi
    );

 
    smashline::install_agent_frames!(
        pichu_frame
    );

}
