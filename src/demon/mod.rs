use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "demon", script = "game_squat", category = ACMD_GAME, low_priority )]
unsafe fn demon_squat(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        demon_squat
    );
}
