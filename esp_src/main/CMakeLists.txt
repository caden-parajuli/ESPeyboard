file(GLOB_RECURSE srcs "src/*.c")

idf_component_register(
    SRCS SRCS "${srcs}"
    INCLUDE_DIRS "./include"
    PRIV_REQUIRES bt nvs_flash esp_driver_gpio
    )
