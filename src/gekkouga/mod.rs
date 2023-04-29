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

#[fighter_frame( agent = FIGHTER_KIND_GEKKOUGA )]
fn gren_frame(fighter: &mut L2CFighterCommon) {
    unsafe{
        
        let doll_id = WorkModule::get_int(fighter.module_accessor, 0x100000C2);
        let doll_boma = sv_battle_object::module_accessor(doll_id as u32);
        let DOLL_POS = Vector3f {x: PostureModule::pos_x(doll_boma), y: PostureModule::pos_y(doll_boma), z: PostureModule::pos_z(doll_boma)};     
        
        let status = StatusModule::status_kind(fighter.module_accessor);

        if status == *FIGHTER_STATUS_KIND_SPECIAL_N {
            PostureModule::set_pos(fighter.module_accessor, &DOLL_POS);
        }
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(

    );

 
    smashline::install_agent_frames!(
        gren_frame
    );

}
