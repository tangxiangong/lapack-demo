add_rules("mode.debug", "mode.release")

target("demo")
    set_kind("shared")
    add_includedirs("./include")
    add_files("c-src/*.c")
    set_targetdir("lib")

set_languages("c23")
