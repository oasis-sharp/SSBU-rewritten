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

use smash::hash40;

#[fighter_frame_callback]
pub fn llpc(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);  
		let sticky = ControlModule::get_stick_y(boma);	
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma,smash::phx::Hash40::new_raw(MotionModule::motion_kind(boma)),false) as f32;
		let frame = MotionModule::frame(boma);

        if ([hash40("landing_heavy"), hash40("landing_air_f"), hash40("landing_air_b"), hash40("landing_air_hi"), hash40("landing_air_n")].contains(&MotionModule::motion_kind(boma))) {
			if GroundModule::is_passable_ground(fighter.module_accessor) && frame/cancel_frame >= (1.0/6.0){
                if sticky <= -0.6875 && ((ControlModule::get_flick_y(boma) >= 3 && ControlModule::get_flick_y(boma) < 20) || sticky <= -1.0) {
					if (
						(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) == 0 &&
						(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 &&
						(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) == 0 &&
						(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) == 0 &&
						(ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP))
					) {
						StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
					};
                };
            }
		};
    };
}	

#[fighter_frame_callback]
pub fn daircancel(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = fighter.module_accessor;
        let situation = StatusModule::situation_kind(boma);
        let status = smash::app::lua_bind::StatusModule::status_kind(boma);

        if status == *FIGHTER_STATUS_KIND_ATTACK_AIR{
            if MotionModule::motion_kind(boma) == hash40("attack_air_lw") { 
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD){

                    if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 || 
                    (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 ||
                        (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
                            if(ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
                            }
                        }   

                    if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
                    }
                    if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                    }
                    if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
                    }
                    if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
                    }


                }
            }
        }
    }
}


#[fighter_frame_callback]
pub fn hitfall_upair(fighter : &mut L2CFighterCommon) {
    unsafe {
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        let status = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let fighter_kind = smash::app::utility::get_kind(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent));

        if status == *FIGHTER_STATUS_KIND_ATTACK_AIR && MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi")
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && stick_y < -0.66
        && [*FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_WOLF, *FIGHTER_KIND_SHEIK].contains(&fighter_kind)
         {
            WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("bayonetta_muzzleflash"), Hash40::new("top"), 4, 6, 4, 17, 45, 180, 0.9, true);
        };
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
		llpc,
        daircancel,
        hitfall_upair
	);
 
    smashline::install_agent_frames!(

    );
}
