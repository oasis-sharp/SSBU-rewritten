cargo skyline package
pause
move ssbu_rewritten\plugin.nro other_files
rename other_files\plugin.nro plugin_old.nro
rename target\aarch64-skyline-switch\release\libplugin.nro plugin.nro
move target\aarch64-skyline-switch\release\plugin.nro ssbu_rewritten
