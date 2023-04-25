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



pub fn install() {
    smashline::install_acmd_scripts!(

    );

 
    smashline::install_agent_frames!(

    );

}
