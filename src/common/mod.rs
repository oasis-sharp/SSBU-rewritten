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

static mut WAVEDASH: bool = false;
static mut TEST: f32 = 0.0;
static mut CAN_WD_BUFFER: bool = true;

#[fighter_frame_callback]
pub fn llpc(fighter : &mut L2CFighterCommon) {
    unsafe {
		let sticky = ControlModule::get_stick_y(fighter.module_accessor);	
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,smash::phx::Hash40::new_raw(MotionModule::motion_kind(fighter.module_accessor)),false) as f32;
		let frame = MotionModule::frame(fighter.module_accessor);

        if ([hash40("landing_heavy"), hash40("landing_air_f"), hash40("landing_air_b"), hash40("landing_air_hi"), hash40("landing_air_n")].contains(&MotionModule::motion_kind(fighter.module_accessor))) {
			if GroundModule::is_passable_ground(fighter.module_accessor) && frame/cancel_frame >= (1.0/6.0){
                if sticky <= -0.6875 && ((ControlModule::get_flick_y(fighter.module_accessor) >= 3 && ControlModule::get_flick_y(fighter.module_accessor) < 20) || sticky <= -1.0) {
					if (
						(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) == 0 &&
						(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 &&
						(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) == 0 &&
						(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) == 0 &&
						(ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP))
					) {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
					};
                };
            }
		};
    };
}	

#[fighter_frame_callback]
pub fn dashdrop(fighter : &mut L2CFighterCommon) {
    unsafe {

		let sticky = ControlModule::get_stick_y(fighter.module_accessor);	
        let status = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);


        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_DASH 
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_TURN_DASH 
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_RUN 
        || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_RUN_BRAKE {
			if GroundModule::is_passable_ground(fighter.module_accessor) {
                if sticky <= -0.6875 && ((ControlModule::get_flick_y(fighter.module_accessor) >= 3 && ControlModule::get_flick_y(fighter.module_accessor) < 20) || sticky <= -1.0) {
					if (
						(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) == 0 &&
						(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 &&
						(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) == 0 &&
						(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE) == 0 &&
						(ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP))
					) {
						StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
					};
                };
            }
		};
    }
}

#[fighter_frame_callback]
pub fn shielddrop(fighter : &mut L2CFighterCommon) {
    unsafe {

		let sticky = ControlModule::get_stick_y(fighter.module_accessor);	
        let status = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_GUARD {
			if GroundModule::is_passable_ground(fighter.module_accessor) {
                if sticky <= -0.6875 && ((ControlModule::get_flick_y(fighter.module_accessor) >= 3 && ControlModule::get_flick_y(fighter.module_accessor) < 20) || sticky <= -1.0) {
					StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
                };
            }
		};
    }
}


#[fighter_frame_callback]
pub fn aircancel(fighter : &mut L2CFighterCommon) {
    unsafe {
        let situation = StatusModule::situation_kind(fighter.module_accessor);
        let status = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let fighter_kind = smash::app::utility::get_kind(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent));


        if status == *FIGHTER_STATUS_KIND_ATTACK_AIR{
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") { 
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
                && [*FIGHTER_KIND_WOLF, *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_INKLING].contains(&fighter_kind)
                {

                    if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 || 
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 ||
                    (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
                        if(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 {
                            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, true);
                        }
                    }   

                    if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
                    }
                    if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                    }
                    if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
                    }
                    if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
                    }


                }
            }
        }
    }
}


#[fighter_frame_callback]
pub fn hitfall(fighter : &mut L2CFighterCommon) {
    unsafe {
        let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
        let status = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let fighter_kind = smash::app::utility::get_kind(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent));
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
        && stick_y < -0.66
        && KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) >= 0.0 {
                
            if status == *FIGHTER_STATUS_KIND_ATTACK_AIR && MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") &&
            [*FIGHTER_KIND_PICHU, *FIGHTER_KIND_WOLF].contains(&fighter_kind) {
                if !(WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE)){
                    WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                    macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -0.0, -0.0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                }     
            }

            if status == *FIGHTER_STATUS_KIND_ATTACK_AIR && MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_b") &&
            [*FIGHTER_KIND_SHEIK].contains(&fighter_kind) {
                if !(WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE)){
                    WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                    macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -0.0, -0.0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                }     
            }

            if status == *FIGHTER_STATUS_KIND_ATTACK_AIR && MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") &&
            [*FIGHTER_KIND_PICHU].contains(&fighter_kind) {
                if !(WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE)){
                    WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                    macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -0.0, -0.0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
                }     
            }
        };
    }
}



#[fighter_frame_callback]
pub fn wavedash(fighter : &mut L2CFighterCommon) {
    unsafe {

        // holy fucking shit i am not happy with this code LMAO

        let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        let status = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);
        let fighter_kind = smash::app::utility::get_kind(smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent));
        let situation = StatusModule::situation_kind(fighter.module_accessor);   

        if(ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)){
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);

            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) &&
            status == FIGHTER_STATUS_KIND_GUARD_ON && CAN_WD_BUFFER {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                CAN_WD_BUFFER = false;
            }
        }

        if ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            CAN_WD_BUFFER = true;
        }

         

        if ([*FIGHTER_STATUS_KIND_LANDING].contains(&status) == true) {
            if TEST > 0.0 {
                WAVEDASH = true;
                println!("WAVEDASH STARTED");
            }
        }
        else{
            WAVEDASH = false;
            println!("WAVEDASH FINISHED");
        }

        if TEST > 0.0 {
            TEST-=1.0;
        }

        if status == *FIGHTER_STATUS_KIND_JUMP_SQUAT {

            if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE) != 0 || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD){
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
                TEST = 20.0;
                StatusModule::set_situation_kind(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);

            }
            
            CAN_WD_BUFFER = false;

        };
    }
}




#[fighter_frame_callback]
pub fn wavedash_actions(fighter : &mut L2CFighterCommon) {
    unsafe {

 // this shit was too broken i cba to balance it so im not adding it in for now even though it was quite fun no cap


        let status = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);

        if (WAVEDASH == true && status == *FIGHTER_STATUS_KIND_LANDING) {

            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);


            if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK, true);
            }
            
            if(ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
            }
            
            if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            }
            
            if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
            }
            

            if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);

            }
            if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);;
            }

            if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }

            if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_S, true);
            }
        }
    }
}



#[fighter_frame_callback]
pub fn dash_edgec(fighter : &mut L2CFighterCommon) {
    unsafe {

        let status = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);

        if status == *FIGHTER_STATUS_KIND_ATTACK_DASH {  
            if MotionModule::frame(fighter.module_accessor) > 8.0 
            && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
        }
    }
}


#[fighter_frame_callback]
pub fn dacus(fighter : &mut L2CFighterCommon) {
    unsafe {

        let status = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);

        if status == *FIGHTER_STATUS_KIND_ATTACK_DASH
        && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {  
            if MotionModule::frame(fighter.module_accessor) < 12.0 {
                if ((ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 ||
                (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0) ||
                
                (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) 
                && ControlModule::get_stick_y(fighter.module_accessor) > 0.8) { 
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
                }
            }
        }
    }
}

//DJC
#[fighter_frame_callback]
pub fn djc(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let ENTRY_ID = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(boma);
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		if [*FIGHTER_KIND_LUCAS].contains(&fighter_kind) {
			if [*FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL].contains(&KineticModule::get_kinetic_type(boma)) {
				if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) && [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_AIR_LASSO, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_N].contains(&status_kind) {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
				};
				if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL {
					KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION);
				};
			};
		};
		
				
    };
}


#[fighter_frame_callback]
pub fn buffer_order(fighter : &mut L2CFighterCommon) { // REDUNTANT CODE
    unsafe {
        let status = smash::app::lua_bind::StatusModule::status_kind(fighter.module_accessor);

        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) &&
        status == FIGHTER_STATUS_KIND_GUARD_ON{
            
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_JUMP);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
            
        }
    }
}



pub fn install() {
    smashline::install_agent_frame_callbacks!(
		llpc,
        aircancel,
        hitfall,
        dashdrop,
        shielddrop,
        wavedash,
        dash_edgec,
        dacus,
        djc,


        wavedash_actions
	);
 
    smashline::install_agent_frames!(

    );
}
